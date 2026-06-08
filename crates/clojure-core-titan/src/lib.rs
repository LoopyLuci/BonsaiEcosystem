//! Clojure Titan Core - Persistent data structures for Clojure
//!
//! Phase 2 stub implementation.
//! This module provides core types and traits for the clojure-core-titan system.

use std::sync::Arc;

/// Core module for clojure-core-titan
pub mod core {
    /// Marker trait for clojure-core-titan components
    pub trait Component: Send + Sync + 'static {}
}

/// Re-export core types
pub use core::Component;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loads() {
        // Placeholder test - Phase 2 will add real tests
    }
}