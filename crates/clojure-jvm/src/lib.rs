//! Clojure JVM Runtime with Capability-Based Sandboxing
//!
//! This module provides:
//! - Embedded JVM launcher
//! - Capability enforcement layer
//! - UABI bridge for inter-language calls
//! - POSIX shim integration

pub mod launcher;
pub mod capabilities;
pub mod jni_bridge;
pub mod error;

pub use launcher::{ClojureRuntime, RuntimeConfig};
pub use capabilities::{Capability, CapabilityToken, AccessControl};
pub use jni_bridge::UABIBridge;
pub use error::{Error, Result};

/// Initialize the Clojure runtime with capabilities
///
/// # Example
/// ```no_run
/// use clojure_jvm::{RuntimeConfig, Capability};
///
/// let config = RuntimeConfig::default()
///     .with_capability(Capability::Filesystem(vec!["/tmp".to_string()]))
///     .with_capability(Capability::Network);
///
/// let runtime = clojure_jvm::init(config).unwrap();
/// ```
pub fn init(config: RuntimeConfig) -> Result<ClojureRuntime> {
    ClojureRuntime::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let config = RuntimeConfig::default();
        let _runtime = init(config).expect("Failed to create runtime");
    }

    #[test]
    fn test_capability_enforcement() {
        let mut config = RuntimeConfig::default();
        config = config.with_capability(Capability::Filesystem(vec!["/safe".to_string()]));

        let runtime = init(config).expect("Failed to create runtime");
        assert!(runtime.can_access_path("/safe/file.txt"));
        assert!(!runtime.can_access_path("/unsafe/file.txt"));
    }
}
