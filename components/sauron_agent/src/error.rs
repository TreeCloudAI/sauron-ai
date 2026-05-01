// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

use thiserror::Error;

pub type Result<T, E = AgentError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("LLM provider error: {0}")]
    Provider(String),

    #[error("tool `{name}` failed: {reason}")]
    Tool { name: String, reason: String },

    #[error("the agent exhausted its turn budget without reaching a goal")]
    BudgetExhausted,

    #[error("invalid agent input: {0}")]
    Invalid(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
