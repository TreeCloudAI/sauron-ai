// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stable identifier for a single tool exposed to the LLM.
pub type ToolName = String;

/// Schema-level description of a tool the agent may invoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: ToolName,
    pub description: String,
    /// JSON Schema describing the input the tool accepts.
    pub input_schema: Value,
}

/// A tool call emitted by the LLM during a turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: ToolName,
    pub input: Value,
}

/// The result of executing a [`ToolCall`] against the browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub ok: bool,
    pub output: Value,
}
