use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;

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
}

impl BonsaiCi {
    pub fn new(config: CiConfig) -> Self {
        Self { config }
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
                results.push(self.sign(&ws).await);
            }
        }
        let fe_dir = format!("{}/bonsai-workspace", ws);
        if self.config.run_npm_build {
            results.push(Self::stage("npm", &["run", "build"], &fe_dir).await);
        }
        if self.config.run_svelte_check {
            results.push(Self::stage("npx", &["svelte-check", "--tsconfig", "./tsconfig.json"], &fe_dir).await);
        }
        results
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
