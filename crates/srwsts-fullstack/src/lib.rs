//! # SRWSTS Full-Stack Integration Testing System
//!
//! Comprehensive stress-testing suite for the entire Omnisystem:
//! - UOSC kernel
//! - Omnisystem services (SLM, Buddy, Workspace, Survival System)
//! - Bonsai applications and runtimes
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  Full-Stack Test Orchestrator           │
//! │  (FullStackTestRunner)                  │
//! └──────────────┬──────────────────────────┘
//!                │
//!    ┌───────────┼───────────┐
//!    │           │           │
//! ┌──▼──┐  ┌────▼────┐  ┌───▼────┐
//! │Boot │  │ Nominal │  │  Peak  │
//! │     │  │ Load    │  │ Load   │
//! └─────┘  └─────────┘  └────────┘
//!    │           │           │
//! ┌──▼──────────────────────────┐
//! │ Vault (UOSC + Services)     │
//! │ ┌────────────────────────┐  │
//! │ │ UOSC Kernel            │  │
//! │ │ ┌──────────────────┐   │  │
//! │ │ │Scheduler/Thread │   │  │
//! │ │ │Memory Manager   │   │  │
//! │ │ │I/O Subsystem    │   │  │
//! │ │ └──────────────────┘   │  │
//! │ └────────────────────────┘  │
//! │                             │
//! │ ┌────────────────────────┐  │
//! │ │ Omnisystem Services    │  │
//! │ │ ┌──────────────────┐   │  │
//! │ │ │SLM (Service Mgr) │   │  │
//! │ │ │Buddy (Sync)      │   │  │
//! │ │ │Workspace (Files) │   │  │
//! │ │ │Survival (Logs)   │   │  │
//! │ │ └──────────────────┘   │  │
//! │ └────────────────────────┘  │
//! │                             │
//! │ ┌────────────────────────┐  │
//! │ │ Bonsai Applications    │  │
//! │ │ ┌──────────────────┐   │  │
//! │ │ │Multi-language    │   │  │
//! │ │ │Runtimes          │   │  │
//! │ │ └──────────────────┘   │  │
//! │ └────────────────────────┘  │
//! └────────────────────────────┘
//! ```
//!
//! ## Test Categories
//!
//! 1. **Bootstrap Tests**: Verify system initialization
//! 2. **Nominal Load Tests**: Baseline performance under normal workload
//! 3. **Peak Load Tests**: CPU, memory, I/O saturation
//! 4. **Cascading Failure Tests**: Component isolation under failures
//! 5. **Recovery Tests**: Resilience and state consistency
//! 6. **Network Partition Tests**: Mesh healing and CRDT convergence
//! 7. **State Consistency Tests**: Audit log verification and replay
//! 8. **Performance Under Failure Tests**: Latency degradation and MTTR
//! 9. **End-to-End User Journey Tests**: Real-world workflows
//! 10. **Long-Duration Stress Tests**: 72-hour endurance runs
//!
//! ## Usage
//!
//! ```no_run
//! use srwsts_fullstack::{FullStackTestRunner, FullStackBootstrap};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let bootstrap = FullStackBootstrap::default();
//!     let vault = bootstrap.initialize().await?;
//!
//!     let runner = FullStackTestRunner::new(vault);
//!     let results = runner.run_all_tests().await?;
//!
//!     println!("{:?}", results.summary());
//!     Ok(())
//! }
//! ```

pub mod bootstrap;
pub mod cascading_failures;
pub mod end_to_end_journey;
pub mod errors;
pub mod long_duration;
pub mod network_partitions;
pub mod nominal_loads;
pub mod peak_loads;
pub mod recovery;
pub mod reporter;
pub mod runner;
pub mod state_consistency;
pub mod vault;

// Re-exports for convenience
pub use bootstrap::{FullStackBootstrap, BootstrapConfig};
pub use errors::{FullStackTestError, FullStackTestResult};
pub use runner::{FullStackTestRunner, FullStackTestResults};
pub use vault::{Vault, VaultConfig, ComponentHealth};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
