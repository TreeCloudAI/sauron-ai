// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

use serde::{Deserialize, Serialize};

use crate::tools::{ToolCall, ToolResult};

/// A user-supplied objective for the agent — what it is being asked to do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub instruction: String,
    #[serde(default)]
    pub starting_url: Option<String>,
    #[serde(default = "default_max_turns")]
    pub max_turns: u32,
}

fn default_max_turns() -> u32 {
    32
}

/// A single observation produced by the browser between agent turns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub url: String,
    /// Concise textual summary of the page (a11y-tree-derived).
    pub summary: String,
    /// Optional screenshot, base64-PNG.
    #[serde(default)]
    pub screenshot_b64: Option<String>,
    pub timestamp_ms: u64,
}

/// One round-trip of the agent loop: model produced these tool calls, the
/// browser ran them, and these are the results that will feed the next turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub tool_calls: Vec<ToolCall>,
    pub tool_results: Vec<ToolResult>,
    pub observation: Option<Observation>,
}
