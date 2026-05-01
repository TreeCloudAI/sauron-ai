// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::anthropic::{AnthropicProvider, ProviderMessage, StopReason};
use crate::error::{AgentError, Result};
use crate::messages::{Goal, Turn};
use crate::tools::{Tool, ToolCall, ToolResult};

/// Async closure that executes a single tool call. The agent holds one
/// of these and invokes it whenever the model emits a `tool_use` block.
pub type ToolDispatcher = Arc<
    dyn Fn(ToolCall) -> Pin<Box<dyn Future<Output = ToolResult> + Send>> + Send + Sync,
>;

/// Sauron agent runtime. Drives a Claude Messages loop, dispatching
/// any tool calls the model emits to the provided dispatcher and
/// feeding the results back as the next turn's input.
pub struct Agent {
    provider: AnthropicProvider,
    tools: Vec<Tool>,
    system: String,
    dispatcher: ToolDispatcher,
}

impl Agent {
    pub fn new(
        provider: AnthropicProvider,
        tools: Vec<Tool>,
        system: impl Into<String>,
        dispatcher: ToolDispatcher,
    ) -> Self {
        Self {
            provider,
            tools,
            system: system.into(),
            dispatcher,
        }
    }

    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }

    pub fn system(&self) -> &str {
        &self.system
    }

    /// Run the agent loop against `goal`, returning the sequence of
    /// turns it executed. Stops when the model emits a turn without
    /// any tool calls (a final answer) or when `goal.max_turns` is
    /// exhausted.
    pub async fn run(&self, goal: Goal) -> Result<Vec<Turn>> {
        if goal.max_turns == 0 {
            return Err(AgentError::Invalid(
                "goal.max_turns must be at least 1".to_owned(),
            ));
        }

        let mut history: Vec<ProviderMessage> = Vec::new();
        history.push(initial_user_message(&goal));

        let mut turns: Vec<Turn> = Vec::new();
        for _ in 0..goal.max_turns {
            let response = self
                .provider
                .step(&self.system, &self.tools, &history)
                .await?;

            history.push(ProviderMessage::assistant_blocks(
                response.assistant_content.clone(),
            ));

            // No tool calls → model produced a final answer.
            if response.tool_calls.is_empty() || response.stop_reason != StopReason::ToolUse {
                turns.push(Turn {
                    tool_calls: response.tool_calls,
                    tool_results: Vec::new(),
                    observation: None,
                });
                return Ok(turns);
            }

            // Dispatch every tool call before the next turn. Sequential
            // execution keeps causality predictable for the model (each
            // tool sees the side effects of the previous one);
            // parallelizing safe tools is a future optimization.
            let mut results = Vec::with_capacity(response.tool_calls.len());
            for call in response.tool_calls.iter() {
                let result = (self.dispatcher)(call.clone()).await;
                results.push(result);
            }

            history.push(ProviderMessage::user_tool_results(&results));
            turns.push(Turn {
                tool_calls: response.tool_calls,
                tool_results: results,
                observation: None,
            });
        }

        Err(AgentError::BudgetExhausted)
    }
}

fn initial_user_message(goal: &Goal) -> ProviderMessage {
    let mut text = String::new();
    if let Some(url) = &goal.starting_url {
        text.push_str(&format!("Starting URL: {url}\n\n"));
    }
    text.push_str(&goal.instruction);
    ProviderMessage::user_text(text)
}
