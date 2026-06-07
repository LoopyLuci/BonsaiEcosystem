//! SRWSTS Services - Omnisystem Service Stress Testing
//!
//! Comprehensive stress testing framework for Omnisystem core services (P2P, Storage, Network,
//! Compositor, Service Discovery, etc.) in isolation without UOSC kernel.
//!
//! ## Architecture
//!
//! - **ServiceBootstrap**: Load and manage Omnisystem services without kernel applications
//! - **Service-specific stress tests**: P2P mesh, storage operations, network stack, etc.
//! - **Fault injection**: Kill services, network partitions, backend failures, timeouts
//! - **Metrics collection**: Per-service latency, throughput, error rates, resource usage
//! - **Result reporting**: JSON output with health metrics, performance, recovery times
//!
//! ## Example Usage
//!
//! ```ignore
//! use srwsts_services::{ServiceBootstrap, ServiceStressConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = ServiceStressConfig::default();
//!     let mut bootstrap = ServiceBootstrap::new(config);
//!
//!     bootstrap.initialize().await?;
//!     bootstrap.run_all_tests().await?;
//!     let report = bootstrap.generate_report().await?;
//!
//!     println!("{}", serde_json::to_string_pretty(&report)?);
//!     Ok(())
//! }
//! ```

pub mod bootstrap;
pub mod compositor;
pub mod errors;
pub mod fault_scenarios;
pub mod metrics;
pub mod network;
pub mod p2p;
pub mod reporting;
pub mod service_discovery;
pub mod service_interaction;
pub mod storage;
pub mod types;

pub use bootstrap::{ServiceBootstrap, ServiceBootstrapConfig};
pub use compositor::CompositorStressTests;
pub use errors::{ServiceError, ServiceResult};
pub use fault_scenarios::FaultScenarioTests;
pub use metrics::{ServiceMetrics, ServiceMetricsCollector};
pub use network::NetworkStressTests;
pub use p2p::P2PStressTests;
pub use reporting::{ServiceHealthStatus, TestReport};
pub use service_discovery::ServiceDiscoveryTests;
pub use service_interaction::ServiceInteractionTests;
pub use storage::StorageStressTests;
pub use types::{Service, ServiceStatus, TestConfig};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_bootstrap_creation() {
        let _config = ServiceBootstrapConfig::default();
        // Tests are in integration test suite
    }
}
