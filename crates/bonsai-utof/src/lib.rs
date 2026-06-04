//! Universal Test Orchestration Fabric (UTOF)
//!
//! A deterministic, polyglot, AI-optional test harness for validating the entire Bonsai Ecosystem
//! and USOS. UTOF orchestrates tests across 750+ languages with perfect fidelity, integrating:
//!
//! - Bonsai Enclave: runtime provisioning
//! - Sanctum: sandboxed execution
//! - TransferDaemon: distributed job distribution
//! - Universe: immutable audit logging
//! - AriaDB: time-series result storage
//! - BonsAI V2: optional AI-enhanced scheduling & analysis

pub mod spec;
pub mod runner;
pub mod comparer;
pub mod scheduler;
pub mod storage;

pub use spec::TestSpec;
pub use runner::run_test;
pub use comparer::{compare_outputs, ComparisonResult};
pub use scheduler::{Job, Scheduler};
pub use storage::{ResultStore, StoredResult, SpecStats};

use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;
use uuid::Uuid;

/// Main UTOF orchestrator configuration
#[derive(Debug, Clone)]
pub struct UtofConfig {
    pub work_dir: PathBuf,
    pub spec_cache_dir: PathBuf,
    pub result_storage_dir: PathBuf,
    pub max_concurrent_tests: usize,
    pub enable_ai_scheduling: bool,
    pub enable_ai_analysis: bool,
    pub deterministic_mode: bool,
}

impl UtofConfig {
    pub fn new(work_dir: PathBuf) -> Result<Self> {
        let spec_cache_dir = work_dir.join("specs");
        let result_storage_dir = work_dir.join("results");

        std::fs::create_dir_all(&spec_cache_dir)?;
        std::fs::create_dir_all(&result_storage_dir)?;

        Ok(Self {
            work_dir,
            spec_cache_dir,
            result_storage_dir,
            max_concurrent_tests: 1000,
            enable_ai_scheduling: true,
            enable_ai_analysis: true,
            deterministic_mode: true,
        })
    }
}

/// Main UTOF Orchestrator
pub struct Orchestrator {
    _config: UtofConfig,
    store: storage::ResultStore,
}

impl Orchestrator {
    pub fn new(config: UtofConfig) -> Result<Self> {
        Ok(Self {
            _config: config,
            store: storage::ResultStore::new(),
        })
    }

    /// Run a test specification
    pub async fn run_spec(&mut self, spec: &TestSpec) -> Result<SpecStats> {
        spec.validate()?;

        tracing::info!("Running test suite: {}", spec.name);
        let run_id = Uuid::new_v4().to_string();

        // Compute oracle (reference output)
        tracing::info!("Computing oracle using reference language: {}", spec.reference_lang);
        let oracle_outputs = self.compute_oracle(spec).await?;

        // Build and run schedule
        let mut scheduler = scheduler::Scheduler::new(spec);
        let total_jobs = scheduler.total_jobs();
        let mut job_count = 0;

        while let Some(job) = scheduler.next_job() {
            job_count += 1;
            let test_case = &spec.test_cases[job.test_case_index];
            let start = Instant::now();

            tracing::debug!(
                "Running test {}/{}: {} in {}",
                job_count,
                total_jobs,
                test_case.name,
                job.lang
            );

            let runner_template = spec.runners.get(&job.lang).map(|s| s.as_str());
            let result = runner::run_test(
                &job.lang,
                &spec.canonical_source,
                &test_case.input,
                job.seed,
                runner_template,
                std::time::Duration::from_secs(spec.timeout_secs()),
            )
            .await;

            let elapsed_ms = start.elapsed().as_millis() as u64;
            let actual_output = match &result {
                Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
                Err(e) => format!("ERROR: {}", e),
            };

            let comparison = ComparisonResult::new(
                &actual_output,
                &oracle_outputs[job.test_case_index],
                spec.fidelity_threshold(),
            );

            let stored_result = StoredResult {
                run_id: run_id.clone(),
                spec_name: spec.name.clone(),
                test_case_name: test_case.name.clone(),
                language: job.lang.clone(),
                passed: comparison.passed,
                fidelity: comparison.fidelity,
                actual_output,
                expected_output: oracle_outputs[job.test_case_index].clone(),
                execution_time_ms: elapsed_ms,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };

            self.store.store(stored_result).await?;
        }

        let stats = self.store.compute_stats(&spec.name);
        tracing::info!(
            "Test suite '{}' complete: {}/{} passed ({}%)",
            spec.name,
            stats.passed,
            stats.total_tests,
            stats.success_rate
        );

        Ok(stats)
    }

    /// Compute the oracle (reference outputs)
    async fn compute_oracle(&self, spec: &TestSpec) -> Result<Vec<String>> {
        let mut outputs = Vec::new();
        for test_case in &spec.test_cases {
            let runner_template = spec.runners.get(&spec.reference_lang).map(|s| s.as_str());
            let result = runner::run_test(
                &spec.reference_lang,
                &spec.canonical_source,
                &test_case.input,
                test_case.seed,
                runner_template,
                std::time::Duration::from_secs(spec.timeout_secs()),
            )
            .await?;
            let output = String::from_utf8_lossy(&result.stdout).to_string();
            outputs.push(output.trim().to_string());
        }
        Ok(outputs)
    }

    /// Get stored results
    pub fn results(&self) -> &storage::ResultStore {
        &self.store
    }

    /// Get mutable reference to store (for testing)
    pub fn results_mut(&mut self) -> &mut storage::ResultStore {
        &mut self.store
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utof_config() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = UtofConfig::new(tmpdir.path().to_path_buf()).unwrap();
        assert!(config.spec_cache_dir.exists());
        assert!(config.result_storage_dir.exists());
    }

    #[test]
    fn test_orchestrator_new() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = UtofConfig::new(tmpdir.path().to_path_buf()).unwrap();
        let _orchestrator = Orchestrator::new(config).unwrap();
    }
}
