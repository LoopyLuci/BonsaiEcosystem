use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod server_config;
pub mod clients;
pub mod external_servers;
pub mod tools;

#[derive(Clone)]
pub struct AppState {
    pub server_config: Arc<RwLock<McpServerConfig>>,
    pub connected_clients: Arc<RwLock<Vec<McpClient>>>,
    pub external_servers: Arc<RwLock<Vec<ExternalMcpServer>>>,
    pub tool_registry: Arc<RwLock<Vec<ToolEntry>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub host: String,
    pub port: u16,
    pub auth_mode: String,
    pub max_clients: u32,
    pub rate_limit_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClient {
    pub client_id: String,
    pub ip_address: String,
    pub connected_since: String,
    pub tools_accessed: Vec<String>,
    pub status: String,
    pub request_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalMcpServer {
    pub name: String,
    pub url: String,
    pub status: String,
    pub last_checked: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntry {
    pub name: String,
    pub description: String,
    pub category: String,
    pub enabled: bool,
    pub schema: serde_json::Value,
}
