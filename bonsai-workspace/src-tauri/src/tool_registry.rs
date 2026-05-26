//! Pluggable tool registry — tools register themselves at startup and can be
//! invoked by the assistant pipeline or directly via Tauri commands.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use tracing::warn;

// ── Tool trait ────────────────────────────────────────────────────────────────

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn run(&self, args: &Value) -> Result<ToolResult, String>;

    /// Optional streaming run with progress updates sent on `progress_tx`.
    /// By default, this calls `run()` and sends no progress updates.
    async fn run_with_progress(
        &self,
        args: &Value,
        _progress_tx: tokio::sync::mpsc::UnboundedSender<serde_json::Value>,
    ) -> Result<ToolResult, String> {
        self.run(args).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// MIME type of the result (e.g. "text/plain", "image/png", "audio/wav", "application/json").
    pub content_type: String,
    /// Raw bytes of the result.
    pub data: Vec<u8>,
}

impl ToolResult {
    pub fn text(s: impl Into<String>) -> Self {
        Self { content_type: "text/plain".into(), data: s.into().into_bytes() }
    }
    pub fn json(v: &Value) -> Self {
        Self {
            content_type: "application/json".into(),
            data: serde_json::to_vec(v).unwrap_or_default(),
        }
    }
    pub fn as_text(&self) -> Option<&str> {
        if self.content_type.starts_with("text/") {
            std::str::from_utf8(&self.data).ok()
        } else {
            None
        }
    }
}

// ── Registry ──────────────────────────────────────────────────────────────────

pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Box<dyn Tool>>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: RwLock::new(HashMap::new()) }
    }

    pub async fn register(&self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        self.tools.write().await.insert(name, tool);
    }

    pub async fn execute(&self, name: &str, args: &Value) -> Option<ToolResult> {
        let tools = self.tools.read().await;
        match tools.get(name)?.run(args).await {
            Ok(r) => Some(r),
            Err(e) => {
                warn!(tool=name, error=%e, "[tool_registry] execution failed");
                None
            }
        }
    }

    /// Execute a tool and forward progress via `progress_tx`.
    pub async fn execute_with_progress(
        &self,
        name: &str,
        args: &Value,
        progress_tx: tokio::sync::mpsc::UnboundedSender<serde_json::Value>,
    ) -> Option<ToolResult> {
        let tools = self.tools.read().await;
        let tool = tools.get(name)?;
        match tool.run_with_progress(args, progress_tx).await {
            Ok(r) => Some(r),
            Err(e) => {
                warn!(tool=name, error=%e, "[tool_registry] execution failed");
                None
            }
        }
    }

    pub async fn list(&self) -> Vec<ToolInfo> {
        self.tools
            .read()
            .await
            .values()
            .map(|t| ToolInfo { name: t.name().to_string(), description: t.description().to_string() })
            .collect()
    }
}

#[derive(Debug, Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
}

// ── Built-in: ExecuteCodeTool ─────────────────────────────────────────────────

pub struct ExecuteCodeTool;

#[async_trait]
impl Tool for ExecuteCodeTool {
    fn name(&self) -> &str { "execute_code" }
    fn description(&self) -> &str { "Execute Python code in a sandboxed venv. Args: {code: string, timeout_secs?: number}" }

    async fn run(&self, args: &Value) -> Result<ToolResult, String> {
        let code = args["code"].as_str().ok_or("Missing 'code' argument")?;
        let timeout_secs = args["timeout_secs"].as_u64();
        let result = crate::sandbox_executor::execute_plugin_code(code).await?;
        let output = serde_json::json!({
            "stdout": result.stdout,
            "stderr": result.stderr,
            "exit_code": result.exit_code,
            "timed_out": result.timed_out,
        });
        Ok(ToolResult::json(&output))
    }
}

// ── Built-in: SystemInfoTool ──────────────────────────────────────────────────

pub struct SystemInfoTool;

#[async_trait]
impl Tool for SystemInfoTool {
    fn name(&self) -> &str { "system_info" }
    fn description(&self) -> &str { "Return CPU, RAM, and OS info." }

    async fn run(&self, _args: &Value) -> Result<ToolResult, String> {
        use sysinfo::System;
        let mut sys = System::new_all();
        sys.refresh_all();
        let info = serde_json::json!({
            "os": System::name(),
            "kernel": System::kernel_version(),
            "cpu_count": sys.cpus().len(),
            "total_ram_mb": sys.total_memory() / 1024 / 1024,
            "used_ram_mb": sys.used_memory() / 1024 / 1024,
        });
        Ok(ToolResult::json(&info))
    }
}

// ── Shared state wrapper ──────────────────────────────────────────────────────

#[derive(Clone)]
pub struct ToolRegistryState {
    pub registry: Arc<ToolRegistry>,
}

impl ToolRegistryState {
    pub async fn new_with_defaults() -> Arc<Self> {
        let registry = Arc::new(ToolRegistry::new());
        registry.register(Box::new(ExecuteCodeTool)).await;
        registry.register(Box::new(SystemInfoTool)).await;
        // Demo streaming tool for testing progress updates
        registry.register(Box::new(crate::tools::demo_streaming::DemoStreamingTool::new())).await;
        let state = Arc::new(Self { registry });
        // Register Phase-1 multi-modal tools (Kokoro TTS, Depth, YOLO).
        crate::multimodal::register_all(&state).await;
        state
    }

    /// Return a simple list of tools for external listing consumers.
    pub fn list_tools(&self) -> Vec<ToolInfo> {
        // Note: This clones the current snapshot; callers should be quick.
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async { self.registry.list().await })
    }

    /// Invoke a tool by name and return a JSON-ish Value result or an error.
    pub async fn invoke_by_name(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value, String> {
        match self.registry.execute(name, &args).await {
            Some(res) => {
                if res.content_type.starts_with("text/") {
                    if let Ok(s) = std::str::from_utf8(&res.data) {
                        Ok(serde_json::json!({ "content": s }))
                    } else {
                        Err("invalid utf8 in text result".into())
                    }
                } else if res.content_type == "application/json" {
                    serde_json::from_slice(&res.data).map_err(|e| e.to_string())
                } else {
                    Err(format!("unsupported content_type: {}", res.content_type))
                }
            }
            None => Err(format!("tool '{}' not found or failed", name)),
        }
    }
}

// ── Tauri commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_tools(
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<ToolInfo>, String> {
    Ok(state.tool_registry.registry.list().await)
}

#[tauri::command]
pub async fn discover_peers_cmd(
    _state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<(String,u16,Vec<String>)>, String> {
    match crate::p2p::sharing::discover_peers().await {
        Ok(p) => Ok(p),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn request_model_cmd(
    _state: tauri::State<'_, crate::AppState>,
    url: String,
    local_path: String,
) -> Result<(), String> {
    crate::p2p::sharing::request_model(&url, &local_path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn run_tool(
    state: tauri::State<'_, crate::AppState>,
    name: String,
    args: Value,
) -> Result<ToolResult, String> {
    state
        .tool_registry
        .registry
        .execute(&name, &args)
        .await
        .ok_or_else(|| format!("Tool '{name}' not found or failed"))
}
