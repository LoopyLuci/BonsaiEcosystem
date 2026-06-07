//! Omni-Bot Core Types & Infrastructure
//! Foundational types, traits, and error handling for the Omni-Bot system

pub mod error;
pub mod types;
pub mod capability;
pub mod action;
pub mod service;

pub use error::{Error, Result};
pub use types::*;
pub use capability::{Capability, CapabilityToken};
pub use action::Action;
pub use service::{ServiceStatus, ServiceState, ServiceInfo};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize core system
pub fn init() {
    log::info!("Omni-Bot Core v{} initialized", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
