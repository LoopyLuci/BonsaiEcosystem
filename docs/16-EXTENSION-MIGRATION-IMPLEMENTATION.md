# Extension Migration: Implementation Guide

> Code-level implementation details for transitioning the Copilot/Claude Code extensions to Bonsai Sovereign Proxy.

---

## 1. Feature Flag Architecture

### Location: `bonsai-workspace/src-tauri/src/extension_config.rs` (new file)

```rust
//! Extension feature flag configuration
//! 
//! This module handles:
//! 1. Feature flag loading from vs code settings + Blueprint
//! 2. Flag validation and dependency checking
//! 3. Hot-reload support (flags update without restart)
//! 4. Enterprise policy enforcement

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Complete feature flag configuration for extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    /// Copilot extension settings
    pub copilot: ExtensionSettings,
    /// Claude Code extension settings
    pub claude_code: ExtensionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionSettings {
    /// Extension is active and running
    pub enabled: bool,
    /// Route inference to local model
    pub use_local_inference: bool,
    /// Fall back to cloud on local failure
    pub cloud_fallback: bool,
    /// Disable all cloud access (requires use_local_inference=true)
    pub offline_only: bool,
    /// Optional: enterprise admin policy (overrides user settings)
    pub admin_policy: Option<AdminPolicy>,
    /// Extension-specific feature flags (Phase 4+)
    pub feature_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPolicy {
    /// Policy ID from Blueprint
    pub policy_id: String,
    /// Enforcement level: "soft" (user can override) or "hard" (enforced)
    pub enforcement: String,
    /// When policy was applied
    pub applied_at: i64,
}

impl Default for ExtensionConfig {
    fn default() -> Self {
        Self {
            copilot: ExtensionSettings {
                enabled: true,
                use_local_inference: false,  // Phase 1-2: cloud-only
                cloud_fallback: true,
                offline_only: false,
                admin_policy: None,
                feature_flags: HashMap::new(),
            },
            claude_code: ExtensionSettings {
                enabled: true,
                use_local_inference: false,
                cloud_fallback: true,
                offline_only: false,
                admin_policy: None,
                feature_flags: HashMap::new(),
            },
        }
    }
}

impl ExtensionConfig {
    /// Load configuration from VS Code settings + Blueprint
    pub fn load() -> BonsaiResult<Self> {
        // 1. Load user settings from vs code (via Tauri read_text_file)
        let user_settings = load_vscode_settings()
            .context("loading vs code settings")?;
        
        // 2. Load Blueprint policy (enterprise admin settings)
        let blueprint_policy = load_blueprint_policy()
            .context("loading blueprint policy")?;
        
        // 3. Merge: Blueprint policy + user settings
        let mut config = Self::from_vscode_settings(&user_settings)?;
        if let Some(policy) = blueprint_policy {
            config = config.apply_blueprint_policy(policy)?;
        }
        
        // 4. Validate config invariants
        config.validate()?;
        
        Ok(config)
    }
    
    /// Validate feature flag invariants
    /// 
    /// Rules:
    /// - offline_only ⟹ use_local_inference ∧ ¬cloud_fallback
    /// - ¬use_local_inference ⟹ ¬<feature_flags that require local>
    pub fn validate(&self) -> BonsaiResult<()> {
        self._validate_settings(&self.copilot, "copilot")?;
        self._validate_settings(&self.claude_code, "claude_code")?;
        Ok(())
    }
    
    fn _validate_settings(&self, settings: &ExtensionSettings, name: &str) -> BonsaiResult<()> {
        // offline_only ⟹ use_local_inference ∧ ¬cloud_fallback
        if settings.offline_only {
            if !settings.use_local_inference {
                bail!(
                    ErrorKind::Config,
                    "{}.offline_only requires use_local_inference=true",
                    name
                );
            }
            if settings.cloud_fallback {
                bail!(
                    ErrorKind::Config,
                    "{}.offline_only requires cloud_fallback=false",
                    name
                );
            }
        }
        
        // Check feature flag dependencies
        if !settings.use_local_inference {
            if settings.feature_flags.get("predict_urv") == Some(&true) {
                bail!(
                    ErrorKind::Config,
                    "{}.feature_flags.predict_urv requires use_local_inference=true",
                    name
                );
            }
        }
        
        Ok(())
    }
    
    /// Get effective setting: enterprise policy overrides user setting (if enforcement=hard)
    pub fn get_effective_flag(settings: &ExtensionSettings, flag: &str) -> bool {
        match &settings.admin_policy {
            Some(policy) if policy.enforcement == "hard" => {
                // Enterprise policy overrides user setting
                // (Policy values are already merged into this settings struct)
                false  // Placeholder; actual logic depends on flag
            }
            _ => {
                // User setting or soft policy
                match flag {
                    "use_local_inference" => settings.use_local_inference,
                    "cloud_fallback" => settings.cloud_fallback,
                    "offline_only" => settings.offline_only,
                    _ => settings.feature_flags.get(flag).copied().unwrap_or(false),
                }
            }
        }
    }
}

/// Global config (hot-reloadable)
static EXTENSION_CONFIG: Lazy<RwLock<ExtensionConfig>> =
    Lazy::new(|| RwLock::new(ExtensionConfig::default()));

/// Get current config
pub fn get_config() -> ExtensionConfig {
    EXTENSION_CONFIG.read().unwrap().clone()
}

/// Hot-reload config (called when VS Code settings change or Blueprint sync)
pub fn reload_config() -> BonsaiResult<()> {
    let new_config = ExtensionConfig::load()?;
    *EXTENSION_CONFIG.write().unwrap() = new_config;
    
    // Notify UI that config changed
    crate::system_event_bus::post_event(
        SystemEvent::ConfigChanged {
            extension_config_changed: true,
        }
    );
    
    Ok(())
}

fn load_vscode_settings() -> BonsaiResult<serde_json::Value> {
    // Read .vscode/settings.json
    let vscode_dir = dirs::config_dir()
        .unwrap_or_default()
        .join("Code/User");
    let settings_path = vscode_dir.join("settings.json");
    
    if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)
            .context("reading vs code settings")?;
        serde_json::from_str(&content)
            .context("parsing vs code settings.json")
    } else {
        Ok(serde_json::json!({}))
    }
}

fn load_blueprint_policy() -> BonsaiResult<Option<BlueprintPolicy>> {
    // Fetch org policy from Blueprint API
    // Stub: would call bonsai-blueprint-api
    Ok(None)
}
```

### Tauri Command Interface

Add to `bonsai-workspace/src-tauri/src/lib.rs`:

```rust
// Add to invoke_handler
mod extension_config;  // Include new module

// In main setup:
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        // ... existing commands
        extension_commands::get_extension_config,
        extension_commands::set_extension_config,
        extension_commands::reload_extension_config,
    ])
```

New file: `bonsai-workspace/src-tauri/src/extension_commands.rs`:

```rust
use crate::extension_config::{self, ExtensionConfig, ExtensionSettings};
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub fn get_extension_config() -> ExtensionConfig {
    extension_config::get_config()
}

#[tauri::command]
#[specta::specta]
pub fn set_extension_config(config: ExtensionConfig) -> Result<(), String> {
    config.validate().map_err(|e| e.to_string())?;
    
    // Write to .vscode/settings.json
    let vscode_dir = dirs::config_dir()
        .unwrap_or_default()
        .join("Code/User");
    let settings_path = vscode_dir.join("settings.json");
    
    let mut settings: serde_json::Value = 
        if settings_path.exists() {
            serde_json::from_str(&std::fs::read_to_string(&settings_path)?)
                .map_err(|e| e.to_string())?
        } else {
            serde_json::json!({})
        };
    
    // Update bonsai.extensions.* keys
    settings["bonsai"]["extensions"] = serde_json::to_value(&config)
        .map_err(|e| e.to_string())?;
    
    std::fs::write(
        &settings_path,
        serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;
    
    // Hot-reload config
    extension_config::reload_config().map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn reload_extension_config() -> Result<(), String> {
    extension_config::reload_config().map_err(|e| e.to_string())
}
```

---

## 2. Proxy Traffic Routing

### Location: `bonsai-workspace/src-tauri/src/extension_proxy.rs` (new file)

```rust
//! Extension proxy: routes Copilot/Claude Code requests to local or cloud inference
//! 
//! Architecture:
//! 1. Intercept request from extension (completion, chat, etc.)
//! 2. Check feature flags (use_local_inference, offline_only)
//! 3. Route to local model or cloud API
//! 4. If local fails and cloud_fallback=true, retry on cloud
//! 5. Record telemetry (latency, errors, fallback)

use crate::extension_config::ExtensionConfig;
use crate::extension_telemetry::TelemetryCollector;
use bonsai_error::{BonsaiError, BonsaiResult, ErrorKind};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Request from Copilot or Claude Code extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionRequest {
    pub request_type: RequestType,
    pub extension: String,  // "copilot" or "claude_code"
    pub content: String,    // Code context or chat message
    pub request_id: String, // For tracking
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RequestType {
    Completion,
    Chat,
    CodeAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionResponse {
    pub response_id: String,
    pub content: String,
    pub inference_mode: InferenceMode,  // Where request was processed
    pub latency_ms: u64,
    pub success: bool,
    pub error_reason: Option<String>,   // If !success
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InferenceMode {
    Local,
    CloudFallback,  // Local failed, fell back to cloud
    CloudDirect,    // Cloud was the only option
}

pub struct ExtensionProxy {
    config: ExtensionConfig,
    local_model: LocalInferenceClient,
    cloud_api: CloudApiClient,
    telemetry: TelemetryCollector,
}

impl ExtensionProxy {
    pub fn new() -> BonsaiResult<Self> {
        let config = crate::extension_config::get_config();
        Ok(Self {
            config,
            local_model: LocalInferenceClient::new()?,
            cloud_api: CloudApiClient::new()?,
            telemetry: TelemetryCollector::new(),
        })
    }
    
    /// Route request to local or cloud inference
    pub async fn handle_request(&mut self, req: ExtensionRequest) -> BonsaiResult<ExtensionResponse> {
        let start = Instant::now();
        let extension = &req.extension;
        let settings = if extension == "copilot" {
            &self.config.copilot
        } else {
            &self.config.claude_code
        };
        
        // Determine inference path
        let use_local = settings.use_local_inference && !settings.offline_only;
        let cloud_fallback = settings.cloud_fallback && !settings.offline_only;
        
        // Try local inference first (if enabled)
        if use_local {
            match self.call_local_inference(&req).await {
                Ok(response) => {
                    let latency_ms = start.elapsed().as_millis() as u64;
                    
                    // Record telemetry
                    self.telemetry.record_completion(
                        extension,
                        InferenceMode::Local,
                        latency_ms,
                        true,
                        None,
                    );
                    
                    return Ok(ExtensionResponse {
                        response_id: req.request_id,
                        content: response,
                        inference_mode: InferenceMode::Local,
                        latency_ms,
                        success: true,
                        error_reason: None,
                    });
                }
                Err(e) => {
                    // Local failed
                    if !cloud_fallback {
                        // Offline mode: no fallback
                        let latency_ms = start.elapsed().as_millis() as u64;
                        self.telemetry.record_completion(
                            extension,
                            InferenceMode::Local,
                            latency_ms,
                            false,
                            Some("offline_only"),
                        );
                        
                        return Err(e.context("local inference failed (offline_only mode)"));
                    }
                    
                    // Fall back to cloud
                    log::warn!(
                        "local inference failed for request {}: {}; falling back to cloud",
                        req.request_id, e
                    );
                }
            }
        }
        
        // Cloud inference (either direct or fallback)
        match self.call_cloud_inference(&req).await {
            Ok(response) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                let mode = if use_local {
                    InferenceMode::CloudFallback
                } else {
                    InferenceMode::CloudDirect
                };
                
                self.telemetry.record_completion(
                    extension,
                    mode,
                    latency_ms,
                    true,
                    None,
                );
                
                Ok(ExtensionResponse {
                    response_id: req.request_id,
                    content: response,
                    inference_mode: mode,
                    latency_ms,
                    success: true,
                    error_reason: None,
                })
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                self.telemetry.record_completion(
                    extension,
                    InferenceMode::CloudDirect,
                    latency_ms,
                    false,
                    Some(&format!("{:?}", e.kind())),
                );
                
                Err(e)
            }
        }
    }
    
    async fn call_local_inference(&self, req: &ExtensionRequest) -> BonsaiResult<String> {
        // Timeout: 2 seconds
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            self.local_model.infer(&req.content)
        )
        .await
        .map_err(|_| BonsaiError::new(
            ErrorKind::Timeout,
            "local inference timed out after 2s"
        ))?
    }
    
    async fn call_cloud_inference(&self, req: &ExtensionRequest) -> BonsaiResult<String> {
        // Timeout: 10 seconds (cloud is usually slower)
        tokio::time::timeout(
            std::time::Duration::from_secs(10),
            self.cloud_api.infer(
                &req.extension,
                &req.content,
                &req.request_id
            )
        )
        .await
        .map_err(|_| BonsaiError::new(
            ErrorKind::Timeout,
            "cloud inference timed out after 10s"
        ))?
    }
}

/// Local inference client (wraps bonsai-inference crate)
pub struct LocalInferenceClient {
    // Placeholder
}

impl LocalInferenceClient {
    pub fn new() -> BonsaiResult<Self> {
        Ok(Self {})
    }
    
    pub async fn infer(&self, prompt: &str) -> BonsaiResult<String> {
        // Stub: would call local model via bonsai-inference
        Ok("Generated completion".into())
    }
}

/// Cloud API client (wraps existing Copilot/Claude Code SDKs)
pub struct CloudApiClient {
    copilot_token: Option<String>,
    claude_code_token: Option<String>,
}

impl CloudApiClient {
    pub fn new() -> BonsaiResult<Self> {
        // Load credentials from VS Code secret storage
        let copilot_token = tauri_plugin_store::with_store(|store| {
            store.get("extension:copilot:api-key").cloned()
        });
        let claude_code_token = tauri_plugin_store::with_store(|store| {
            store.get("extension:claude-code:api-key").cloned()
        });
        
        Ok(Self {
            copilot_token,
            claude_code_token,
        })
    }
    
    pub async fn infer(
        &self,
        extension: &str,
        prompt: &str,
        request_id: &str,
    ) -> BonsaiResult<String> {
        match extension {
            "copilot" => {
                let token = self.copilot_token.as_ref()
                    .ok_or_else(|| BonsaiError::new(
                        ErrorKind::Auth,
                        "copilot api key not configured"
                    ))?;
                
                // Call Copilot API
                // self.call_copilot_api(token, prompt).await
                Ok("Copilot response".into())
            }
            "claude_code" => {
                let token = self.claude_code_token.as_ref()
                    .ok_or_else(|| BonsaiError::new(
                        ErrorKind::Auth,
                        "claude code api key not configured"
                    ))?;
                
                // Call Claude Code API
                // self.call_claude_code_api(token, prompt).await
                Ok("Claude response".into())
            }
            _ => Err(BonsaiError::new(
                ErrorKind::Invalid,
                format!("unknown extension: {}", extension)
            ))
        }
    }
}
```

---

## 3. Telemetry Collection

### Location: `bonsai-workspace/src-tauri/src/extension_telemetry.rs` (new file)

```rust
//! Telemetry collection for extension migration tracking
//! 
//! Metrics:
//! - Usage (completions, chat messages)
//! - Performance (latency percentiles)
//! - Reliability (error rates, fallbacks)
//! - Engagement (user satisfaction)

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionMetric {
    pub timestamp: i64,
    pub extension: String,
    pub inference_mode: String,  // "local", "cloud_fallback", "cloud_direct"
    pub latency_ms: u64,
    pub success: bool,
    pub error_kind: Option<String>,
}

/// Time-windowed telemetry aggregator
pub struct TelemetryCollector {
    completions: VecDeque<CompletionMetric>,  // Last 10,000 completions
    session_id: String,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self {
            completions: VecDeque::new(),
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
    
    /// Record a completion (local or cloud)
    pub fn record_completion(
        &mut self,
        extension: &str,
        mode: crate::extension_proxy::InferenceMode,
        latency_ms: u64,
        success: bool,
        error_kind: Option<&str>,
    ) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        let mode_str = match mode {
            crate::extension_proxy::InferenceMode::Local => "local",
            crate::extension_proxy::InferenceMode::CloudFallback => "cloud_fallback",
            crate::extension_proxy::InferenceMode::CloudDirect => "cloud_direct",
        };
        
        self.completions.push_back(CompletionMetric {
            timestamp: now,
            extension: extension.to_string(),
            inference_mode: mode_str.to_string(),
            latency_ms,
            success,
            error_kind: error_kind.map(String::from),
        });
        
        // Keep last 10k completions only
        while self.completions.len() > 10000 {
            self.completions.pop_front();
        }
    }
    
    /// Compute aggregated metrics for telemetry submission
    pub fn aggregate(&self) -> AggregatedMetrics {
        let mut stats = AggregatedMetrics::default();
        
        let mut local_latencies = Vec::new();
        let mut cloud_latencies = Vec::new();
        
        for metric in &self.completions {
            if metric.success {
                stats.total_completions += 1;
                match metric.inference_mode.as_str() {
                    "local" => {
                        stats.completions_local += 1;
                        local_latencies.push(metric.latency_ms);
                    }
                    "cloud_fallback" => {
                        stats.completions_cloud_fallback += 1;
                        cloud_latencies.push(metric.latency_ms);
                    }
                    "cloud_direct" => {
                        stats.completions_cloud += 1;
                        cloud_latencies.push(metric.latency_ms);
                    }
                    _ => {}
                }
            } else {
                stats.failures += 1;
                if let Some(ref kind) = metric.error_kind {
                    *stats.failures_by_kind.entry(kind.clone()).or_insert(0) += 1;
                }
            }
        }
        
        // Compute percentiles
        local_latencies.sort_unstable();
        cloud_latencies.sort_unstable();
        
        if !local_latencies.is_empty() {
            stats.latency_local_p50 = local_latencies[local_latencies.len() / 2] as f64;
            stats.latency_local_p99 = local_latencies[
                std::cmp::min(local_latencies.len() * 99 / 100, local_latencies.len() - 1)
            ] as f64;
        }
        
        if !cloud_latencies.is_empty() {
            stats.latency_cloud_p50 = cloud_latencies[cloud_latencies.len() / 2] as f64;
            stats.latency_cloud_p99 = cloud_latencies[
                std::cmp::min(cloud_latencies.len() * 99 / 100, cloud_latencies.len() - 1)
            ] as f64;
        }
        
        stats.fallback_rate = if stats.completions_local > 0 {
            (stats.completions_cloud_fallback as f64 /
             (stats.completions_local + stats.completions_cloud_fallback) as f64) * 100.0
        } else {
            0.0
        };
        
        stats
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub total_completions: u64,
    pub completions_local: u64,
    pub completions_cloud: u64,
    pub completions_cloud_fallback: u64,
    pub failures: u64,
    pub failures_by_kind: std::collections::HashMap<String, u64>,
    
    pub latency_local_p50: f64,
    pub latency_local_p99: f64,
    pub latency_cloud_p50: f64,
    pub latency_cloud_p99: f64,
    pub fallback_rate: f64,  // Percentage
}
```

---

## 4. Phase-by-Phase Checklist

### Phase 1: Idle Mode (Weeks 1-2)

- [ ] Implement `ExtensionConfig` struct in `extension_config.rs`
- [ ] Add feature flag getters/setters to Tauri commands
- [ ] Implement proxy routing (idle mode: all traffic → cloud)
- [ ] Implement basic telemetry collection
- [ ] Create Phase 1 dashboard (cloud baseline metrics)
- [ ] Deploy to internal testing (Bonsai team)
- [ ] Monitor for crashes, uninstalls, latency regression
- [ ] Expand to early adopters (100 users)
- [ ] General release (all users)

### Phase 2: Local Inference Opt-In (Weeks 3-8)

- [ ] Implement local model inference routing
- [ ] Add cloud fallback on local failure
- [ ] Implement feature flag UI in settings panel
- [ ] Hot-reload feature flags (no restart required)
- [ ] Update telemetry: track local vs cloud latency
- [ ] Create Phase 2 dashboard (adoption, fallback rate, latency comparison)
- [ ] Deploy to early adopters (invite via in-app)
- [ ] Monitor key metrics (fallback <2%, satisfaction >4.0)
- [ ] Expand to 5% of users
- [ ] Expand to 20% of users
- [ ] Go/no-go decision: ready for Phase 3?

### Phase 3: Hybrid Default (Weeks 9-12)

- [ ] Flip default flag: `use_local_inference: true`
- [ ] Deploy to all users
- [ ] Monitor opt-out reasons (users who set `cloud_only: true`)
- [ ] Track session retention (do users stay in local mode?)
- [ ] Support training: troubleshooting local inference
- [ ] Post knowledge base articles
- [ ] Go/no-go decision: ready for Phase 4?

### Phase 4: Full Offline (Month 4+)

- [ ] Implement `offline_only` flag
- [ ] Enable advanced feature flags (predict-URV, traffic classification)
- [ ] Implement Universe checkpointing for conversations
- [ ] Test extensively on internal team
- [ ] Deploy to power users (opt-in)
- [ ] Monitor for data loss, sync failures
- [ ] Document compliance implications

---

## 5. Testing Checklist

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation_offline_only() {
        let config = ExtensionConfig {
            copilot: ExtensionSettings {
                offline_only: true,
                use_local_inference: false,  // INVALID
                ..Default::default()
            },
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_feature_flag_hot_reload() {
        // Change flag in memory
        // Verify UI receives ConfigChanged event
        // Verify proxy respects new flag
    }
    
    #[test]
    fn test_cloud_fallback() {
        // Mock local inference failure
        // Verify proxy falls back to cloud
        // Verify telemetry records "cloud_fallback" mode
    }
}
```

### Integration Tests

- [ ] End-to-end request: completion → local → telemetry
- [ ] End-to-end request: completion → local failure → cloud fallback
- [ ] End-to-end request: chat → cloud (if disabled locally)
- [ ] VS Code settings change → hot-reload flag → proxy respects new flag
- [ ] Blueprint policy update → flag overrides user setting (if soft enforcement)

### User Acceptance Testing (UAT)

- [ ] Phase 1: User installs extension, no impact on existing workflows
- [ ] Phase 2: User enables local inference, completions are faster
- [ ] Phase 2: Local inference fails (e.g., model timeout), cloud fallback works transparently
- [ ] Phase 3: User disables local inference, all traffic goes to cloud
- [ ] Phase 4: User enables offline mode, no internet access but still works

---

## 6. Monitoring & Alerting

### Prometheus Metrics to Export

```rust
// In bonsai-workspace/src-tauri/src/metrics.rs

lazy_static! {
    // Counters
    pub static ref COMPLETIONS_TOTAL: Counter = Counter::new(
        "bonsai_completions_total",
        "Total completions processed"
    ).unwrap();
    
    pub static ref COMPLETIONS_LOCAL: Counter = Counter::new(
        "bonsai_completions_local",
        "Completions processed locally"
    ).unwrap();
    
    pub static ref COMPLETIONS_CLOUD_FALLBACK: Counter = Counter::new(
        "bonsai_completions_cloud_fallback",
        "Completions that fell back to cloud"
    ).unwrap();
    
    // Histograms
    pub static ref LATENCY_LOCAL_MS: Histogram = Histogram::new(
        "bonsai_latency_local_ms",
        "Local inference latency in milliseconds"
    ).unwrap();
    
    pub static ref LATENCY_CLOUD_MS: Histogram = Histogram::new(
        "bonsai_latency_cloud_ms",
        "Cloud inference latency in milliseconds"
    ).unwrap();
}
```

### Alert Rules (Prometheus)

```yaml
groups:
  - name: bonsai_extension_migration
    rules:
      - alert: HighFallbackRate
        expr: (rate(bonsai_completions_cloud_fallback_total[5m]) / rate(bonsai_completions_local_total[5m])) > 0.05
        annotations:
          summary: "Fallback rate >5% (Phase 2 red line)"
          
      - alert: LocalLatencyRegression
        expr: histogram_quantile(0.99, bonsai_latency_local_ms) > histogram_quantile(0.99, bonsai_latency_cloud_ms) * 1.5
        annotations:
          summary: "Local P99 latency 50% worse than cloud"
          
      - alert: HighErrorRate
        expr: (rate(bonsai_completions_error_total[5m]) / rate(bonsai_completions_total[5m])) > 0.03
        annotations:
          summary: "Error rate >3%"
```

---

*This implementation guide pairs with [16-EXTENSION-MIGRATION-STRATEGY.md](16-EXTENSION-MIGRATION-STRATEGY.md).*
