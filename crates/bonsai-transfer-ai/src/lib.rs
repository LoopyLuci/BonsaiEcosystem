//! Optional AI Enhancements for TransferDaemon v2
//!
//! This crate provides advisory-only AI enhancements that are:
//! - Disabled by default
//! - Feature-gated and sandboxed
//! - Safety-clamped to Axiom-verified bounds
//! - Never required for correct operation
//!
//! If the AI subsystem is disabled or fails, the deterministic core
//! continues at baseline performance.

pub mod advisor;
pub mod safety;

pub use advisor::AiCongestionAdvisor;
pub use safety::SafetyEnvelope;

use serde::{Serialize, Deserialize};

/// Central AI enhancement manager – all AI components are optional.
#[derive(Debug, Clone)]
pub struct AiEnhancementLayer {
    pub enabled: bool,
    pub congestion_advisor: Option<AiCongestionAdvisor>,
}

impl AiEnhancementLayer {
    /// Create with all AI components disabled.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            congestion_advisor: None,
        }
    }

    /// Create with AI components initialized (but disabled until explicitly enabled).
    pub fn new() -> Self {
        Self {
            enabled: false,
            congestion_advisor: Some(AiCongestionAdvisor::new(0.9)),
        }
    }

    /// Disable all AI — connections continue with deterministic fallback.
    pub fn disable_all(&mut self) {
        self.enabled = false;
        self.congestion_advisor = None;
    }

    /// Check if any AI component is active.
    pub fn is_any_active(&self) -> bool {
        self.enabled && self.congestion_advisor.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_disabled_by_default() {
        let layer = AiEnhancementLayer::new();
        assert!(!layer.enabled);
    }

    #[test]
    fn test_ai_disable_all() {
        let mut layer = AiEnhancementLayer::new();
        layer.enabled = true;
        layer.disable_all();
        assert!(!layer.is_any_active());
    }
}
