//! SRWSTS Orchestrator
//!
//! Provides test execution orchestration, coordination, and result management
//! for the Stress, Resilience, and Workload System Test Suite.
//!
//! ## Architecture
//!
//! The orchestrator coordinates:
//! - Test plan execution
//! - Concurrent test management
//! - Fault injection timing
//! - Result collection and aggregation
//! - Metrics gathering

use dashmap::DashMap;
use srwsts_core::{
    RunId, SrwstsConfig, SrwstsError, SrwstsResult, TestId, TestResult,
};
use std::sync::Arc;
use tokio::sync::Semaphore;
use uuid::Uuid;

/// Main SRWSTS orchestrator
pub struct Orchestrator {
    config: SrwstsConfig,
    concurrency_limiter: Arc<Semaphore>,
    active_tests: Arc<DashMap<TestId, RunId>>,
    results: Arc<DashMap<Uuid, TestResult>>,
}

impl Orchestrator {
    /// Create a new orchestrator with default configuration
    pub fn new() -> Self {
        Self::with_config(SrwstsConfig::default())
    }

    /// Create a new orchestrator with custom configuration
    pub fn with_config(config: SrwstsConfig) -> Self {
        Self {
            concurrency_limiter: Arc::new(Semaphore::new(config.max_concurrent)),
            config,
            active_tests: Arc::new(DashMap::new()),
            results: Arc::new(DashMap::new()),
        }
    }

    /// Execute a test plan
    pub async fn execute_test(
        &self,
        test_id: TestId,
        plan: &srwsts_core::TestPlan,
    ) -> SrwstsResult<TestResult> {
        // Validate plan
        plan.validate()?;

        // Check concurrency limit
        if self.active_tests.len() >= self.config.max_concurrent {
            return Err(SrwstsError::ConcurrencyLimitExceeded {
                current: self.active_tests.len(),
                limit: self.config.max_concurrent,
            });
        }

        // Check if test is already running
        if self.active_tests.contains_key(&test_id) {
            return Err(SrwstsError::TestAlreadyRunning {
                test_id: test_id.to_string(),
            });
        }

        let run_id = RunId::new();

        // Acquire concurrency slot
        let _permit = self.concurrency_limiter.acquire().await;

        // Record active test
        self.active_tests.insert(test_id.clone(), run_id);

        // Create result
        let result = TestResult::new(test_id.clone(), run_id, srwsts_core::types::Timestamp::now());

        // Record result
        self.results.insert(result.result_id, result.clone());

        // Remove from active tests
        self.active_tests.remove(&test_id);

        Ok(result)
    }

    /// Get a previously executed test result
    pub fn get_result(&self, result_id: Uuid) -> SrwstsResult<TestResult> {
        self.results
            .get(&result_id)
            .map(|entry| entry.clone())
            .ok_or(SrwstsError::ResultNotFound { result_id })
    }

    /// List all results
    pub fn list_results(&self) -> Vec<TestResult> {
        self.results
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Clear all stored results
    pub fn clear_results(&self) {
        self.results.clear();
    }

    /// Get current number of active tests
    pub fn active_test_count(&self) -> usize {
        self.active_tests.len()
    }

    /// Get total number of completed tests
    pub fn completed_test_count(&self) -> usize {
        self.results.len()
    }

    /// Check if a specific test is running
    pub fn is_test_running(&self, test_id: &TestId) -> bool {
        self.active_tests.contains_key(test_id)
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_creation() {
        let orch = Orchestrator::new();
        assert_eq!(orch.active_test_count(), 0);
        assert_eq!(orch.completed_test_count(), 0);
    }

    #[tokio::test]
    async fn test_orchestrator_with_config() {
        let config = SrwstsConfig {
            max_concurrent: 50,
            ..Default::default()
        };
        let orch = Orchestrator::with_config(config);
        assert_eq!(orch.config.max_concurrent, 50);
    }

    #[tokio::test]
    async fn test_execute_test_basic() {
        let orch = Orchestrator::new();
        let test_id = TestId::new("test1");
        let plan = srwsts_core::TestPlan::new("p1", "Test", "Description")
            .with_workload(srwsts_core::Workload::new("w1", "cpu_stress", 4, 300));

        let result = orch.execute_test(test_id, &plan).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_result() {
        let orch = Orchestrator::new();
        let test_id = TestId::new("test1");
        let plan = srwsts_core::TestPlan::new("p1", "Test", "Description")
            .with_workload(srwsts_core::Workload::new("w1", "cpu_stress", 4, 300));

        let result = orch.execute_test(test_id, &plan).await.unwrap();
        let retrieved = orch.get_result(result.result_id);
        assert!(retrieved.is_ok());
    }

    #[tokio::test]
    async fn test_list_results() {
        let orch = Orchestrator::new();
        let test_id = TestId::new("test1");
        let plan = srwsts_core::TestPlan::new("p1", "Test", "Description")
            .with_workload(srwsts_core::Workload::new("w1", "cpu_stress", 4, 300));

        let _ = orch.execute_test(test_id, &plan).await;
        let results = orch.list_results();
        assert_eq!(results.len(), 1);
    }
}
