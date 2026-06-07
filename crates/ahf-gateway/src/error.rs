//! Error types for the Anti-Hallucination Gateway

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Gateway-specific errors
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum GatewayError {
    /// Core AHF error
    #[error("AHF error: {0}")]
    AhfError(String),

    /// Pipeline execution error
    #[error("Pipeline error: {reason}")]
    PipelineError { reason: String },

    /// Configuration error
    #[error("Configuration error: {reason}")]
    ConfigError { reason: String },

    /// Actor error
    #[error("Actor error: {reason}")]
    ActorError { reason: String },

    /// Timeout during pipeline execution
    #[error("Pipeline timeout exceeded {ms}ms")]
    PipelineTimeout { ms: u64 },

    /// Audit logging error
    #[error("Audit log error: {reason}")]
    AuditError { reason: String },

    /// Metrics collection error
    #[error("Metrics error: {reason}")]
    MetricsError { reason: String },

    /// Invalid input
    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },

    /// Component not available
    #[error("Component unavailable: {component}")]
    ComponentUnavailable { component: String },

    /// Serialization error
    #[error("Serialization error: {reason}")]
    SerializationError { reason: String },

    /// Internal error
    #[error("Internal error: {reason}")]
    Internal { reason: String },
}

impl GatewayError {
    pub fn pipeline_error(reason: impl Into<String>) -> Self {
        Self::PipelineError {
            reason: reason.into(),
        }
    }

    pub fn config_error(reason: impl Into<String>) -> Self {
        Self::ConfigError {
            reason: reason.into(),
        }
    }

    pub fn actor_error(reason: impl Into<String>) -> Self {
        Self::ActorError {
            reason: reason.into(),
        }
    }

    pub fn audit_error(reason: impl Into<String>) -> Self {
        Self::AuditError {
            reason: reason.into(),
        }
    }

    pub fn metrics_error(reason: impl Into<String>) -> Self {
        Self::MetricsError {
            reason: reason.into(),
        }
    }

    pub fn invalid_input(reason: impl Into<String>) -> Self {
        Self::InvalidInput {
            reason: reason.into(),
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::PipelineTimeout { .. } | Self::AuditError { .. } | Self::MetricsError { .. }
        )
    }

    pub fn requires_escalation(&self) -> bool {
        matches!(
            self,
            Self::ConfigError { .. } | Self::ComponentUnavailable { .. } | Self::Internal { .. }
        )
    }
}

impl From<ahf_core::AhfError> for GatewayError {
    fn from(err: ahf_core::AhfError) -> Self {
        GatewayError::AhfError(err.to_string())
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        GatewayError::SerializationError {
            reason: err.to_string(),
        }
    }
}

impl From<std::io::Error> for GatewayError {
    fn from(err: std::io::Error) -> Self {
        GatewayError::Internal {
            reason: err.to_string(),
        }
    }
}

/// Result type for Gateway operations
pub type GatewayResult<T> = Result<T, GatewayError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_error_creation() {
        let err = GatewayError::pipeline_error("test error");
        assert!(matches!(err, GatewayError::PipelineError { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = GatewayError::config_error("invalid threshold");
        let msg = err.to_string();
        assert!(msg.contains("Configuration error"));
    }

    #[test]
    fn test_is_recoverable() {
        assert!(GatewayError::PipelineTimeout { ms: 50 }.is_recoverable());
        assert!(!GatewayError::ConfigError {
            reason: "test".to_string()
        }
        .is_recoverable());
    }

    #[test]
    fn test_requires_escalation() {
        assert!(GatewayError::ConfigError {
            reason: "test".to_string()
        }
        .requires_escalation());
        assert!(!GatewayError::PipelineTimeout { ms: 50 }.requires_escalation());
    }

    #[test]
    fn test_error_conversion() {
        let ahf_err = ahf_core::AhfError::internal("test");
        let gw_err: GatewayError = ahf_err.into();
        assert!(matches!(gw_err, GatewayError::AhfError(_)));
    }
}
