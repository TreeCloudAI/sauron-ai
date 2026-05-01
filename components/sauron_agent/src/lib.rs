// Copyright 2026 TreeCloud AI
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied. See the License for the specific language governing
// permissions and limitations under the License.

//! Sauron Agent — the agentic layer of Sauron AI.
//!
//! This crate hosts the long-running agent runtime that drives the browser:
//! it consumes a user goal, calls an LLM (currently Anthropic Claude) with
//! tool definitions, and dispatches the chosen tool calls to the browser
//! engine via [`tools`]. Browser observations (DOM snapshots, screenshots,
//! navigation events) flow back as [`messages::Observation`]s and feed the
//! next turn of the conversation.
//!
//! The crate is intentionally minimal at this point — only the public types
//! and module boundary are defined so that the rest of the Sauron stack can
//! depend on a stable surface while implementations land.

pub mod agent;
pub mod error;
pub mod messages;
pub mod tools;

pub use agent::Agent;
pub use error::{AgentError, Result};
pub use messages::{Goal, Observation, Turn};
pub use tools::{Tool, ToolCall, ToolResult};
