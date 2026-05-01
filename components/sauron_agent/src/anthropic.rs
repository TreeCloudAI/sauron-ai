// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

//! Anthropic Claude provider for Sauron's agent loop.
//!
//! Implements a minimal client of the Messages API
//! (<https://docs.anthropic.com/en/api/messages>) with prompt caching
//! enabled on the system prompt and the tool list. Caching is critical
//! for an agent loop — the system prompt and tool definitions stay
//! constant across turns, so caching them turns each follow-up call
//! from "send the entire ~10k token preamble again" into "reuse the
//! cached prefix" with a 90% cost reduction on cached tokens.
//!
//! The provider returns the Anthropic response translated into the
//! agent's [`Turn`](crate::messages::Turn) representation. Agent loops
//! drive this provider step-by-step, dispatching tool calls externally
//! and feeding tool results back as the next call's `messages`.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{AgentError, Result};
use crate::tools::{Tool, ToolCall, ToolResult};

/// Default model used by the Sauron agent. Sonnet 4.6 strikes a good
/// balance of latency and capability for browser-automation loops where
/// many short turns matter more than peak reasoning depth.
pub const DEFAULT_MODEL: &str = "claude-sonnet-4-6";

/// Anthropic API endpoint for the Messages API.
const MESSAGES_URL: &str = "https://api.anthropic.com/v1/messages";

/// Pinned API version. Anthropic's API is versioned via this header so
/// new fields and behaviors are opt-in.
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// A configured Claude client. Cheap to clone — the inner `reqwest::Client`
/// is reference-counted and pools connections.
#[derive(Clone)]
pub struct AnthropicProvider {
    http: reqwest::Client,
    api_key: String,
    model: String,
    max_tokens: u32,
}

impl AnthropicProvider {
    /// Build a provider from an API key. Uses sensible defaults for
    /// model and `max_tokens`; override with the builder methods below.
    pub fn new(api_key: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .pool_idle_timeout(Duration::from_secs(60))
            .build()
            .expect("reqwest client should build with rustls features");
        Self {
            http,
            api_key: api_key.into(),
            model: DEFAULT_MODEL.to_owned(),
            max_tokens: 4096,
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Drive one Messages API call.
    ///
    /// `system` is the agent's system prompt; `tools` are the JSON-Schema
    /// tool definitions. Both are marked with `cache_control: ephemeral`
    /// so subsequent calls within the 5-minute cache window reuse the
    /// cached prefix.
    ///
    /// `history` is the conversation built up so far — alternating user
    /// and assistant turns. The most recent turn must be a user turn
    /// (either the original goal or the prior turn's tool results).
    pub async fn step(
        &self,
        system: &str,
        tools: &[Tool],
        history: &[ProviderMessage],
    ) -> Result<ProviderResponse> {
        let request = AnthropicRequest {
            model: &self.model,
            max_tokens: self.max_tokens,
            system: vec![SystemBlock {
                kind: "text",
                text: system,
                cache_control: Some(CacheControl::ephemeral()),
            }],
            tools: tools_with_cache_breakpoint(tools),
            messages: history,
        };

        let response = self
            .http
            .post(MESSAGES_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AgentError::Provider(format!("request failed: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AgentError::Provider(format!(
                "{status}: {body}",
                status = status.as_u16()
            )));
        }

        let body: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| AgentError::Provider(format!("decode failed: {e}")))?;

        Ok(ProviderResponse::from_anthropic(body))
    }
}

/// One message in the conversation. Anthropic uses string roles
/// `"user"` and `"assistant"` with structured content blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMessage {
    pub role: Role,
    pub content: Vec<ContentBlock>,
}

impl ProviderMessage {
    pub fn user_text(text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: vec![ContentBlock::Text { text: text.into() }],
        }
    }

    /// Build a user turn from a list of tool results, the standard way
    /// to feed tool outputs back to the model.
    pub fn user_tool_results(results: &[ToolResult]) -> Self {
        let content = results
            .iter()
            .map(|r| ContentBlock::ToolResult {
                tool_use_id: r.call_id.clone(),
                content: serde_json::to_string(&r.output)
                    .unwrap_or_else(|_| String::from("\"<unserializable>\"")),
                is_error: !r.ok,
            })
            .collect();
        Self {
            role: Role::User,
            content,
        }
    }

    pub fn assistant_blocks(content: Vec<ContentBlock>) -> Self {
        Self {
            role: Role::Assistant,
            content,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// One content block inside a message — text, tool use, or tool result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
        #[serde(default, skip_serializing_if = "is_false")]
        is_error: bool,
    },
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// What [`AnthropicProvider::step`] returns: the full assistant turn
/// the model produced, broken out into text, tool calls, and the stop
/// reason that ended the turn.
pub struct ProviderResponse {
    /// All assistant content blocks, in order. Useful for re-feeding the
    /// model's exact response back as conversation history.
    pub assistant_content: Vec<ContentBlock>,
    /// Concatenated text-only content (convenience).
    pub text: String,
    /// Any tool calls the model emitted this turn.
    pub tool_calls: Vec<ToolCall>,
    pub stop_reason: StopReason,
}

impl ProviderResponse {
    fn from_anthropic(resp: AnthropicResponse) -> Self {
        let mut text = String::new();
        let mut tool_calls = Vec::new();
        for block in resp.content.iter() {
            match block {
                ContentBlock::Text { text: t } => {
                    if !text.is_empty() {
                        text.push('\n');
                    }
                    text.push_str(t);
                },
                ContentBlock::ToolUse { id, name, input } => {
                    tool_calls.push(ToolCall {
                        id: id.clone(),
                        name: name.clone(),
                        input: input.clone(),
                    });
                },
                ContentBlock::ToolResult { .. } => {
                    // Models do not emit tool_result blocks in their
                    // own turns; ignore if seen.
                },
            }
        }
        Self {
            assistant_content: resp.content,
            text,
            tool_calls,
            stop_reason: resp.stop_reason.unwrap_or(StopReason::Other),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    StopSequence,
    /// Anything we did not anticipate; surfaced verbatim by the API.
    #[serde(other)]
    Other,
}

// ---- Wire types: only used at the serialization boundary ----

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    system: Vec<SystemBlock<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<ToolBlock<'a>>,
    messages: &'a [ProviderMessage],
}

#[derive(Serialize)]
struct SystemBlock<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<CacheControl>,
}

#[derive(Serialize)]
struct ToolBlock<'a> {
    name: &'a str,
    description: &'a str,
    input_schema: &'a Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<CacheControl>,
}

#[derive(Serialize)]
struct CacheControl {
    #[serde(rename = "type")]
    kind: &'static str,
}

impl CacheControl {
    fn ephemeral() -> Self {
        Self { kind: "ephemeral" }
    }
}

/// Mark only the *last* tool definition with `cache_control: ephemeral`.
/// Anthropic caches the prefix up to the breakpoint, so one breakpoint
/// at the end of `tools` covers every tool definition with a single
/// cache entry.
fn tools_with_cache_breakpoint(tools: &[Tool]) -> Vec<ToolBlock<'_>> {
    let last_idx = tools.len().checked_sub(1);
    tools
        .iter()
        .enumerate()
        .map(|(i, t)| ToolBlock {
            name: &t.name,
            description: &t.description,
            input_schema: &t.input_schema,
            cache_control: if Some(i) == last_idx {
                Some(CacheControl::ephemeral())
            } else {
                None
            },
        })
        .collect()
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    stop_reason: Option<StopReason>,
}
