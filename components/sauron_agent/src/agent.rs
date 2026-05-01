// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

use crate::error::Result;
use crate::messages::{Goal, Turn};
use crate::tools::Tool;

/// The Sauron agent runtime.
///
/// At this stage the type is a placeholder that pins the public surface: a
/// constructor, a `tools()` accessor, and a `run` entry point that takes a
/// [`Goal`] and yields the sequence of [`Turn`]s the agent went through.
/// The actual model invocation, tool dispatch, and observation collection
/// will land in subsequent commits.
pub struct Agent {
    tools: Vec<Tool>,
}

impl Agent {
    pub fn new(tools: Vec<Tool>) -> Self {
        Self { tools }
    }

    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }

    /// Drive the agent against a goal. Currently unimplemented — the loop
    /// will be wired up once a concrete LLM provider trait and a
    /// browser-side tool dispatcher are in place.
    pub async fn run(&self, _goal: Goal) -> Result<Vec<Turn>> {
        unimplemented!(
            "Agent::run is not implemented yet — see the Sauron agent roadmap"
        )
    }
}
