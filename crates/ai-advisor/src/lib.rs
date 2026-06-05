//! Bonsai AI Fallback Framework
//!
//! Universal, `no_std`-compatible foundation for building AI-optional, deterministic-first systems
//! across the entire Bonsai Ecosystem and USOS.
//!
//! # Core Principle
//!
//! Every subsystem must be built on three layers:
//! 1. **Deterministic Core** — pure algorithms, formally verified, no ML runtime dependencies
//! 2. **Heuristic Layer** — rule-based fallback for cases the core cannot handle
//! 3. **Advisory Domain** — optional AI/ML enhancements, sandboxed, advisory-only
//!
//! A **Trusted Arbiter** orchestrates these layers via a graceful degradation ladder:
//! `AI → Heuristic → Deterministic Core → Safe Stub`
//!
//! If AI is disabled, fails, or is attacked, the system continues operating at baseline
//! performance with no loss of core functionality.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use core::fmt;
use serde::{Serialize, Deserialize};

mod error;
mod service;
mod arbiter;
mod advisory;
mod metrics;

pub use error::{Error, Result};
pub use service::{SovereignService, DeterministicCore, HeuristicLayer, ExecutionTier};
pub use arbiter::{Arbiter, ArbiterConfig, ExecutionDecision};
pub use advisory::{AdvisoryOutput, AdvisoryDomain};
pub use metrics::{ArbiterMetrics, ArbiterState};

/// Version of the framework
pub const VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::AiTimeout;
        assert!(!format!("{}", err).is_empty());
    }
}
