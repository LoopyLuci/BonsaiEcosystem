use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use bonsai_cas::{CasKey, CasStore};
use bonsai_ui_utils::SwapBuffer;
use tokio::fs as tokio_fs;
use tracing::info;
use std::env;
use tokio::time::{timeout, Duration};
use bonsai_runtime::RuntimeManager;
use rand::Rng;
use tokio::task::JoinHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentManifest {
    pub id: String,
    pub version: String,
    pub cas_key: String,
    pub entrypoint: String,
    pub host_capabilities: Vec<String>,
}

/// UIOrchestrator manages generated components and supports hot-reload/rollback
pub struct UIOrchestrator {
    /// In-memory deployment history keyed by component id
    registry: Mutex<HashMap<String, Vec<ComponentManifest>>>,
    /// Active swap buffers for running components (atomic hot-swap)
    active: Mutex<HashMap<String, Arc<SwapBuffer<ComponentManifest>>>>,
    /// CAS store for artifact + metadata persistence
    cas: Arc<CasStore>,
    /// Local index path for quick recovery
    index_path: PathBuf,
    /// Active runtime controllers for running components
    runtimes: Mutex<HashMap<String, Box<dyn bonsai_runtime::RuntimeController + Send + Sync>>> ,
    /// Optional canary manifests (not fully promoted)
    canaries: Mutex<HashMap<String, ComponentManifest>>,
    /// Routing weights (0-100) for canary traffic per component
    routing_weights: Mutex<HashMap<String, u8>>,
    /// Background rollout tasks (component_id -> JoinHandle)
    rollout_tasks: Mutex<HashMap<String, JoinHandle<()>>>,
}

impl UIOrchestrator {
    /// Create a new UIOrchestrator with a CAS store and an optional index path.
    /// If `index_path` is `None`, a default `runtimes/ui_registry.json` under
    /// the current working directory is used.
    pub fn new(cas: Arc<CasStore>, index_path: Option<PathBuf>) -> Self {
        let idx = index_path.unwrap_or_else(|| PathBuf::from("runtimes/ui_registry.json"));
        Self {
            registry: Mutex::new(HashMap::new()),
            active: Mutex::new(HashMap::new()),
            cas,
            index_path: idx,
            runtimes: Mutex::new(HashMap::new()),
            canaries: Mutex::new(HashMap::new()),
            routing_weights: Mutex::new(HashMap::new()),
            rollout_tasks: Mutex::new(HashMap::new()),
        }
    }

    /// Simple generate operation: for now just returns a placeholder manifest.
    pub async fn generate_ui(&self, description: &str) -> Result<ComponentManifest> {
        // In phase 1 this will call the codegen model and builder.
        let manifest = ComponentManifest {
            id: format!("generated.{}", uuid::Uuid::new_v4()),
            version: "0.1.0".to_string(),
            cas_key: "sha256:placeholder".to_string(),
            entrypoint: "dist/main.wasm".to_string(),
            host_capabilities: vec![],
        };
        tracing::info!(desc = %description, manifest_id = %manifest.id, "generated UI manifest");
        Ok(manifest)
    }

    /// Publish a manifest into the local registry (in-memory). In future this will persist to CAS/index.
    pub async fn publish_manifest(&self, manifest: ComponentManifest) -> Result<()> {
        // Basic health check: ensure artifact exists in CAS and basic validation.
        self.health_check(&manifest).await?;

        // Insert into registry history
        let mut reg = self.registry.lock().await;
        let entry = reg.entry(manifest.id.clone()).or_default();
        entry.push(manifest.clone());

        // Ensure an active swap buffer exists (first deployment)
        let mut act = self.active.lock().await;
        if !act.contains_key(&manifest.id) {
            let buf = Arc::new(SwapBuffer::new(Arc::new(manifest.clone())));
            act.insert(manifest.id.clone(), buf);
        }

        // Persist index to disk and to CAS for recovery
        let _ = self.persist_index().await?;
        // Optionally start runtime for this manifest (best-effort)
        let _ = self.activate_manifest(&manifest, Some(10)).await;
        Ok(())
    }

    /// Hot-reload a component by inserting a new manifest version and (in future)
    /// triggering an atomic swap of the running component.
    pub async fn hot_reload(&self, manifest: ComponentManifest) -> Result<()> {
        // Validate (deep) and perform a canary run before swapping
        self.health_check(&manifest).await?;
        let canary_ok = self.canary_run(&manifest).await.unwrap_or(false);
        if !canary_ok {
            anyhow::bail!("canary run failed for manifest {}", manifest.id);
        }
        let mut reg = self.registry.lock().await;
        let entry = reg.entry(manifest.id.clone()).or_default();
        entry.push(manifest.clone());

        // Swap in active buffer
        let mut act = self.active.lock().await;
        if let Some(buf) = act.get(&manifest.id) {
            buf.store(Arc::new(manifest.clone()));
        } else {
            let buf = Arc::new(SwapBuffer::new(Arc::new(manifest.clone())));
            act.insert(manifest.id.clone(), buf);
        }

        let _ = self.persist_index().await?;
        // Spawn runtime for new manifest
        let _ = self.activate_manifest(&manifest, Some(0)).await;
        Ok(())
    }

    /// Roll back the most recent version of `component_id`. Returns Ok(true) if rolled back.
    pub async fn rollback(&self, component_id: &str) -> Result<bool> {
        let mut reg = self.registry.lock().await;
        if let Some(versions) = reg.get_mut(component_id) {
            if versions.len() > 1 {
                // remove latest
                versions.pop();
                // set running to previous
                if let Some(prev) = versions.last() {
                    let mut act = self.active.lock().await;
                    if let Some(buf) = act.get(component_id) {
                        buf.store(Arc::new(prev.clone()));
                    } else {
                        let buf = Arc::new(SwapBuffer::new(Arc::new(prev.clone())));
                        act.insert(component_id.to_string(), buf);
                    }
                }
                let _ = self.persist_index().await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Basic health checks for a manifest: ensure CAS contains the referenced artifact
    /// and perform a light wasm header check if the entrypoint looks like a wasm file.
    async fn health_check(&self, manifest: &ComponentManifest) -> Result<()> {
        // Try to parse cas key (strip common prefix)
        let hex = manifest.cas_key.trim();
        let hex = hex.strip_prefix("sha256:").unwrap_or(hex);
        let ck = match CasKey::from_hex(hex) {
            Ok(k) => k,
            Err(_) => anyhow::bail!("invalid cas key format for manifest: {}", manifest.cas_key),
        };

        // Existence check
        if !self.cas.exists(&ck).await.map_err(|e| anyhow::anyhow!(e.to_string()))? {
            anyhow::bail!("artifact cas key not found: {}", manifest.cas_key);
        }

        // If entrypoint is wasm, fetch blob and do magic header check
        if manifest.entrypoint.ends_with(".wasm") {
            if let Some(bytes) = self.cas.get(&ck).await.map_err(|e| anyhow::anyhow!(e.to_string()))? {
                if bytes.len() < 4 || &bytes[0..4] != b"\0asm" {
                    anyhow::bail!("wasm validation failed: not a wasm module for {}", manifest.id);
                }
                // If the crate was compiled with wasmtime-host feature, run a stronger validation
                #[cfg(feature = "wasmtime-host")]
                {
                    // spawn a blocking validation to avoid blocking async runtime
                    let bytes_clone = bytes.clone();
                    let valid = tokio::task::spawn_blocking(move || -> anyhow::Result<bool> {
                        use wasmtime::*;
                        use wasmtime_wasi::sync::WasiCtxBuilder;
                        let mut config = Config::new();
                        config.epoch_interruption(true);
                        let engine = Engine::new(&config)?;
                        let module = Module::from_binary(&engine, &bytes_clone)?;
                        // Try to instantiate briefly
                        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
                        let mut store = Store::new(&engine, wasi);
                        let mut linker = Linker::new(&engine);
                        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut _| cx)?;
                        let _inst = linker.instantiate(&mut store, &module)?;
                        Ok(true)
                    })
                    .await
                    .map_err(|e| anyhow::anyhow!(e.to_string()))??;

                    if !valid {
                        anyhow::bail!("wasmtime validation failed for {}", manifest.id);
                    }
                }
            } else {
                anyhow::bail!("wasm artifact missing for {}", manifest.id);
            }
        }
        Ok(())
    }

    /// Perform a lightweight canary run of the manifest's wasm artifact.
    /// Returns true if the canary appears healthy.
    async fn canary_run(&self, manifest: &ComponentManifest) -> Result<bool> {
        let hex = manifest.cas_key.trim();
        let hex = hex.strip_prefix("sha256:").unwrap_or(hex);
        let ck = CasKey::from_hex(hex).map_err(|_| anyhow::anyhow!("invalid cas key"))?;
        let data = match self.cas.get(&ck).await? {
            Some(b) => b,
            None => anyhow::bail!("artifact missing for canary"),
        };

        // If wasmtime-host feature is enabled, instantiate module in-process with a short timeout
        #[cfg(feature = "wasmtime-host")]
        {
            let bytes = data.clone();
            let fut = tokio::task::spawn_blocking(move || -> anyhow::Result<bool> {
                use wasmtime::*;
                use wasmtime_wasi::sync::WasiCtxBuilder;
                let mut config = Config::new();
                config.consume_fuel(true);
                config.epoch_interruption(true);
                let engine = Engine::new(&config)?;
                let module = Module::from_binary(&engine, &bytes)?;
                let wasi = WasiCtxBuilder::new().inherit_stdio().build();
                let mut store = Store::new(&engine, wasi);
                let mut linker = Linker::new(&engine);
                wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut _| cx)?;
                let inst = linker.instantiate(&mut store, &module)?;
                // Call _start if present
                if let Some(start) = inst.get_func(&mut store, "_start") {
                    let typed = start.typed::<(), ()>(&mut store)?;
                    let _ = typed.call(&mut store, ())?;
                }
                // If module exports a health-check function, call it and expect 0==ok
                if let Some(health) = inst.get_func(&mut store, "bonsai_health_check").or_else(|| inst.get_func(&mut store, "_bonsai_health_check")) {
                    if let Ok(typed_h) = health.typed::<(), i32>(&mut store) {
                        let code = typed_h.call(&mut store, ())?;
                        if code != 0 {
                            anyhow::bail!("health check returned error code {}", code);
                        }
                    }
                }
                Ok(true)
            });

            match timeout(Duration::from_secs(4), fut).await {
                Ok(Ok(Ok(true))) => return Ok(true),
                Ok(Ok(Err(e))) => {
                    tracing::warn!(err = %e, "canary in-process failed");
                    return Ok(false);
                }
                _ => return Ok(false),
            }
        }

        #[cfg(not(feature = "wasmtime-host"))]
        {
            // Fallback: write to temp file and run wasmtime CLI for a short timeout
            let tmp = env::temp_dir().join(format!("bonsai_ui_canary_{}.wasm", uuid::Uuid::new_v4()));
            tokio_fs::write(&tmp, &data).await?;
            let cmd = tokio::process::Command::new("wasmtime").arg(tmp.as_os_str()).spawn();
            match cmd {
                Ok(mut child) => {
                    match timeout(Duration::from_secs(4), child.wait()).await {
                        Ok(Ok(status)) => Ok(status.success()),
                        _ => Ok(false),
                    }
                }
                Err(_) => Ok(false),
            }
        }
    }

    /// Activate (spawn) a runtime for a manifest using `bonsai-runtime`.
    /// If `timeout_secs` is Some(n) it's used as the runtime watchdog timeout.
    pub async fn activate_manifest(&self, manifest: &ComponentManifest, timeout_secs: Option<u64>) -> Result<()> {
        let hex = manifest.cas_key.trim();
        let hex = hex.strip_prefix("sha256:").unwrap_or(hex);
        let ck = CasKey::from_hex(hex).map_err(|_| anyhow::anyhow!("invalid cas key"))?;
        let data = match self.cas.get(&ck).await? {
            Some(b) => b,
            None => anyhow::bail!("artifact missing for activation"),
        };

        let tmp = env::temp_dir().join(format!("bonsai_ui_runtime_{}_{}.wasm", manifest.id, uuid::Uuid::new_v4()));
        tokio_fs::write(&tmp, &data).await?;

        // Spawn runtime via RuntimeManager; it will choose in-process wasmtime when available or CLI fallback.
        let mgr = RuntimeManager::new();
        let controller = mgr.start_clojurewasm_worker(tmp.to_string_lossy().as_ref(), timeout_secs).await?;

        let mut rmap = self.runtimes.lock().await;
        rmap.insert(manifest.id.clone(), controller);
        Ok(())
    }

    /// Stop and remove a running manifest runtime if present.
    pub async fn stop_runtime(&self, component_id: &str) -> Result<()> {
        let mut rmap = self.runtimes.lock().await;
        if let Some(mut c) = rmap.remove(component_id) {
            // Best-effort kill + wait
            let _ = c.kill().await;
            let _ = c.wait().await;
        }
        Ok(())
    }

    /// Start a staged rollout: deploy `canary` and gradually increase traffic
    /// percentage from 0 to `target_percent` over `duration_secs` in steps of
    /// `step_secs`. If `target_percent == 100`, the canary is promoted at the end.
    pub async fn start_staged_rollout(self: Arc<Self>, component_id: &str, canary: ComponentManifest, target_percent: u8, duration_secs: u64, step_secs: u64) -> Result<()> {
        if target_percent > 100 {
            anyhow::bail!("target_percent must be <= 100");
        }

        // Validate canary artifact
        self.health_check(&canary).await?;

        // Abort existing rollout for this component if present
        let _ = self.abort_rollout(component_id).await;

        // Insert canary into registry and canaries map
        {
            let mut reg = self.registry.lock().await;
            let entry = reg.entry(component_id.to_string()).or_default();
            entry.push(canary.clone());
        }
        self.canaries.lock().await.insert(component_id.to_string(), canary.clone());

        // Activate canary runtime
        let _ = self.activate_manifest(&canary, Some(10)).await;

        // Initialize weight to 0
        self.routing_weights.lock().await.insert(component_id.to_string(), 0u8);

        // Spawn rollout task
        let this = Arc::clone(&self);
        let comp = component_id.to_string();
        let canary_clone = canary.clone();
        let ticks = if duration_secs == 0 || step_secs == 0 { 1 } else { ((duration_secs + step_secs - 1) / step_secs) as u32 };
        let step_secs = if step_secs == 0 { 1 } else { step_secs };

        let handle = tokio::spawn(async move {
            for i in 1..=ticks {
                // compute fractional weight
                let frac = (i as f64) / (ticks as f64);
                let w = ((target_percent as f64) * frac).round() as u8;
                {
                    let mut weights = this.routing_weights.lock().await;
                    weights.insert(comp.clone(), w);
                }
                tokio::time::sleep(Duration::from_secs(step_secs)).await;
            }

            // Finalize rollout
            if target_percent >= 100 {
                // Promote canary to active by performing hot_reload
                if let Err(e) = this.hot_reload(canary_clone.clone()).await {
                    tracing::error!(err = %e, "promotion failed for {}", comp);
                } else {
                    // Clear canary state
                    let mut c = this.canaries.lock().await;
                    c.remove(&comp);
                    let mut w = this.routing_weights.lock().await;
                    w.remove(&comp);
                }
            }

            // Remove task handle entry
            let mut tasks = this.rollout_tasks.lock().await;
            tasks.remove(&comp);
        });

        self.rollout_tasks.lock().await.insert(component_id.to_string(), handle);
        let _ = self.persist_index().await;
        Ok(())
    }

    /// Abort an in-progress staged rollout (if any) and remove canary state.
    pub async fn abort_rollout(&self, component_id: &str) -> Result<()> {
        // Abort background task
        if let Some(h) = self.rollout_tasks.lock().await.remove(component_id) {
            h.abort();
        }

        // Stop any canary runtime
        let _ = self.stop_runtime(component_id).await;

        // Clear canary and routing state
        self.canaries.lock().await.remove(component_id);
        self.routing_weights.lock().await.remove(component_id);
        let _ = self.persist_index().await;
        Ok(())
    }

    /// Select a manifest for the next incoming request using routing weights.
    /// Returns an Arc to the chosen `ComponentManifest` (canary or active).
    pub async fn select_manifest_for_request(&self, component_id: &str) -> Option<Arc<ComponentManifest>> {
        let weight = { self.routing_weights.lock().await.get(component_id).copied().unwrap_or(0) };
        if weight > 0 {
            if let Some(canary) = self.canaries.lock().await.get(component_id) {
                let mut rng = rand::thread_rng();
                let roll: u8 = rng.gen_range(0..100);
                if roll < weight {
                    return Some(Arc::new(canary.clone()));
                }
            }
        }

        // fall back to the active swap buffer
        if let Some(buf) = self.active.lock().await.get(component_id) {
            return Some(buf.load());
        }

        // fallback: return last registry entry if present
        if let Some(list) = self.registry.lock().await.get(component_id) {
            if let Some(last) = list.last() {
                return Some(Arc::new(last.clone()));
            }
        }
        None
    }

    /// Persist the in-memory registry to disk and to CAS, return the CAS key of the index.
    async fn persist_index(&self) -> Result<CasKey> {
        let reg = self.registry.lock().await;
        let bytes = serde_json::to_vec(&*reg)?;

        // Ensure parent dir exists
        if let Some(parent) = self.index_path.parent() {
            let _ = tokio_fs::create_dir_all(parent).await;
        }
        tokio_fs::write(&self.index_path, &bytes).await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        // Also store index in CAS
        let key = self.cas.put(&bytes, "application/vnd.bonsai.ui-registry+json").await?;
        let _ = self.cas.pin(&key).await;
        info!(path = %self.index_path.display(), index_key = %key.hex(), "persisted UI registry");
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    async fn open_temp_cas() -> bonsai_cas::CasStore {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("cas.db");
        let blob_dir = dir.path().join("blobs");
        // leak so it lives for test duration
        std::mem::forget(dir);
        bonsai_cas::CasStore::open(&db_path, &blob_dir).await.unwrap()
    }

    #[tokio::test]
    async fn gen_ui() {
        let cas = Arc::new(open_temp_cas().await);
        let o = UIOrchestrator::new(cas, None);
        let m = o.generate_ui("Show me a transfer dashboard").await.unwrap();
        assert!(m.id.starts_with("generated."));
    }
}
