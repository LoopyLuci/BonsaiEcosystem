//! SRWSTS Applications - Comprehensive Stress Testing Suite
//!
//! This crate provides intensive stress testing for Bonsai Ecosystem applications
//! (Workspace, Buddy, Omni-Bot) within the full Omnisystem environment.
//!
//! ## Architecture
//!
//! - **Bootstrap**: Load complete Bonsai Ecosystem image on top of Omnisystem
//! - **WorkspaceStressTests**: File handling, compilation, multi-user collaboration
//! - **BuddyStressTests**: Offline-online transitions, CRDT merging, large file sync
//! - **OmniBotStressTests**: Chat sessions, NLP parsing, task execution
//! - **InteractionTests**: Cross-application data flow and dependency handling
//! - **FaultScenarios**: System failures, crashes, network loss, storage corruption
//! - **Metrics**: Performance, memory, latency, and UI responsiveness tracking
//! - **InputSimulation**: Deterministic input replay for reproducible testing

pub mod bootstrap;
pub mod errors;
pub mod metrics;
pub mod scenarios;
pub mod simulation;
pub mod tests;

pub use bootstrap::{
    ApplicationBootstrap, BootstrapConfig, BootstrapError, BootstrapResult, EcosystemState,
};
pub use errors::{ApplicationStressError, ApplicationStressResult};
pub use metrics::{
    ApplicationMetrics, MemoryProfile, PerformanceMetrics, ResponseTimeMetrics, UIMetrics,
};
pub use scenarios::{
    FaultScenario, FaultScenarioExecutor, InteractionScenario, InteractionScenarioExecutor,
};
pub use simulation::{InputEvent, InputSimulator, UserSimulation};
pub use tests::{
    ApplicationTestRunner, BuddyStressTest, OmniBotStressTest, TestContext, TestResult,
    WorkspaceStressTest,
};

use srwsts_core::{RunId, TestId};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application test state
pub type SharedApplicationState<T> = Arc<RwLock<T>>;

/// Create a new shared application state container
pub fn shared_state<T>(value: T) -> SharedApplicationState<T> {
    Arc::new(RwLock::new(value))
}

/// Configuration for application stress testing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApplicationStressConfig {
    /// Maximum concurrent application instances
    pub max_concurrent_apps: usize,
    /// Maximum concurrent users per application
    pub max_concurrent_users: usize,
    /// Test timeout in seconds
    pub test_timeout_secs: u64,
    /// Enable verbose logging
    pub verbose: bool,
    /// Store detailed artifacts
    pub store_artifacts: bool,
    /// Artifact storage directory
    pub artifact_dir: std::path::PathBuf,
    /// Enable performance profiling
    pub profile_performance: bool,
    /// Enable fault injection
    pub enable_fault_injection: bool,
    /// Memory monitoring interval in milliseconds
    pub memory_monitor_interval_ms: u64,
    /// Deterministic mode for reproducible tests
    pub deterministic: bool,
}

impl Default for ApplicationStressConfig {
    fn default() -> Self {
        Self {
            max_concurrent_apps: 10,
            max_concurrent_users: 100,
            test_timeout_secs: 600,
            verbose: false,
            store_artifacts: true,
            artifact_dir: std::path::PathBuf::from("./test-artifacts"),
            profile_performance: true,
            enable_fault_injection: true,
            memory_monitor_interval_ms: 500,
            deterministic: false,
        }
    }
}

/// Application test execution environment
pub struct ApplicationStressEnvironment {
    pub config: ApplicationStressConfig,
    pub state: SharedApplicationState<ExecutionState>,
    pub bootstrap: Arc<ApplicationBootstrap>,
    pub metrics: Arc<ApplicationMetrics>,
}

/// Current execution state for application stress tests
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecutionState {
    pub run_id: Option<String>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub applications_running: usize,
    pub current_memory_mb: u64,
    pub peak_memory_mb: u64,
}

impl ApplicationStressEnvironment {
    /// Create a new application stress testing environment
    pub async fn new(config: ApplicationStressConfig) -> ApplicationStressResult<Self> {
        let bootstrap = Arc::new(ApplicationBootstrap::new().await?);
        let metrics = Arc::new(ApplicationMetrics::new());

        // Create artifact directory if enabled
        if config.store_artifacts {
            tokio::fs::create_dir_all(&config.artifact_dir).await.ok();
        }

        Ok(Self {
            config,
            state: shared_state(ExecutionState::default()),
            bootstrap,
            metrics,
        })
    }

    /// Initialize the Omnisystem environment
    pub async fn initialize(&self) -> ApplicationStressResult<()> {
        let mut state = self.state.write().await;
        state.run_id = Some(uuid::Uuid::new_v4().to_string());
        state.start_time = Some(chrono::Utc::now());

        tracing::info!("Initializing Bonsai Ecosystem applications on Omnisystem");

        Ok(())
    }

    /// Shutdown the testing environment
    pub async fn shutdown(&self) -> ApplicationStressResult<()> {
        let mut state = self.state.write().await;
        state.end_time = Some(chrono::Utc::now());

        tracing::info!("Shutting down application stress testing environment");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ApplicationStressConfig::default();
        assert_eq!(config.max_concurrent_apps, 10);
        assert_eq!(config.max_concurrent_users, 100);
        assert!(!config.deterministic);
    }

    #[tokio::test]
    async fn test_environment_creation() {
        let config = ApplicationStressConfig::default();
        let env = ApplicationStressEnvironment::new(config).await;
        assert!(env.is_ok());
    }

    #[tokio::test]
    async fn test_environment_initialization() {
        let config = ApplicationStressConfig::default();
        let env = ApplicationStressEnvironment::new(config).await.unwrap();
        env.initialize().await.unwrap();

        let state = env.state.read().await;
        assert!(state.run_id.is_some());
        assert!(state.start_time.is_some());
    }

    #[tokio::test]
    async fn test_environment_shutdown() {
        let config = ApplicationStressConfig::default();
        let env = ApplicationStressEnvironment::new(config).await.unwrap();
        env.initialize().await.unwrap();
        env.shutdown().await.unwrap();

        let state = env.state.read().await;
        assert!(state.end_time.is_some());
    }
}
