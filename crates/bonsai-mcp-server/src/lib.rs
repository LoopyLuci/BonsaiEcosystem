pub mod server;
pub mod tools;
pub mod auth;
pub mod bridge;
pub mod uacs;
pub mod mobile_session;
pub mod bti_commands;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
	pub name: String,
	pub description: String,
	pub input_schema: serde_json::Value,
}
