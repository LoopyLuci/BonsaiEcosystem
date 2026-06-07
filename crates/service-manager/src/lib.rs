//! Service Lifecycle Manager (SLM) - Phase 2 Implementation
//!
//! Demand-activated, snapshotable background services with automatic lifecycle management.
//!
//! # Architecture
//!
//! ```text
//! Client Request
//!     │
//!     ▼
//! Service Lifecycle Manager (SLM Actor)
//!     │
//!     ├─► Check Capability
//!     ├─► Look up Service Manifest (UMS)
//!     ├─► Spawn or Restore from Snapshot
//!     └─► Return Connection Capability
//!
//! Background Loop (every 10 seconds)
//!     │
//!     ├─► Check idle timeout
//!     ├─► If idle: Pause & Snapshot
//!     ├─► Check health (heartbeat)
//!     └─► If crashed: Restore from snapshot
//! ```
//!
//! # Features
//!
//! * **Demand-Activated**: Services spawned only when requested
//! * **Snapshotable**: Full state preservation (memory, registers, capabilities)
//! * **Auto-Recovery**: Automatic restart from snapshots on failure
//! * **Resource Quotas**: Memory, CPU, I/O limits per service
//! * **Health Monitoring**: Heartbeat-based health checks
//! * **Audit Logging**: Complete lifecycle event logging
//! * **Hot-Reloadable**: Service binaries updated atomically via UMS

pub mod error;
pub mod kernel_adapter;
pub mod lifecycle;
pub mod service_registry;
pub mod types;

pub use error::{SLMError, Result};
pub use kernel_adapter::KernelAdapter;
pub use lifecycle::LifecycleManager;
pub use service_registry::ServiceRegistry;
pub use types::*;

/// SLM version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging
pub fn init_logging() {
    let _ = env_logger::builder().try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
