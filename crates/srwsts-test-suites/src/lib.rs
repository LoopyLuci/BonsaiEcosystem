//! SRWSTS Test Suites
//!
//! Comprehensive modular test suites for validating all layers of the Bonsai Ecosystem:
//!
//! - **KernelTestSuite**: Scheduler, memory management, IPC, driver tests
//! - **ServiceTestSuite**: P2P, storage, network, compositor services
//! - **LanguageTestSuite**: Titan, Sylva, Aether, Axiom verification tests
//! - **ApplicationTestSuite**: Workspace, Buddy, Omni-Bot application tests
//! - **HardwareEquivalenceSuite**: x86_64, ARM, RISC-V equivalence validation
//! - **FullStackTestSuite**: Integrated end-to-end system tests
//! - **TestSuiteRegistry**: Enumerate and load available test suites

pub mod registry;
pub mod suites {
    pub mod kernel;
    pub mod service;
    pub mod language;
    pub mod application;
    pub mod hardware;
    pub mod fullstack;
}
pub mod test_plan;
pub mod result;
pub mod executor;
pub mod coordinator;

pub use registry::{TestSuiteRegistry, SuiteId, SuiteMetadata};
pub use test_plan::{TestPlan, TestPlanGenerator};
pub use result::{SuiteResult, SuiteResultCollector};
pub use executor::{SuiteExecutor, ExecutionContext};
pub use coordinator::SuiteCoordinator;

pub use srwsts_core::{SrwstsError, SrwstsResult, TestId, RunId};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared suite state for concurrent execution
pub type SharedSuiteState<T> = Arc<RwLock<T>>;

/// Create a new shared suite state container
pub fn shared_state<T>(value: T) -> SharedSuiteState<T> {
    Arc::new(RwLock::new(value))
}

/// Configuration for test suite execution
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuiteConfig {
    /// Maximum concurrent tests per suite
    pub max_concurrent: usize,
    /// Timeout per test in seconds
    pub test_timeout: u64,
    /// Enable verbose output
    pub verbose: bool,
    /// Store detailed test artifacts
    pub store_artifacts: bool,
    /// Artifact storage directory
    pub artifact_dir: std::path::PathBuf,
    /// Enable performance profiling
    pub profile_performance: bool,
    /// Enable fault injection
    pub enable_fault_injection: bool,
}

impl Default for SuiteConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 8,
            test_timeout: 300,
            verbose: false,
            store_artifacts: true,
            artifact_dir: std::path::PathBuf::from("./test-artifacts"),
            profile_performance: false,
            enable_fault_injection: false,
        }
    }
}

/// Test suite execution environment
pub struct SuiteEnvironment {
    pub config: SuiteConfig,
    pub registry: Arc<TestSuiteRegistry>,
    pub state: SharedSuiteState<ExecutionState>,
}

/// Current execution state across all suites
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecutionState {
    pub total_suites: usize,
    pub completed_suites: usize,
    pub failed_suites: usize,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl SuiteEnvironment {
    /// Create a new test suite environment
    pub async fn new(config: SuiteConfig) -> SrwstsResult<Self> {
        // Create artifact directory if needed
        if config.store_artifacts {
            std::fs::create_dir_all(&config.artifact_dir)
                .map_err(|e| SrwstsError::DirectoryCreationFailed {
                    path: config.artifact_dir.to_string_lossy().to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(Self {
            config,
            registry: Arc::new(TestSuiteRegistry::new()),
            state: shared_state(ExecutionState::default()),
        })
    }

    /// Get the test suite registry
    pub fn registry(&self) -> &Arc<TestSuiteRegistry> {
        &self.registry
    }

    /// Get the current execution state
    pub async fn execution_state(&self) -> ExecutionState {
        self.state.read().await.clone()
    }

    /// Update execution state
    pub async fn update_state<F>(&self, f: F) -> SrwstsResult<()>
    where
        F: FnOnce(&mut ExecutionState),
    {
        let mut state = self.state.write().await;
        f(&mut state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_suite_environment_creation() {
        let config = SuiteConfig::default();
        let env = SuiteEnvironment::new(config).await;
        assert!(env.is_ok());
    }

    #[tokio::test]
    async fn test_execution_state_tracking() {
        let config = SuiteConfig::default();
        let env = SuiteEnvironment::new(config).await.unwrap();

        env.update_state(|state| {
            state.total_suites = 5;
            state.total_tests = 100;
        })
        .await
        .unwrap();

        let state = env.execution_state().await;
        assert_eq!(state.total_suites, 5);
        assert_eq!(state.total_tests, 100);
    }
}
