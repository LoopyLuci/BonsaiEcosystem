# Copilot Tool Translation Engine - Implementation Quickstart

**For:** Implementation Team  
**Status:** Ready for Phase 1 Development  
**Date:** 2026-06-01

---

## Overview

This document provides concrete, copy-paste-ready pseudocode to jumpstart implementation of the Copilot Tool Translation Engine.

Each code example follows the architecture from `docs/25-COPILOT-TOOL-TRANSLATION-ENGINE.md`.

---

## 1. Core Translation Layer (Rust)

### 1.1 Translation Request Handler

```rust
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde_json::{json, Value};
use std::sync::Arc;

/// Main entry point for Copilot tool invocations
#[post("/tool/{tool_name}")]
pub async fn translate_tool_call(
    tool_name: web::Path<String>,
    req: HttpRequest,
    body: web::Json<Value>,
    registry: web::Data<Arc<ToolSchemaRegistry>>,
    approval_gate: web::Data<Arc<ApprovalGate>>,
    universe: web::Data<Arc<Universe>>,
) -> HttpResponse {
    let tool_name = tool_name.into_inner();
    let copilot_args = body.into_inner();

    // 1. Validate request
    let token = match extract_bearer_token(&req) {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::Unauthorized()
                .json(error_response("invalid_token", &e.to_string()))
        }
    };

    // 2. Look up tool in registry
    let tool_schema = match registry.get_tool(&tool_name) {
        Some(t) => t,
        None => {
            return HttpResponse::NotFound()
                .json(error_response("tool_not_found", &format!("Tool '{}' not found", tool_name)))
        }
    };

    // 3. Check permissions
    let user_id = decode_token(&token).unwrap_or_default();
    let user_perms = get_user_permissions(&user_id);
    for required_perm in &tool_schema.permissions_required {
        if !user_perms.contains(required_perm) {
            return HttpResponse::Forbidden()
                .json(error_response(
                    "permission_denied",
                    &format!("Missing permission: {}", required_perm),
                ))
        }
    }

    // 4. Validate input parameters
    if let Err(e) = validate_copilot_parameters(&tool_name, &copilot_args, &tool_schema) {
        return HttpResponse::BadRequest()
            .json(error_response("validation_error", &e.to_string()))
    }

    // 5. Compute confidence score
    let confidence = compute_confidence_score(
        &tool_name,
        &copilot_args,
        &user_id,
        &ToolInvocationContext::default(),
    );

    // 6. Check if approval is needed
    let needs_approval = should_require_approval(confidence, &tool_schema.risk_level);

    if needs_approval {
        match approval_gate
            .request_approval(
                &user_id,
                &tool_name,
                &copilot_args,
                confidence,
            )
            .await
        {
            Ok(true) => {
                // User approved
            }
            Ok(false) => {
                return HttpResponse::Forbidden()
                    .json(error_response("approval_denied", "User denied approval"))
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(error_response("approval_error", &e.to_string()))
            }
        }
    }

    // 7. Translate parameters to Bonsai format
    let bonsai_args = match translate_parameters(&tool_name, &copilot_args, &registry) {
        Ok(args) => args,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(error_response("translation_error", &e.to_string()))
        }
    };

    // 8. Execute tool (direct, bridge, or cloud)
    let result = match tool_schema.bonsai_mapping.strategy.as_str() {
        "direct" => {
            execute_bonsai_tool(&tool_schema.bonsai_mapping.tool_name, &bonsai_args, &token)
                .await
        }
        "bridge" => {
            execute_bridge_tool(&tool_name, &bonsai_args, &registry, &universe).await
        }
        "cloud_fallback" => {
            execute_cloud_fallback(&tool_name, &bonsai_args, &token).await
        }
        _ => Err("unknown strategy".into()),
    };

    // 9. Translate result back to Copilot format
    let copilot_result = match result {
        Ok(bonsai_result) => {
            match translate_result(&tool_name, &bonsai_result, &registry) {
                Ok(result) => result,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(error_response("result_translation_error", &e.to_string()))
                }
            }
        }
        Err(e) => {
            return map_tool_error_to_http(&tool_name, &e, &registry)
        }
    };

    // 10. Log to Universe
    emit_universe_event(
        &universe,
        ToolInvocationEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            user_id: user_id.clone(),
            tool_name: tool_name.clone(),
            parameters_hash: hash_json(&copilot_args),
            execution_strategy: tool_schema.bonsai_mapping.strategy.clone(),
            execution_duration_ms: 0, // Will be updated
            confidence_score: confidence,
            approval_required: needs_approval,
            approval_given: needs_approval,
            permissions_held: user_perms.clone(),
            status: if copilot_result.get("error").is_none() {
                "success".into()
            } else {
                "failure".into()
            },
            error_code: copilot_result.get("error_code").map(|v| v.to_string()),
            error_message: copilot_result.get("error_message").map(|v| v.to_string()),
            position_in_sequence: None,
            total_sequence_length: None,
            previous_tool_name: None,
            next_tool_name: None,
        },
    );

    HttpResponse::Ok().json(copilot_result)
}

fn error_response(error_code: &str, message: &str) -> Value {
    json!({
        "error": error_code,
        "message": message,
        "timestamp": chrono::Utc::now().timestamp_millis()
    })
}
```

---

### 1.2 Parameter Translation

```rust
/// Translate Copilot parameters to Bonsai parameters
fn translate_parameters(
    tool_name: &str,
    copilot_args: &Value,
    registry: &ToolSchemaRegistry,
) -> Result<Value, Box<dyn std::error::Error>> {
    let tool_schema = registry.get_tool(tool_name)
        .ok_or("Tool not found")?;

    let mut bonsai_args = json!({});

    for rule in &tool_schema.bonsai_mapping.parameter_translation {
        let copilot_field = &rule.copilot_field;
        let bonsai_field = &rule.bonsai_field;
        let transform = &rule.transform;

        // Skip if parameter not provided
        if !copilot_args.get(copilot_field).is_some() {
            continue;
        }

        let value = copilot_args.get(copilot_field).unwrap();

        // Apply transformation
        let transformed = match transform.as_str() {
            "as_is" => value.clone(),
            "resolve_to_absolute" => {
                let path = value.as_str().unwrap_or("");
                let abs_path = resolve_path_to_absolute(path)?;
                json!(abs_path)
            }
            "drop" => {
                // Skip this parameter
                continue;
            }
            "handle_in_bridge" => {
                // This transformation is handled by the bridge
                value.clone()
            }
            _ => return Err(format!("Unknown transformation: {}", transform).into()),
        };

        // Only set if bonsai_field is Some
        if let Some(field) = bonsai_field {
            bonsai_args[field] = transformed;
        }
    }

    Ok(bonsai_args)
}

/// Helper to resolve paths to absolute
fn resolve_path_to_absolute(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::path::PathBuf;

    let p = PathBuf::from(path);

    if p.is_absolute() {
        // Validate it's within workspace
        let workspace_root = std::env::var("WORKSPACE_ROOT")
            .unwrap_or_else(|_| ".".into());
        let workspace = PathBuf::from(&workspace_root);

        if !p.starts_with(&workspace) {
            return Err("Path escapes workspace".into());
        }

        Ok(p.to_string_lossy().into_owned())
    } else {
        // Relative: resolve relative to workspace root
        let workspace_root = std::env::var("WORKSPACE_ROOT")
            .unwrap_or_else(|_| ".".into());
        let abs = PathBuf::from(&workspace_root).join(&p);
        Ok(abs.to_string_lossy().into_owned())
    }
}
```

---

### 1.3 Result Translation

```rust
/// Translate Bonsai result back to Copilot format
fn translate_result(
    tool_name: &str,
    bonsai_result: &Value,
    registry: &ToolSchemaRegistry,
) -> Result<Value, Box<dyn std::error::Error>> {
    let tool_schema = registry.get_tool(tool_name)
        .ok_from("Tool not found")?;

    let mut copilot_result = json!({});

    for rule in &tool_schema.bonsai_mapping.result_translation {
        let bonsai_field = &rule.bonsai_field;
        let copilot_field = &rule.copilot_field;
        let constant = &rule.constant;

        if let Some(const_val) = constant {
            // Add constant field
            copilot_result[copilot_field] = json!(const_val);
        } else if let Some(field) = bonsai_field {
            // Copy from Bonsai result
            if let Some(val) = bonsai_result.get(field) {
                copilot_result[copilot_field] = val.clone();
            }
        }
    }

    Ok(copilot_result)
}
```

---

## 2. Confidence Scoring (Rust)

```rust
use std::collections::HashMap;

/// Compute confidence score for a tool invocation
pub fn compute_confidence_score(
    tool_name: &str,
    args: &Value,
    user_id: &str,
    context: &ToolInvocationContext,
) -> f32 {
    let mut score = 0.0;

    // 1. User approval history (0.0-0.3)
    let history_score = compute_history_score(user_id, tool_name);
    score += history_score;

    // 2. Parameter safety (0.0-0.2)
    let param_score = compute_parameter_safety_score(tool_name, args);
    score += param_score;

    // 3. Tool risk baseline (0.2-1.0)
    let risk_baseline = get_risk_baseline(tool_name);
    score += risk_baseline;

    // 4. Permissions available (0.0-0.2)
    let perm_score = compute_permission_score(user_id, tool_name);
    score += perm_score;

    // 5. Tool chain safety (0.0-0.1)
    let chain_score = if context.is_first_tool {
        0.05
    } else if context.previous_tool_succeeded {
        0.05
    } else {
        0.0
    };
    score += chain_score;

    // Cap at 1.0
    score.min(1.0)
}

fn compute_history_score(user_id: &str, tool_name: &str) -> f32 {
    let history = get_user_approval_history(user_id, tool_name);

    if history.total_approvals > 10 {
        0.3  // Well-established trust
    } else if history.approval_rate > 0.9 {
        0.2  // High approval rate
    } else if history.approval_rate > 0.7 {
        0.1  // Moderate approval
    } else {
        0.0
    }
}

fn compute_parameter_safety_score(tool_name: &str, args: &Value) -> f32 {
    let mut score = 0.0;

    // Read-only operations are safer
    if is_read_only_operation(tool_name, args) {
        score += 0.1;
    }

    // Parameters within safe ranges
    if parameters_within_bounds(tool_name, args) {
        score += 0.05;
    }

    // No dangerous patterns detected
    if !dangerous_pattern_detected(tool_name, args) {
        score += 0.05;
    }

    score
}

fn get_risk_baseline(tool_name: &str) -> f32 {
    match get_tool_risk_level(tool_name) {
        "low" => 0.9,
        "medium" => 0.5,
        "high" => 0.2,
        _ => 0.3,
    }
}

fn compute_permission_score(user_id: &str, tool_name: &str) -> f32 {
    let required = get_tool_permissions(tool_name);
    let available = get_user_permissions(user_id);

    if required.iter().all(|p| available.contains(p)) {
        0.2  // All permissions available
    } else if required.iter().any(|p| available.contains(p)) {
        0.1  // Some permissions available
    } else {
        0.0  // No permissions
    }
}

pub fn should_require_approval(score: f32, risk_level: &str) -> bool {
    match risk_level {
        "low" => score < 0.3,
        "medium" => score < 0.6,
        "high" => true,  // Always require
        _ => score < 0.5,
    }
}
```

---

## 3. Bridge Tool Template (Rust)

```rust
use async_trait::async_trait;
use serde_json::Value;

/// Base trait for all bridge tools
#[async_trait]
pub trait ToolBridge: Send + Sync {
    async fn translate_and_execute(
        &self,
        copilot_args: &Value,
        registry: &ToolSchemaRegistry,
        universe: &Universe,
    ) -> Result<Value, Box<dyn std::error::Error>>;
}

/// Bridge implementation for list_files
pub struct ListFilesBridge;

#[async_trait]
impl ToolBridge for ListFilesBridge {
    async fn translate_and_execute(
        &self,
        copilot_args: &Value,
        registry: &ToolSchemaRegistry,
        universe: &Universe,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // 1. Extract parameters
        let path = copilot_args.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");
        let recursive = copilot_args.get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let pattern = copilot_args.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let max_results = copilot_args.get("max_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(1000) as usize;

        // 2. Resolve path
        let abs_path = resolve_path_to_absolute(path)?;

        // 3. Validate path is in workspace
        if !is_within_workspace(&abs_path) {
            return Err("Path escapes workspace".into());
        }

        // 4. Execute glob
        use glob::glob;
        let glob_pattern = if recursive {
            format!("{}/**/{}", abs_path, pattern)
        } else {
            format!("{}/{}", abs_path, pattern)
        };

        let entries = glob(&glob_pattern)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
            .take(max_results)
            .collect::<Result<Vec<_>, _>>()?;

        // 5. Enrich with metadata
        let mut files = Vec::new();
        for entry in entries {
            let metadata = std::fs::metadata(&entry)?;
            files.push(serde_json::json!({
                "path": relative_to_workspace(&entry)?,
                "type": if metadata.is_file() { "file" } else { "directory" },
                "size": metadata.len(),
                "mtime": metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            }));
        }

        Ok(serde_json::json!({
            "files": files,
            "count": files.len()
        }))
    }
}

/// Registry of all bridge tools
pub struct BridgeRegistry {
    bridges: std::collections::HashMap<String, Box<dyn ToolBridge>>,
}

impl BridgeRegistry {
    pub fn new() -> Self {
        let mut bridges: std::collections::HashMap<String, Box<dyn ToolBridge>> =
            std::collections::HashMap::new();

        bridges.insert("list_files".into(), Box::new(ListFilesBridge));
        // Add more bridges...

        Self { bridges }
    }

    pub fn get_bridge(&self, tool_name: &str) -> Option<&Box<dyn ToolBridge>> {
        self.bridges.get(tool_name)
    }
}
```

---

## 4. Approval Gate (Rust)

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct ApprovalGate {
    // Externalize this to a UI dialog or notification system
    approval_handler: Arc<dyn ApprovalHandler>,
}

#[async_trait::async_trait]
pub trait ApprovalHandler: Send + Sync {
    /// Request user approval for a tool call
    /// Returns Ok(true) if approved, Ok(false) if denied
    async fn request_approval(
        &self,
        user_id: &str,
        tool_name: &str,
        args: &Value,
        confidence: f32,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}

impl ApprovalGate {
    pub fn new(handler: Arc<dyn ApprovalHandler>) -> Self {
        Self {
            approval_handler: handler,
        }
    }

    pub async fn request_approval(
        &self,
        user_id: &str,
        tool_name: &str,
        args: &Value,
        confidence: f32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        self.approval_handler
            .request_approval(user_id, tool_name, args, confidence)
            .await
    }
}

/// Example: UI handler that shows a dialog
pub struct TauriApprovalHandler {
    app_handle: tauri::AppHandle,
}

#[async_trait::async_trait]
impl ApprovalHandler for TauriApprovalHandler {
    async fn request_approval(
        &self,
        user_id: &str,
        tool_name: &str,
        args: &Value,
        confidence: f32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Emit event to frontend
        self.app_handle.emit(
            "copilot:approval_required",
            serde_json::json!({
                "user_id": user_id,
                "tool_name": tool_name,
                "args": args,
                "confidence": confidence,
            }),
        ).ok();

        // Wait for response from frontend (with timeout)
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(60),
            wait_for_approval_response(user_id, tool_name),
        )
        .await;

        match result {
            Ok(Ok(approved)) => Ok(approved),
            Ok(Err(e)) => Err(Box::new(e) as Box<dyn std::error::Error>),
            Err(_) => {
                // Timeout
                Ok(false)
            }
        }
    }
}
```

---

## 5. Universe Event Emission (Rust)

```rust
use bonsai_universe::{EventCategory, EventSource, Universe, UniverseEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolInvocationEvent {
    pub id: String,
    pub timestamp: u64,
    pub user_id: String,
    pub tool_name: String,
    pub parameters_hash: String,
    pub execution_strategy: String,
    pub execution_duration_ms: u64,
    pub confidence_score: f32,
    pub approval_required: bool,
    pub approval_given: bool,
    pub permissions_held: Vec<String>,
    pub status: String,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub position_in_sequence: Option<usize>,
    pub total_sequence_length: Option<usize>,
    pub previous_tool_name: Option<String>,
    pub next_tool_name: Option<String>,
}

pub fn emit_universe_event(universe: &Universe, event: ToolInvocationEvent) {
    let category = match event.status.as_str() {
        "success" => EventCategory::AgentAction,
        "failure" => EventCategory::SurvivalEvent,
        "denied" => EventCategory::SecurityEvent,
        _ => EventCategory::AgentAction,
    };

    let summary = format!(
        "Tool invocation: {} — {}",
        event.tool_name, event.status
    );

    let universe_event = UniverseEvent {
        id: event.id.clone(),
        timestamp: event.timestamp,
        category,
        summary,
        source: EventSource::Tool {
            tool: format!("copilot:{}", event.tool_name),
        },
        target: format!("tool:{}", event.tool_name),
        metadata: serde_json::json!({
            "confidence_score": event.confidence_score,
            "approval_required": event.approval_required,
            "execution_strategy": event.execution_strategy,
            "error_code": event.error_code,
        }),
    };

    universe.emitter.emit(universe_event);
}
```

---

## 6. Test Suite Skeleton

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_direct_mapping() {
        let registry = ToolSchemaRegistry::load("tool-schema-registry.yaml");
        let tool = registry.get_tool("read_file").expect("Tool not found");

        assert_eq!(tool.bonsai_mapping.strategy, "direct");
        assert_eq!(tool.risk_level, "low");
    }

    #[test]
    fn test_parameter_translation_path_resolution() {
        let copilot_args = json!({ "path": "src/main.rs" });
        let registry = ToolSchemaRegistry::load("tool-schema-registry.yaml");

        let bonsai_args = translate_parameters("read_file", &copilot_args, &registry)
            .expect("Translation failed");

        // Should be absolute path
        assert!(bonsai_args["path"].as_str().unwrap().starts_with("/"));
    }

    #[test]
    fn test_confidence_score_read_only() {
        let score = compute_confidence_score(
            "read_file",
            &json!({ "path": "README.md" }),
            "user_123",
            &ToolInvocationContext::default(),
        );

        assert!(score > 0.8, "Read-only operations should have high confidence");
    }

    #[test]
    fn test_confidence_score_high_risk() {
        let score = compute_confidence_score(
            "execute_shell",
            &json!({ "command": "cargo build" }),
            "user_123",
            &ToolInvocationContext::default(),
        );

        assert!(score < 0.5, "High-risk operations should have lower confidence");
    }

    #[tokio::test]
    async fn test_list_files_bridge() {
        let bridge = ListFilesBridge;
        let registry = ToolSchemaRegistry::load("tool-schema-registry.yaml");
        let universe = Universe::new();

        let result = bridge.translate_and_execute(
            &json!({
                "path": ".",
                "recursive": false,
                "pattern": "*.rs"
            }),
            &registry,
            &universe,
        )
        .await;

        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.get("files").is_some());
    }

    #[test]
    fn test_approval_required_for_write() {
        let tool = ToolSchemaRegistry::load("tool-schema-registry.yaml")
            .get_tool("write_file")
            .unwrap();

        assert!(tool.safety.approval_required);
    }

    #[test]
    fn test_approval_not_required_for_read() {
        let tool = ToolSchemaRegistry::load("tool-schema-registry.yaml")
            .get_tool("read_file")
            .unwrap();

        assert!(!tool.safety.approval_required);
    }
}
```

---

## 7. YAML Registry Loading (Rust)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolSchema {
    pub id: String,
    pub name: String,
    pub category: String,
    pub risk_level: String,
    pub description: String,
    pub permissions_required: Vec<String>,
    pub bonsai_mapping: BonsaiMapping,
    pub safety: SafetyConfig,
    pub version: String,
    pub deprecated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BonsaiMapping {
    pub strategy: String,  // direct | bridge | cloud_fallback
    pub tool_name: Option<String>,
    pub parameter_translation: Vec<ParameterRule>,
    pub result_translation: Vec<ResultRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterRule {
    pub copilot_field: String,
    pub bonsai_field: Option<String>,
    pub transform: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultRule {
    pub bonsai_field: Option<String>,
    pub copilot_field: String,
    pub constant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetyConfig {
    pub approval_required: bool,
    pub user_prompt: Option<String>,
    pub path_validation: Option<String>,
    pub secret_scan: bool,
}

pub struct ToolSchemaRegistry {
    tools: HashMap<String, ToolSchema>,
}

impl ToolSchemaRegistry {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let content = std::fs::read_to_string(path)
            .expect("Failed to read registry YAML");

        let registry: serde_yaml::Value = serde_yaml::from_str(&content)
            .expect("Failed to parse YAML");

        let mut tools = HashMap::new();

        if let Some(tool_list) = registry.get("tools").and_then(|v| v.as_sequence()) {
            for tool_value in tool_list {
                let tool: ToolSchema = serde_yaml::from_value(tool_value.clone())
                    .expect("Failed to deserialize tool");
                tools.insert(tool.name.clone(), tool);
            }
        }

        Self { tools }
    }

    pub fn get_tool(&self, name: &str) -> Option<ToolSchema> {
        self.tools.get(name).cloned()
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    pub fn get_tools_by_category(&self, category: &str) -> Vec<ToolSchema> {
        self.tools
            .values()
            .filter(|t| t.category == category)
            .cloned()
            .collect()
    }
}
```

---

## 8. Next Steps

### Phase 1 Implementation Tasks

1. **Registry Loading** – Implement `ToolSchemaRegistry::load()` with full YAML parsing
2. **Parameter Translation** – Test with all parameter types (string, int, bool, array, object)
3. **Result Translation** – Verify round-trip translation for each tool
4. **Confidence Scoring** – Implement all 5 scoring components and test thresholds
5. **Bridge Template** – Create base `ToolBridge` trait and implement 3 example bridges
6. **Approval Gate** – Wire to Tauri frontend for user prompts
7. **Universe Integration** – Emit events to Universe for all tool invocations

### Testing Checklist

- [ ] Unit tests for each translation rule
- [ ] Integration tests with mock Bonsai MCP server
- [ ] End-to-end tests with real MCP server
- [ ] Approval flow tests
- [ ] Error handling tests
- [ ] Permission validation tests
- [ ] Telemetry logging tests

### Documentation Checklist

- [ ] API documentation for each tool
- [ ] Bridge implementation guide
- [ ] Approval flow user guide
- [ ] Error handling troubleshooting guide
- [ ] Telemetry dashboard setup guide

---

## References

- Main design: `docs/25-COPILOT-TOOL-TRANSLATION-ENGINE.md`
- Schema registry: `crates/bonsai-mcp-server/tool-schema-registry.yaml`
- Existing MCP server: `crates/bonsai-mcp-server/src/`
- Universe integration: `bonsai-workspace/src-tauri/src/universe_hooks.rs`

---

**Ready to start implementation!**
