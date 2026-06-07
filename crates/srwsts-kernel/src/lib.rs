//! SRWSTS Kernel - Independent Stress Testing for UOSC Kernel
//!
//! A comprehensive stress testing framework for testing the UOSC kernel in isolation
//! with no userspace services running. Tests kernel subsystems independently through
//! emulation and validation.
//!
//! ## Components
//!
//! - **KernelTestBootstrap**: Boot minimal kernel image (kernel + initrd only)
//! - **KernelSchedulerTests**: EDF scheduler, CFS fairness, priority, preemption, context switching
//! - **KernelMemoryTests**: Allocation/deallocation stress, fragmentation, NUMA, OOM, huge pages
//! - **KernelIPCTests**: Message passing (1M msgs/sec target), latency, capability revocation, semaphores
//! - **KernelDriverTests**: Storage I/O (100% random), network driver (line-rate), interrupt latency
//! - **KernelInvariantTests**: Verify Axiom-proven invariants hold under stress
//! - **KernelSnapshotTests**: Boot from snapshot, restore under load, state consistency
//! - **FaultScenarios**: Memory pressure, clock skew, hardware failures, thermal throttling
//! - **MetricsCollection**: Latency histograms, throughput, CPU/memory profiling, cache behavior
//! - **ResultReporting**: JSON output with pass/fail, metrics, detailed logs

pub mod bootstrap;
pub mod scheduler;
pub mod memory;
pub mod ipc;
pub mod drivers;
pub mod invariants;
pub mod snapshots;
pub mod faults;
pub mod metrics;
pub mod reporting;

pub use bootstrap::{KernelBootstrap, BootstrapConfig};
pub use scheduler::{SchedulerTest, SchedulerConfig};
pub use memory::{MemoryTest, MemoryConfig};
pub use ipc::{IPCTest, IPCConfig};
pub use drivers::{DriverTest, DriverConfig};
pub use invariants::{InvariantTest, InvariantConfig};
pub use snapshots::{SnapshotTest, SnapshotConfig};
pub use faults::{FaultScenario, FaultConfig};
pub use metrics::{MetricsCollector, LatencyHistogram};
pub use reporting::{ResultReport, ReportGenerator};

use srwsts_core::{TestId, SrwstsConfig};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Global kernel test context
#[derive(Debug, Clone)]
pub struct KernelTestContext {
    /// Test suite configuration
    pub config: SrwstsConfig,
    /// Metrics collector
    pub metrics: Arc<RwLock<MetricsCollector>>,
    /// Shared test state
    pub test_id: TestId,
}

impl KernelTestContext {
    /// Create a new kernel test context
    pub fn new(config: SrwstsConfig) -> Self {
        Self {
            config,
            metrics: Arc::new(RwLock::new(MetricsCollector::new())),
            test_id: TestId::new("kernel-test"),
        }
    }

    /// Create with custom test ID
    pub fn with_id(mut self, test_id: TestId) -> Self {
        self.test_id = test_id;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_test_context_creation() {
        let context = KernelTestContext::new(SrwstsConfig::default());
        assert_eq!(context.config.max_concurrent, 100);
    }

    #[test]
    fn test_kernel_test_context_with_id() {
        let context = KernelTestContext::new(SrwstsConfig::default())
            .with_id(TestId::new("custom-id"));
        assert_eq!(context.test_id.as_str(), "custom-id");
    }
}
