//! Error types for the Anti-Hallucination Framework.
//!
//! Provides comprehensive error handling with 15+ variants covering all failure modes
//! in verification, knowledge base access, bias detection, and policy enforcement.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Comprehensive error type for all AHF operations.
///
/// Each variant captures specific failure modes that can occur during:
/// - Knowledge base lookup and source validation
/// - Verification and proof checking
/// - Bias detection and violation analysis
/// - Confidence extraction and calibration
/// - Policy enforcement and decision arbitration
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum AhfError {
    /// Knowledge base lookup failed (fact not found or source unavailable)
    #[error("Knowledge base lookup failed: {reason}")]
    KnowledgeBaseLookupFailed { reason: String },

    /// Source validation failed (source is invalid, untrustworthy, or inaccessible)
    #[error("Source validation failed: {reason}")]
    SourceValidationFailed { reason: String },

    /// Verification failed (schema mismatch, consistency violation, or proof invalid)
    #[error("Verification failed: {reason}")]
    VerificationFailed { reason: String },

    /// Proof validation failed (cryptographic hash mismatch, signature verification failed)
    #[error("Proof validation failed: {reason}")]
    ProofValidationFailed { reason: String },

    /// Consistency check failed (conflicting evidence, contradictory claims)
    #[error("Consistency check failed: {reason}")]
    ConsistencyCheckFailed { reason: String },

    /// Bias detected in claim or reasoning
    #[error("Bias detected: {reason}")]
    BiasDetected { reason: String },

    /// Bias violation exceeds policy threshold
    #[error("Bias violation policy exceeded: {reason}")]
    BiasViolationExceeded { reason: String },

    /// Confidence extraction failed (insufficient calibration data, invalid model)
    #[error("Confidence extraction failed: {reason}")]
    ConfidenceExtractionFailed { reason: String },

    /// Confidence calibration failed (invalid score, out of bounds)
    #[error("Confidence calibration failed: {reason}")]
    ConfidenceCalibrationFailed { reason: String },

    /// Policy not found (requested policy version does not exist)
    #[error("Policy not found: {reason}")]
    PolicyNotFound { reason: String },

    /// Policy enforcement failed (threshold violation, invalid configuration)
    #[error("Policy enforcement failed: {reason}")]
    PolicyEnforcementFailed { reason: String },

    /// Arbiter decision failed (unable to reach decision, escalation required)
    #[error("Arbiter decision failed: {reason}")]
    ArbiterDecisionFailed { reason: String },

    /// Invalid configuration (invalid threshold values, malformed policy)
    #[error("Invalid configuration: {reason}")]
    InvalidConfiguration { reason: String },

    /// Serialization failed (serde error, JSON encoding issue)
    #[error("Serialization failed: {reason}")]
    SerializationFailed { reason: String },

    /// Internal error (unexpected state, implementation bug)
    #[error("Internal error: {reason}")]
    Internal { reason: String },

    /// IO error (file read/write, network communication)
    #[error("IO error: {reason}")]
    IoError { reason: String },

    /// Timeout error (operation exceeded time limit)
    #[error("Operation timed out: {reason}")]
    Timeout { reason: String },
}

impl AhfError {
    /// Create a KnowledgeBaseLookupFailed error with custom reason.
    pub fn knowledge_base_lookup_failed(reason: impl Into<String>) -> Self {
        Self::KnowledgeBaseLookupFailed {
            reason: reason.into(),
        }
    }

    /// Create a SourceValidationFailed error with custom reason.
    pub fn source_validation_failed(reason: impl Into<String>) -> Self {
        Self::SourceValidationFailed {
            reason: reason.into(),
        }
    }

    /// Create a VerificationFailed error with custom reason.
    pub fn verification_failed(reason: impl Into<String>) -> Self {
        Self::VerificationFailed {
            reason: reason.into(),
        }
    }

    /// Create a ProofValidationFailed error with custom reason.
    pub fn proof_validation_failed(reason: impl Into<String>) -> Self {
        Self::ProofValidationFailed {
            reason: reason.into(),
        }
    }

    /// Create a ConsistencyCheckFailed error with custom reason.
    pub fn consistency_check_failed(reason: impl Into<String>) -> Self {
        Self::ConsistencyCheckFailed {
            reason: reason.into(),
        }
    }

    /// Create a BiasDetected error with custom reason.
    pub fn bias_detected(reason: impl Into<String>) -> Self {
        Self::BiasDetected {
            reason: reason.into(),
        }
    }

    /// Create a BiasViolationExceeded error with custom reason.
    pub fn bias_violation_exceeded(reason: impl Into<String>) -> Self {
        Self::BiasViolationExceeded {
            reason: reason.into(),
        }
    }

    /// Create a ConfidenceExtractionFailed error with custom reason.
    pub fn confidence_extraction_failed(reason: impl Into<String>) -> Self {
        Self::ConfidenceExtractionFailed {
            reason: reason.into(),
        }
    }

    /// Create a ConfidenceCalibrationFailed error with custom reason.
    pub fn confidence_calibration_failed(reason: impl Into<String>) -> Self {
        Self::ConfidenceCalibrationFailed {
            reason: reason.into(),
        }
    }

    /// Create a PolicyNotFound error with custom reason.
    pub fn policy_not_found(reason: impl Into<String>) -> Self {
        Self::PolicyNotFound {
            reason: reason.into(),
        }
    }

    /// Create a PolicyEnforcementFailed error with custom reason.
    pub fn policy_enforcement_failed(reason: impl Into<String>) -> Self {
        Self::PolicyEnforcementFailed {
            reason: reason.into(),
        }
    }

    /// Create an ArbiterDecisionFailed error with custom reason.
    pub fn arbiter_decision_failed(reason: impl Into<String>) -> Self {
        Self::ArbiterDecisionFailed {
            reason: reason.into(),
        }
    }

    /// Create an InvalidConfiguration error with custom reason.
    pub fn invalid_configuration(reason: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            reason: reason.into(),
        }
    }

    /// Create a SerializationFailed error with custom reason.
    pub fn serialization_failed(reason: impl Into<String>) -> Self {
        Self::SerializationFailed {
            reason: reason.into(),
        }
    }

    /// Create an Internal error with custom reason.
    pub fn internal(reason: impl Into<String>) -> Self {
        Self::Internal {
            reason: reason.into(),
        }
    }

    /// Create an IoError with custom reason.
    pub fn io_error(reason: impl Into<String>) -> Self {
        Self::IoError {
            reason: reason.into(),
        }
    }

    /// Create a Timeout error with custom reason.
    pub fn timeout(reason: impl Into<String>) -> Self {
        Self::Timeout {
            reason: reason.into(),
        }
    }

    /// Check if this error is recoverable or fatal.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::KnowledgeBaseLookupFailed { .. }
                | Self::SourceValidationFailed { .. }
                | Self::BiasDetected { .. }
                | Self::Timeout { .. }
        )
    }

    /// Check if this error requires immediate escalation.
    pub fn requires_escalation(&self) -> bool {
        matches!(
            self,
            Self::VerificationFailed { .. }
                | Self::ProofValidationFailed { .. }
                | Self::BiasViolationExceeded { .. }
                | Self::PolicyEnforcementFailed { .. }
        )
    }
}

impl From<serde_json::Error> for AhfError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationFailed {
            reason: err.to_string(),
        }
    }
}

impl From<std::io::Error> for AhfError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            reason: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_base_lookup_failed() {
        let err = AhfError::knowledge_base_lookup_failed("fact not found");
        assert!(matches!(err, AhfError::KnowledgeBaseLookupFailed { .. }));
    }

    #[test]
    fn test_bias_detected() {
        let err = AhfError::bias_detected("confirmation bias detected");
        assert!(matches!(err, AhfError::BiasDetected { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = AhfError::verification_failed("schema mismatch");
        let display = format!("{}", err);
        assert!(display.contains("Verification failed"));
        assert!(display.contains("schema mismatch"));
    }

    #[test]
    fn test_is_recoverable() {
        assert!(AhfError::knowledge_base_lookup_failed("test").is_recoverable());
        assert!(AhfError::timeout("test").is_recoverable());
        assert!(!AhfError::verification_failed("test").is_recoverable());
    }

    #[test]
    fn test_requires_escalation() {
        assert!(AhfError::verification_failed("test").requires_escalation());
        assert!(AhfError::bias_violation_exceeded("test").requires_escalation());
        assert!(!AhfError::timeout("test").requires_escalation());
    }

    #[test]
    fn test_error_serialization() {
        let err = AhfError::bias_detected("test bias");
        let json = serde_json::to_string(&err).expect("serialization failed");
        assert!(json.contains("BiasDetected"));
        assert!(json.contains("test bias"));
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json_err: AhfError = serde_json::from_str::<serde_json::Value>("invalid json")
            .map(|_| ())
            .unwrap_err()
            .into();
        assert!(matches!(json_err, AhfError::SerializationFailed { .. }));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let ahf_err: AhfError = io_err.into();
        assert!(matches!(ahf_err, AhfError::IoError { .. }));
    }
}
