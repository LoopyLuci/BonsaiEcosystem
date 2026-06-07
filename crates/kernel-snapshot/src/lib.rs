//! Phase 1: Real Kernel Snapshot Syscalls (UOSC Extension)
//!
//! Provides real implementations of snapshot_vault and restore_vault syscalls
//! that were mocked in Phase 2 SLM.
//!
//! This replaces the mock kernel adapter with actual memory management operations:
//! - Memory region enumeration
//! - Process state serialization
//! - Capability table persistence
//! - CAS-backed snapshot storage
//! - Deterministic restore with context preservation

pub mod error;
pub mod memory;
pub mod syscalls;
pub mod snapshot;
pub mod restore;
pub mod capability_table;

pub use error::{KernelError, Result};
pub use syscalls::{snapshot_vault, restore_vault, create_vault, destroy_vault};
pub use snapshot::Snapshot;
pub use restore::RestoreContext;

/// Kernel version string
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize kernel snapshot module
pub fn init() {
    let _ = env_logger::builder().try_init();
    log::info!("Kernel snapshot support initialized (v{})", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
