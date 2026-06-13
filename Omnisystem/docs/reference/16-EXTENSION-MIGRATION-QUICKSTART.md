# Extension Migration: Quick Start for Engineers

> Get Phase 1 implemented and deployed in 2 weeks.

---

## Phase 1 Overview

**Goal:** Deploy Bonsai Sovereign Proxy in idle mode to 100% of VS Code users.

**What it does:**
- Installs alongside Copilot/Claude Code
- Routes **all traffic to cloud APIs** (no change in behavior)
- Collects baseline telemetry (cloud latency, throughput, device specs)
- Runs silently; users see optional info notification

**Timeline:** 2 weeks (internal test → early adopters → general release)

**Success Criteria:**
- ✅ 0 crashes
- ✅ 0 VS Code hangs
- ✅ Telemetry flowing (>95% success)
- ✅ Cloud latency stable (no >2% regression)
- ✅ <1% uninstall rate in week 2

---

## Code Implementation (Days 1-4)

### Step 1: Create Feature Flag Structure

**File:** `bonsai-workspace/src-tauri/src/extension_config.rs` (new)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub copilot: ExtensionSettings,
    pub claude_code: ExtensionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionSettings {
    pub enabled: bool,
    pub use_local_inference: bool,
    pub cloud_fallback: bool,
    pub offline_only: bool,
    pub admin_policy: Option<AdminPolicy>,
    pub feature_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPolicy {
    pub policy_id: String,
    pub enforcement: String,  // "soft" or "hard"
    pub applied_at: i64,
}

impl Default for ExtensionConfig {
    fn default() -> Self {
        Self {
            copilot: ExtensionSettings {
                enabled: true,
                use_local_inference: false,  // Phase 1: cloud-only
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
    pub fn load() -> Result<Self, String> {
        // Phase 1: return defaults
        // Phase 2+: load from VS Code settings
        Ok(Self::default())
    }
    
    pub fn validate(&self) -> Result<(), String> {
        // Phase 1: minimal validation
        if self.copilot.offline_only && !self.copilot.use_local_inference {
            return Err("offline_only requires use_local_inference".into());
        }
        Ok(())
    }
}

static EXTENSION_CONFIG: Lazy<RwLock<ExtensionConfig>> =
    Lazy::new(|| RwLock::new(ExtensionConfig::default()));

pub fn get_config() -> ExtensionConfig {
    EXTENSION_CONFIG.read().unwrap().clone()
}

pub fn reload_config() -> Result<(), String> {
    let new_config = ExtensionConfig::load()?;
    *EXTENSION_CONFIG.write().unwrap() = new_config;
    Ok(())
}
```

### Step 2: Create Idle Mode Proxy

**File:** `bonsai-workspace/src-tauri/src/extension_proxy.rs` (new)

```rust
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionRequest {
    pub request_type: String,  // "completion", "chat", etc.
    pub extension: String,     // "copilot" or "claude_code"
    pub content: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionResponse {
    pub response_id: String,
    pub content: String,
    pub inference_mode: String,  // "cloud_direct" for Phase 1
    pub latency_ms: u64,
    pub success: bool,
    pub error_reason: Option<String>,
}

pub struct ExtensionProxy {
    cloud_client: CloudApiClient,
    telemetry: TelemetryCollector,
}

impl ExtensionProxy {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            cloud_client: CloudApiClient::new()?,
            telemetry: TelemetryCollector::new(),
        })
    }
    
    /// Phase 1: route all requests to cloud
    pub async fn handle_request(&mut self, req: ExtensionRequest) -> Result<ExtensionResponse, String> {
        let start = Instant::now();
        
        // Call cloud API
        match self.cloud_client.infer(&req.extension, &req.content).await {
            Ok(response) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                
                // Record telemetry
                self.telemetry.record_completion(
                    &req.extension,
                    "cloud_direct",
                    latency_ms,
                    true,
                    None,
                );
                
                Ok(ExtensionResponse {
                    response_id: req.request_id,
                    content: response,
                    inference_mode: "cloud_direct".into(),
                    latency_ms,
                    success: true,
                    error_reason: None,
                })
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                
                self.telemetry.record_completion(
                    &req.extension,
                    "cloud_direct",
                    latency_ms,
                    false,
                    Some(&format!("{:?}", e)),
                );
                
                Err(e)
            }
        }
    }
}

pub struct CloudApiClient {
    // Placeholder; would call actual cloud APIs
}

impl CloudApiClient {
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }
    
    pub async fn infer(&self, extension: &str, prompt: &str) -> Result<String, String> {
        // Phase 1: stub; returns mock response
        // Phase 2+: call actual cloud API
        Ok("Mock response from cloud".into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionMetric {
    pub extension: String,
    pub inference_mode: String,
    pub latency_ms: u64,
    pub success: bool,
    pub error_kind: Option<String>,
}

pub struct TelemetryCollector {
    completions: Vec<CompletionMetric>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self { completions: Vec::new() }
    }
    
    pub fn record_completion(
        &mut self,
        extension: &str,
        mode: &str,
        latency_ms: u64,
        success: bool,
        error_kind: Option<&str>,
    ) {
        self.completions.push(CompletionMetric {
            extension: extension.into(),
            inference_mode: mode.into(),
            latency_ms,
            success,
            error_kind: error_kind.map(String::from),
        });
        
        // Keep last 10k completions
        if self.completions.len() > 10000 {
            self.completions.remove(0);
        }
    }
    
    pub fn aggregate(&self) -> AggregatedMetrics {
        let mut stats = AggregatedMetrics::default();
        let mut latencies = Vec::new();
        
        for metric in &self.completions {
            if metric.success {
                stats.total += 1;
                latencies.push(metric.latency_ms);
            } else {
                stats.failures += 1;
            }
        }
        
        latencies.sort_unstable();
        if !latencies.is_empty() {
            stats.p50 = latencies[latencies.len() / 2] as f64;
            stats.p99 = latencies[latencies.len() * 99 / 100] as f64;
        }
        
        stats
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub total: u64,
    pub failures: u64,
    pub p50: f64,
    pub p99: f64,
}
```

### Step 3: Add Tauri Commands

**File:** `bonsai-workspace/src-tauri/src/extension_commands.rs` (new or updated)

```rust
use crate::extension_config::{self, ExtensionConfig};
use crate::extension_proxy::ExtensionRequest;

#[tauri::command]
#[specta::specta]
pub fn get_extension_config() -> ExtensionConfig {
    extension_config::get_config()
}

#[tauri::command]
#[specta::specta]
pub fn set_extension_config(config: ExtensionConfig) -> Result<(), String> {
    config.validate()?;
    // Phase 1: just validate and update in-memory
    // Phase 2+: persist to VS Code settings
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn extension_handle_request(req: ExtensionRequest) -> Result<String, String> {
    // Phase 1: route to cloud
    // Phase 2+: use feature flags to decide routing
    Ok("Response from extension proxy".into())
}
```

**Update** `bonsai-workspace/src-tauri/src/lib.rs`:

```rust
mod extension_config;
mod extension_proxy;
mod extension_commands;

// In tauri::Builder::default()
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    extension_commands::get_extension_config,
    extension_commands::set_extension_config,
    extension_commands::extension_handle_request,
])
```

### Step 4: Unit Tests

**File:** `bonsai-workspace/src-tauri/src/extension_config.rs` (add to bottom)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config_is_cloud_only() {
        let config = ExtensionConfig::default();
        assert!(!config.copilot.use_local_inference);
        assert!(config.copilot.cloud_fallback);
        assert!(!config.copilot.offline_only);
    }
    
    #[test]
    fn test_config_validation_offline_without_local() {
        let mut config = ExtensionConfig::default();
        config.copilot.offline_only = true;
        config.copilot.use_local_inference = false;
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_validation_passes_for_cloud_only() {
        let config = ExtensionConfig::default();
        assert!(config.validate().is_ok());
    }
}
```

Run tests:
```bash
cd bonsai-workspace/src-tauri
cargo test extension_config
```

---

## Telemetry Dashboard (Days 5-6)

### Create Dashboard UI Component

**File:** `bonsai-workspace/src/lib/components/ExtensionHealthDashboard.svelte` (new)

```svelte
<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api';
  
  let metrics = {
    total_completions: 0,
    failures: 0,
    p50_latency: 0,
    p99_latency: 0,
    failure_rate: 0,
  };
  
  let loading = true;
  
  onMount(async () => {
    // Poll telemetry every 5 seconds
    const interval = setInterval(async () => {
      try {
        // This would fetch aggregated metrics from Tauri backend
        const response = await invoke('get_extension_health');
        metrics = response;
      } catch (e) {
        console.error('Failed to fetch metrics:', e);
      }
    }, 5000);
    
    loading = false;
    return () => clearInterval(interval);
  });
</script>

<div class="dashboard">
  <h2>Bonsai Extension Health</h2>
  
  {#if loading}
    <p>Loading...</p>
  {:else}
    <div class="metric">
      <span class="label">Total Completions:</span>
      <span class="value">{metrics.total_completions}</span>
    </div>
    
    <div class="metric">
      <span class="label">Failure Rate:</span>
      <span class="value">{(metrics.failure_rate * 100).toFixed(2)}%</span>
    </div>
    
    <div class="metric">
      <span class="label">Latency (P50/P99):</span>
      <span class="value">{metrics.p50_latency}ms / {metrics.p99_latency}ms</span>
    </div>
  {/if}
</div>

<style>
  .dashboard {
    padding: 1rem;
    border: 1px solid #ddd;
    border-radius: 8px;
  }
  
  .metric {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #eee;
  }
  
  .label {
    font-weight: bold;
  }
  
  .value {
    font-family: monospace;
  }
</style>
```

---

## Testing Checklist (Days 7-10)

### Internal Testing (Bonsai Team)

**Device Coverage:**
- [ ] Windows 10/11 (x86_64)
- [ ] macOS 12+ (Intel & Apple Silicon)
- [ ] Ubuntu 20.04+ (x86_64)

**Test Cases:**
- [ ] Extension installs without errors
- [ ] Extension doesn't crash VS Code
- [ ] Extension doesn't slow down editor
- [ ] Memory usage stable (<100MB delta)
- [ ] Telemetry flows to backend
- [ ] Telemetry aggregation is accurate
- [ ] Feature flags load correctly

**Monitoring:**
- [ ] Dashboard shows metrics live
- [ ] Alert system working (mock an alert)
- [ ] Log output is clean (no errors)

### Early Adopter Testing (100 Users, Days 11-14)

**Recruitment:**
- Email: "Bonsai Sovereign Proxy Early Access — Help us test!"
- Incentive: Early access to Phase 2 features, swag
- Slack channel: #bonsai-early-adopters

**Daily Monitoring (4-7pm PT, 30 min):**
- [ ] Day 1: Install rate, crash rate, error logs
- [ ] Day 2: Telemetry flow, latency baseline
- [ ] Day 3: NPS survey, feedback Slack posts
- [ ] Day 4: Stability check, any anomalies?

**Success Criteria for Early Adopters:**
- ✅ 0 crashes in 4 days
- ✅ <1% uninstall rate
- ✅ Telemetry >99% accurate
- ✅ Team NPS ≥4.0/5.0

---

## Deployment Plan (Day 15+)

### General Release to All Users

**Day 1: Publish to VS Code Marketplace**
```bash
# Build extension
cd bonsai-extensions/extension
npm run build

# Package as VSIX
vsce package

# Publish (requires MS token)
vsce publish
```

**Day 1-5: Gradual Rollout**
- Publish with `x-ms-pre-release: false` (stable channel)
- Monitor install velocity: target 100k+ installs/day
- Watch for uninstall surge (alert if >2%)
- Watch for crash reports (alert if >0.1%)

**Day 6-14: Stabilization**
- Continue hourly monitoring
- Respond to support tickets
- Fix any bugs found
- Post daily status in Slack

---

## Key Metrics to Track

| Metric | Phase 1 Target | Dashboard Widget |
|--------|---------|---|
| Installs | >80% of active users | Big counter |
| Uninstall rate | <1% | Red if >1% |
| Crash rate | <0.01% | Red if >0.1% |
| Cloud API latency (p50) | <600ms | Green line chart |
| Cloud API latency (p99) | <2000ms | Green line chart |
| Error rate | <0.5% | Red if >1% |
| Telemetry success rate | >99% | Green counter |
| Device inventory | Show distribution | Stacked bar chart (OS, CPU, RAM) |

---

## Phase 1 Rollback Plan

**If critical issue found:**

1. **Detection:** Metric crosses red line (e.g., crash rate >1%)
2. **Verification:** On-call engineer confirms (5 min)
3. **Decision:** Is rollback necessary? (15 min)
4. **Rollback:** Unpublish extension from marketplace (5 min)
5. **Communication:** Post in-app notification + Twitter post (10 min)
6. **Investigation:** Root cause analysis (2-24 hours)

**Rollback will happen if:**
- Crash rate >1% of users
- Data loss incidents
- Cloud API integration broken
- Security issue discovered

---

## Next Actions

### Immediate (This Week)
1. Create 3 files: `extension_config.rs`, `extension_proxy.rs`, `extension_commands.rs`
2. Write unit tests, verify they pass
3. Create telemetry dashboard UI
4. Deploy to internal team for testing

### Week 2
1. Monitor internal testing (fix any crashes)
2. Recruit 100 early adopters
3. Deploy to early adopters (watch metrics hourly)
4. Fix any reported issues
5. Go/no-go decision: ready for general release?

### Week 3+
1. General release to all VS Code users
2. Hourly monitoring (first 3 days), then daily
3. Preparation for Phase 2 (local inference opt-in)

---

## Slack Channel

Create: `#bonsai-extension-migration`

**Daily standup:** 4pm PT
- What did we ship?
- What are top metrics?
- Any issues to escalate?
- Next 24-hour plan

---

## Reference

- **Full strategy:** [16-EXTENSION-MIGRATION-STRATEGY.md](16-EXTENSION-MIGRATION-STRATEGY.md)
- **Implementation guide:** [16-EXTENSION-MIGRATION-IMPLEMENTATION.md](16-EXTENSION-MIGRATION-IMPLEMENTATION.md)
- **Execution checklist:** [16-EXTENSION-MIGRATION-CHECKLIST.md](16-EXTENSION-MIGRATION-CHECKLIST.md)
- **Navigation index:** [16-EXTENSION-MIGRATION-INDEX.md](16-EXTENSION-MIGRATION-INDEX.md)

---

**Good luck! Let's ship this. 🚀**
