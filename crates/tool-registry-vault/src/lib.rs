pub mod registry;
pub mod executor;
pub mod manifest;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
	pub name: String,
	pub description: String,
	pub input_schema: serde_json::Value,
	pub output_schema: Option<serde_json::Value>,
	pub required_capabilities: Vec<String>,
	pub execution_mode: ToolExecutionMode,
	pub examples: Vec<ToolExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolExecutionMode {
	Sandbox,
	Direct,
	AskPermission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExample {
	pub description: String,
	pub input: serde_json::Value,
	pub output: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
	pub tool_name: String,
	pub arguments: serde_json::Value,
	pub caller_capability: String,
	pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResponse {
	pub success: bool,
	pub result: serde_json::Value,
	pub error: Option<String>,
	pub execution_time_ms: u64,
}
pub fn vault_ready() -> bool { true }

#[cfg(test)]
mod tests { use super::*; #[test] fn smoke() { assert!(vault_ready()); } }
