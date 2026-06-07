//! SRWSTS Core Infrastructure
//!
//! Provides fundamental types, traits, and error handling for the Stress,
//! Resilience, and Workload System Test Suite (SRWSTS).
//!
//! ## Key Components
//!
//! - **Types**: Core data structures for test plans, results, and metrics
//! - **Traits**: Extensible interfaces for test execution, fault injection, and result collection
//! - **Errors**: Comprehensive error types for all failure scenarios
//! - **Enums**: Status, result types, and fault definitions
//!
//! ## Architecture
//!
//! SRWSTS follows a modular design where:
//! - Test orchestrators implement `TestExecutor`
//! - Fault injection systems implement `FaultInjector`
//! - Result collectors implement `ResultCollector`
//! - All components communicate through standardized types

pub mod errors;
pub mod metrics;
pub mod result;
pub mod status;
pub mod test_plan;
pub mod traits;
pub mod types;

pub use errors::{SrwstsError, SrwstsResult};
pub use metrics::{ResourceMetrics, TestMetrics};
pub use result::TestResult;
pub use status::{ExecutionStatus, ResultStatus};
pub use test_plan::{FaultDefinition, FaultType, ResourceLimits, TestPlan, Workload};
pub use traits::{FaultInjector, ResultCollector, TestExecutor};
pub use types::{Duration, Timestamp};

use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Unique identifier for test runs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// Unique identifier for test cases
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TestId(String);

impl TestId {
    /// Create a new test ID from a string
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Shared, thread-safe container for test state
pub type SharedState<T> = Arc<RwLock<T>>;

/// Create a new shared state container
pub fn shared_state<T>(value: T) -> SharedState<T> {
    Arc::new(RwLock::new(value))
}

/// Configuration for SRWSTS execution environment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SrwstsConfig {
    /// Maximum number of concurrent test executions
    pub max_concurrent: usize,
    /// Maximum test duration before timeout
    pub test_timeout: std::time::Duration,
    /// Whether to enable fault injection
    pub enable_faults: bool,
    /// Whether to enable result collection
    pub enable_collection: bool,
    /// Deterministic mode: disable random fault timing
    pub deterministic: bool,
}

impl Default for SrwstsConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            test_timeout: std::time::Duration::from_secs(300),
            enable_faults: true,
            enable_collection: true,
            deterministic: false,
        }
    }
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
    fn test_test_id_creation() {
        let id = TestId::new("test-01");
        assert_eq!(id.as_str(), "test-01");
    }

    #[test]
    fn test_shared_state_creation() {
        let shared = shared_state(42);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let val = shared.read().await;
            assert_eq!(*val, 42);
        });
    }

    #[test]
    fn test_srwsts_config_default() {
        let config = SrwstsConfig::default();
        assert_eq!(config.max_concurrent, 100);
        assert!(!config.deterministic);
    }
}
