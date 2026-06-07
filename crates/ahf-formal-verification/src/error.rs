//! Error types for formal verification and production hardening

use thiserror::Error;

/// Errors that can occur during formal verification
#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),

    #[error("Theorem {theorem} violated: {reason}")]
    TheoremViolation { theorem: String, reason: String },

    #[error("Knowledge base integrity check failed: {0}")]
    KnowledgeBaseIntegrityError(String),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Continuous validation failed: {0}")]
    ContinuousValidationError(String),

    #[error("Hallucination rate exceeded threshold: {rate}% (threshold: {threshold}%)")]
    HallucinationRateExceeded { rate: f64, threshold: f64 },

    #[error("Hot-reload operation failed: {0}")]
    HotReloadError(String),

    #[error("Policy update rejected: {0}")]
    PolicyUpdateRejected(String),

    #[error("Deployment configuration error: {0}")]
    DeploymentError(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Metrics collection error: {0}")]
    MetricsError(String),

    #[error("Performance profiling error: {0}")]
    ProfilingError(String),

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Governance council error: {0}")]
    GovernanceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl VerificationError {
    /// Create a proof verification failure
    pub fn proof_failed(reason: impl Into<String>) -> Self {
        Self::ProofVerificationFailed(reason.into())
    }

    /// Create a theorem violation error
    pub fn theorem_violated(theorem: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::TheoremViolation {
            theorem: theorem.into(),
            reason: reason.into(),
        }
    }

    /// Create a knowledge base integrity error
    pub fn kb_integrity_error(reason: impl Into<String>) -> Self {
        Self::KnowledgeBaseIntegrityError(reason.into())
    }

    /// Create a hash mismatch error
    pub fn hash_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::HashMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Check if this is a critical error requiring immediate action
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            Self::TheoremViolation { .. }
                | Self::HallucinationRateExceeded { .. }
                | Self::KnowledgeBaseIntegrityError(_)
                | Self::HealthCheckFailed(_)
        )
    }

    /// Get a severity score (0.0 = low, 1.0 = critical)
    pub fn severity(&self) -> f64 {
        match self {
            Self::TheoremViolation { .. } => 1.0,
            Self::HallucinationRateExceeded { .. } => 1.0,
            Self::KnowledgeBaseIntegrityError(_) => 1.0,
            Self::HealthCheckFailed(_) => 0.9,
            Self::ProofVerificationFailed(_) => 0.8,
            Self::HotReloadError(_) => 0.7,
            Self::PolicyUpdateRejected(_) => 0.6,
            _ => 0.3,
        }
    }
}

/// Result type for verification operations
pub type VerificationResult<T> = std::result::Result<T, VerificationError>;
