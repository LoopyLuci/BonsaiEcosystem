use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;
use std::path::Path;
use std::sync::Arc;
use bonsai_cas::CasStore;
use bonsai_transfer_crypto::BonsaiIdentity;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;
use std::ffi::OsStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageDef {
    /// A simple shell command (first element is executable, rest are args).
    pub cmd: Vec<String>,
    /// Optional working directory for the stage.
    pub cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineDef {
    pub id: Option<String>,
    pub stages: Vec<StageDef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
    pub status: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// Run a single job (command vector) and capture stdout/stderr.
pub async fn run_job(cmd: Vec<String>, cwd: Option<PathBuf>) -> Result<RunResult> {
    if cmd.is_empty() {
        return Ok(RunResult {
            status: "empty".to_string(),
            exit_code: None,
            stdout: String::new(),
            stderr: String::new(),
        });
    }

    let mut command = Command::new(&cmd[0]);
    if cmd.len() > 1 {
        command.args(&cmd[1..]);
    }
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }

    let out = command.output().await?;

    Ok(RunResult {
        status: if out.status.success() {
            "ok".to_string()
        } else {
            "failed".to_string()
        },
        exit_code: out.status.code(),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    })
}

/// A tiny orchestrator struct for Phase 1. For now it's a light-weight holder
/// around spawn/execute helpers. In later phases this will be replaced with
/// a full actor-based supervisor.
#[derive(Debug, Default)]
pub struct OrchestratorActor {}

impl OrchestratorActor {
    pub fn new() -> Self {
        Self {}
    }

    /// Submit a pipeline and run only the first stage (Phase 1).
    pub async fn submit_pipeline(&self, pipeline: PipelineDef) -> Result<RunResult> {
        let first = match pipeline.stages.into_iter().next() {
            Some(s) => s,
            None => {
                return Ok(RunResult {
                    status: "no-stages".to_string(),
                    exit_code: None,
                    stdout: String::new(),
                    stderr: String::new(),
                })
            }
        };

        let cwd = first.cwd.map(PathBuf::from);
        run_job(first.cmd, cwd).await
    }
}

// ── BonsaiCi: full pipeline runner ───────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CiStageResult {
    pub stage: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub artifact_hash: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CiConfig {
    pub workspace_root: PathBuf,
    pub run_check: bool,
    pub run_clippy: bool,
    pub run_test: bool,
    pub run_build: bool,
    pub sign_artifacts: bool,
    pub run_npm_build: bool,
    pub run_svelte_check: bool,
}

impl Default for CiConfig {
    fn default() -> Self {
        Self {
            workspace_root: PathBuf::from("."),
            run_check: true,
            run_clippy: false,
            run_test: true,
            run_build: true,
            sign_artifacts: true,
            run_npm_build: false,
            run_svelte_check: false,
        }
    }
}

pub struct BonsaiCi {
    pub config: CiConfig,
    pub cas: Option<Arc<CasStore>>,
    pub identity: Option<Arc<BonsaiIdentity>>,
}

impl BonsaiCi {
    pub fn new(config: CiConfig) -> Self {
        Self { config, cas: None, identity: None }
    }

    pub fn with_cas_and_identity(config: CiConfig, cas: Arc<CasStore>, identity: Option<Arc<BonsaiIdentity>>) -> Self {
        Self { config, cas: Some(cas), identity }
    }

    pub async fn run_full_pipeline(&self) -> Vec<CiStageResult> {
        let mut results = Vec::new();
        let ws = self.config.workspace_root.to_string_lossy().to_string();

        if self.config.run_check {
            results.push(Self::stage("cargo", &["check", "--workspace", "--all-targets"], &ws).await);
            if !results.last().unwrap().success { return results; }
        }
        if self.config.run_clippy {
            results.push(Self::stage("cargo", &["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"], &ws).await);
        }
        if self.config.run_test {
            results.push(Self::stage("cargo", &["test", "--workspace"], &ws).await);
        }
        if self.config.run_build {
            let build = Self::stage("cargo", &["build", "--release", "--workspace"], &ws).await;
            let build_ok = build.success;
            results.push(build);
            if build_ok && self.config.sign_artifacts {
                let sign_res = self.sign(&ws).await;
                results.push(sign_res.clone());
                // If CAS configured, upload the built binary and attach signature metadata
                if let Some(cas) = &self.cas {
                    // Attempt to upload main workspace binary if present
                    let candidate = Path::new(&ws).join("target/release/bonsai-workspace");
                    if candidate.exists() {
                        match self.upload_and_sign_artifact(cas.clone(), &candidate).await {
                            Ok(Some(meta)) => {
                                results.push(CiStageResult {
                                    stage: "upload-artifact-to-cas".into(),
                                    success: true,
                                    stdout: format!("meta_key: {}", meta),
                                    stderr: String::new(),
                                    artifact_hash: None,
                                    duration_ms: 0,
                                });
                            }
                            Ok(None) => {
                                results.push(CiStageResult {
                                    stage: "upload-artifact-to-cas".into(),
                                    success: false,
                                    stdout: String::new(),
                                    stderr: "no artifact produced".into(),
                                    artifact_hash: None,
                                    duration_ms: 0,
                                });
                            }
                            Err(e) => {
                                results.push(CiStageResult {
                                    stage: "upload-artifact-to-cas".into(),
                                    success: false,
                                    stdout: String::new(),
                                    stderr: e.to_string(),
                                    artifact_hash: None,
                                    duration_ms: 0,
                                });
                            }
                        }
                    }
                }
            }
        }
        let fe_dir = format!("{}/bonsai-workspace", ws);
        if self.config.run_npm_build {
            // Run frontend builds (bonsai-workspace and discovered package.json dirs)
            let mut fe_results = self.run_frontend_builds(&ws).await;
            results.append(&mut fe_results);
        }
        if self.config.run_svelte_check {
            results.push(Self::stage("npx", &["svelte-check", "--tsconfig", "./tsconfig.json"], &fe_dir).await);
        }
        results
    }

    /// Run frontend builds discovered in the workspace. Returns stage results.
    async fn run_frontend_builds(&self, ws: &str) -> Vec<CiStageResult> {
        let mut results = Vec::new();
        let fe_dir = Path::new(ws).join("bonsai-workspace");
        if fe_dir.exists() {
            results.push(Self::stage("npm", &["ci"], fe_dir.to_string_lossy().as_ref()).await);
            results.push(Self::stage("npm", &["run", "build"], fe_dir.to_string_lossy().as_ref()).await);
        }

        // Also discover package.json files across shortlist of folders and run builds
        let walker = WalkDir::new(ws).max_depth(3).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_type().is_file() && entry.file_name() == "package.json" {
                let dir = entry.path().parent().unwrap_or_else(|| Path::new(ws));
                // Skip node_modules or hidden
                if dir.components().any(|c| c.as_os_str() == OsStr::new("node_modules")) {
                    continue;
                }
                let dir_str = dir.to_string_lossy().to_string();
                results.push(Self::stage("npm", &["ci"], &dir_str).await);
                results.push(Self::stage("npm", &["run", "build"], &dir_str).await);
            }
        }

        // After builds, discover artifacts and upload to CAS if configured
        if let Some(cas) = &self.cas {
            let artifacts = self.discover_artifacts().await;
            for p in artifacts {
                match self.upload_and_sign_artifact(cas.clone(), &p).await {
                    Ok(Some(meta)) => results.push(CiStageResult { stage: format!("upload:{}", p.display()), success: true, stdout: meta, stderr: String::new(), artifact_hash: None, duration_ms: 0 }),
                    Ok(None) => results.push(CiStageResult { stage: format!("upload:{}", p.display()), success: false, stdout: String::new(), stderr: "no artifact".into(), artifact_hash: None, duration_ms: 0 }),
                    Err(e) => results.push(CiStageResult { stage: format!("upload:{}", p.display()), success: false, stdout: String::new(), stderr: e.to_string(), artifact_hash: None, duration_ms: 0 }),
                }
            }
        }

        results
    }

    /// Discover candidate artifacts (wasm, frontend builds) under the workspace.
    async fn discover_artifacts(&self) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let root = &self.config.workspace_root;
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() { continue; }
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                if ext.eq_ignore_ascii_case("wasm") {
                    out.push(path.to_path_buf());
                }
            }
            // also collect common build outputs
            if path.ends_with("index.html") || path.ends_with("bundle.js") {
                out.push(path.to_path_buf());
            }
        }
        out
    }

    pub fn all_passed(results: &[CiStageResult]) -> bool {
        results.iter().all(|r| r.success)
    }

    pub fn failure_summary(results: &[CiStageResult]) -> String {
        results.iter()
            .filter(|r| !r.success)
            .map(|r| format!("[{}] {}", r.stage, r.stderr))
            .collect::<Vec<_>>()
            .join("\n---\n")
    }

    async fn stage(cmd: &str, args: &[&str], cwd: &str) -> CiStageResult {
        use tokio::process::Command;
        let start = std::time::Instant::now();
        match Command::new(cmd).args(args).current_dir(cwd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output().await
        {
            Ok(out) => CiStageResult {
                stage: format!("{cmd} {}", args.join(" ")),
                success: out.status.success(),
                stdout: String::from_utf8_lossy(&out.stdout).into(),
                stderr: String::from_utf8_lossy(&out.stderr).into(),
                artifact_hash: None,
                duration_ms: start.elapsed().as_millis() as u64,
            },
            Err(e) => CiStageResult {
                stage: format!("{cmd} {}", args.join(" ")),
                success: false,
                stdout: String::new(),
                stderr: e.to_string(),
                artifact_hash: None,
                duration_ms: start.elapsed().as_millis() as u64,
            },
        }
    }

    async fn sign(&self, workspace_root: &str) -> CiStageResult {
        let start = std::time::Instant::now();
        let binary = format!("{}/target/release/bonsai-workspace", workspace_root);
        match tokio::fs::read(&binary).await {
            Ok(data) => {
                let hash = blake3::hash(&data).to_hex().to_string();
                CiStageResult {
                    stage: "sign-artifacts".into(),
                    success: true,
                    stdout: format!("BLAKE3: {}", hash),
                    stderr: String::new(),
                    artifact_hash: Some(hash),
                    duration_ms: start.elapsed().as_millis() as u64,
                }
            }
            Err(e) => CiStageResult {
                stage: "sign-artifacts".into(),
                success: false,
                stdout: String::new(),
                stderr: e.to_string(),
                artifact_hash: None,
                duration_ms: start.elapsed().as_millis() as u64,
            },
        }
    }

    /// Upload artifact bytes to CAS, optionally sign using self.identity and upload metadata.
    pub async fn upload_and_sign_artifact(&self, cas: Arc<CasStore>, path: &Path) -> Result<Option<String>> {
        if !path.exists() {
            return Ok(None);
        }
        let data = tokio::fs::read(path).await?;
        let mime = if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext {
                "wasm" => "application/wasm",
                _ => "application/octet-stream",
            }
        } else {
            "application/octet-stream"
        };

        let blob_key = cas.put(&data, mime).await?;
        let _ = cas.pin(&blob_key).await;

        // Compute hash and optional signature
        let hash = blake3::hash(&data).to_hex().to_string();
        let mut signature: Option<String> = None;
        let mut signer_pub: Option<String> = None;
        if let Some(id) = &self.identity {
            let sig = id.sign(&data);
            signature = Some(hex::encode(sig));
            signer_pub = Some(id.public_key.to_hex());
        }

        let meta = serde_json::json!({
            "artifact_name": path.file_name().and_then(|s| s.to_str()).unwrap_or("unknown"),
            "blob_key": blob_key.hex(),
            "blake3": hash,
            "signature": signature,
            "signer": signer_pub,
        });
        let meta_bytes = serde_json::to_vec_pretty(&meta)?;
        let meta_key = cas.put(&meta_bytes, "application/vnd.bonsai.artifact-meta+json").await?;
        let _ = cas.pin(&meta_key).await;
        Ok(Some(meta_key.hex()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn run_echo() {
        let orch = OrchestratorActor::new();
        #[cfg(windows)]
        let cmd = vec![
            "cmd".to_string(),
            "/C".to_string(),
            "echo".to_string(),
            "hello".to_string(),
        ];
        #[cfg(not(windows))]
        let cmd = vec!["echo".to_string(), "hello".to_string()];
        let pipeline = PipelineDef {
            id: Some("t1".to_string()),
            stages: vec![StageDef { cmd, cwd: None }],
        };
        let r = orch.submit_pipeline(pipeline).await.unwrap();
        assert!(r.stdout.contains("hello"));
    }
}
