//! SRWSTS Test Harness
//!
//! Provides a complete test execution environment with vault isolation, resource limiting,
//! and comprehensive metrics collection.
//!
//! ## Key Components
//!
//! - **TestExecutor**: Executes test binaries inside Sanctum vaults
//! - **VaultManager**: Manages vault lifecycle (spawn, snapshot, restore, destroy)
//! - **ResourceLimiter**: Enforces memory, CPU, and I/O limits
//! - **ResultCapture**: Collects stdout, stderr, metrics, and logs
//! - **TestRunner**: Orchestrates test execution with emulation and fault injection
//! - **MetricsCollector**: Parses and extracts performance metrics
//! - **TraceRecorder**: Records execution traces for deterministic replay
//!
//! ## Architecture
//!
//! The test harness is organized around test execution workflows:
//! - Preparation: Vault setup, resource allocation, environment initialization
//! - Execution: Test running, metric collection, trace recording
//! - Cleanup: Result archival, trace storage, resource deallocation
//! - Analysis: Metrics aggregation, failure analysis, trend detection

pub mod errors;
pub mod executor;
pub mod metrics;
pub mod result;
pub mod trace;
pub mod vault;
pub mod runner;
pub mod limiter;

pub use errors::{HarnessError, HarnessResult};
pub use executor::TestExecutor;
pub use metrics::{MetricsCollector, PerformanceMetrics};
pub use result::ResultCapture;
pub use trace::TraceRecorder;
pub use vault::{Vault, VaultManager};
pub use runner::TestRunner;
pub use limiter::ResourceLimiter;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Unique identifier for test runs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(Uuid);

impl RunId {
    /// Generate a new random run ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Test harness configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessConfig {
    /// Maximum number of concurrent vaults
    pub max_concurrent_vaults: usize,
    /// Maximum test duration in seconds
    pub test_timeout_secs: u64,
    /// Default memory limit per vault in bytes
    pub default_memory_limit: u64,
    /// Default CPU limit (cores)
    pub default_cpu_limit: f64,
    /// Enable metric collection
    pub enable_metrics: bool,
    /// Enable trace recording
    pub enable_tracing: bool,
    /// Enable fault injection
    pub enable_faults: bool,
    /// Vault snapshot frequency (every N executions)
    pub snapshot_frequency: u32,
}

impl Default for HarnessConfig {
    fn default() -> Self {
        Self {
            max_concurrent_vaults: 64,
            test_timeout_secs: 300,
            default_memory_limit: 512 * 1024 * 1024, // 512 MB
            default_cpu_limit: 4.0,
            enable_metrics: true,
            enable_tracing: true,
            enable_faults: true,
            snapshot_frequency: 1000,
        }
    }
}

/// Test harness session
pub struct HarnessSession {
    /// Configuration
    pub config: Arc<HarnessConfig>,
    /// Vault manager
    vault_manager: Arc<RwLock<VaultManager>>,
    /// Active test runs
    active_runs: Arc<RwLock<std::collections::HashMap<RunId, TestRunState>>>,
    /// Metrics collector
    metrics_collector: Arc<RwLock<MetricsCollector>>,
    /// Session start time
    pub start_time: DateTime<Utc>,
}

/// State of a test run
#[derive(Debug, Clone)]
pub struct TestRunState {
    /// Run ID
    pub run_id: RunId,
    /// Test binary path
    pub binary_path: String,
    /// Vault ID
    pub vault_id: Uuid,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// Status
    pub status: TestStatus,
}

/// Test execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    /// Test is queued
    Queued,
    /// Test is running
    Running,
    /// Test completed successfully
    Passed,
    /// Test failed
    Failed,
    /// Test was skipped
    Skipped,
    /// Test timeout
    Timeout,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "Queued"),
            Self::Running => write!(f, "Running"),
            Self::Passed => write!(f, "Passed"),
            Self::Failed => write!(f, "Failed"),
            Self::Skipped => write!(f, "Skipped"),
            Self::Timeout => write!(f, "Timeout"),
        }
    }
}

impl HarnessSession {
    /// Create a new harness session
    pub async fn new(config: HarnessConfig) -> HarnessResult<Self> {
        let config = Arc::new(config);
        let vault_manager = Arc::new(RwLock::new(VaultManager::new(config.max_concurrent_vaults)));

        Ok(Self {
            config,
            vault_manager,
            active_runs: Arc::new(RwLock::new(std::collections::HashMap::new())),
            metrics_collector: Arc::new(RwLock::new(MetricsCollector::new())),
            start_time: Utc::now(),
        })
    }

    /// Get vault manager
    pub async fn vault_manager(&self) -> Arc<RwLock<VaultManager>> {
        self.vault_manager.clone()
    }

    /// Get metrics collector
    pub async fn metrics_collector(&self) -> Arc<RwLock<MetricsCollector>> {
        self.metrics_collector.clone()
    }

    /// Register a test run
    pub async fn register_run(&self, run_id: RunId, state: TestRunState) -> HarnessResult<()> {
        let mut runs = self.active_runs.write().await;
        runs.insert(run_id, state);
        tracing::info!("Registered test run: {}", run_id);
        Ok(())
    }

    /// Unregister a test run
    pub async fn unregister_run(&self, run_id: RunId) -> HarnessResult<Option<TestRunState>> {
        let mut runs = self.active_runs.write().await;
        Ok(runs.remove(&run_id))
    }

    /// Get test run state
    pub async fn get_run(&self, run_id: RunId) -> HarnessResult<Option<TestRunState>> {
        let runs = self.active_runs.read().await;
        Ok(runs.get(&run_id).cloned())
    }

    /// Update test run status
    pub async fn update_run_status(&self, run_id: RunId, status: TestStatus) -> HarnessResult<()> {
        let mut runs = self.active_runs.write().await;
        if let Some(run) = runs.get_mut(&run_id) {
            run.status = status;
            tracing::debug!("Updated run {} status to {}", run_id, status);
        }
        Ok(())
    }

    /// Get all active runs
    pub async fn active_runs(&self) -> HarnessResult<Vec<TestRunState>> {
        let runs = self.active_runs.read().await;
        Ok(runs.values().cloned().collect())
    }

    /// Shutdown the harness session
    pub async fn shutdown(&self) -> HarnessResult<()> {
        let mut vault_manager = self.vault_manager.write().await;
        vault_manager.shutdown().await?;

        let metrics = self.metrics_collector.read().await;
        tracing::info!(
            "Harness session complete. Total tests: {}",
            metrics.total_tests()
        );

        Ok(())
    }

    /// Get session statistics
    pub async fn statistics(&self) -> HarnessResult<SessionStatistics> {
        let metrics = self.metrics_collector.read().await;
        let runs = self.active_runs.read().await;

        Ok(SessionStatistics {
            total_runs: runs.len(),
            total_tests: metrics.total_tests(),
            passed_tests: metrics.passed_tests(),
            failed_tests: metrics.failed_tests(),
            session_duration: Utc::now().signed_duration_since(self.start_time),
        })
    }
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    /// Total active runs
    pub total_runs: usize,
    /// Total tests executed
    pub total_tests: u64,
    /// Passed tests
    pub passed_tests: u64,
    /// Failed tests
    pub failed_tests: u64,
    /// Session duration
    pub session_duration: chrono::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_id_generation() {
        let id1 = RunId::new();
        let id2 = RunId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_test_status_display() {
        assert_eq!(TestStatus::Passed.to_string(), "Passed");
        assert_eq!(TestStatus::Failed.to_string(), "Failed");
        assert_eq!(TestStatus::Timeout.to_string(), "Timeout");
    }

    #[tokio::test]
    async fn test_harness_session_creation() {
        let config = HarnessConfig::default();
        let session = HarnessSession::new(config).await.unwrap();
        assert_eq!(session.config.max_concurrent_vaults, 64);
    }

    #[tokio::test]
    async fn test_harness_session_register_run() {
        let config = HarnessConfig::default();
        let session = HarnessSession::new(config).await.unwrap();
        let run_id = RunId::new();

        let state = TestRunState {
            run_id,
            binary_path: "/path/to/test".to_string(),
            vault_id: Uuid::new_v4(),
            start_time: Utc::now(),
            status: TestStatus::Running,
        };

        session.register_run(run_id, state).await.unwrap();

        let retrieved = session.get_run(run_id).await.unwrap();
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_harness_session_statistics() {
        let config = HarnessConfig::default();
        let session = HarnessSession::new(config).await.unwrap();

        let stats = session.statistics().await.unwrap();
        assert_eq!(stats.total_runs, 0);
    }
}
