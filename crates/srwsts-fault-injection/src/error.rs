//! Error types for fault injection framework.

use thiserror::Error;

/// Result type for fault injection operations.
pub type Result<T> = std::result::Result<T, FaultError>;

/// Comprehensive error type for fault injection failures.
#[derive(Debug, Error, Clone)]
pub enum FaultError {
    #[error("fault {0} not found")]
    FaultNotFound(String),

    #[error("invalid fault configuration: {0}")]
    InvalidConfiguration(String),

    #[error("fault schedule validation failed: {0}")]
    ScheduleValidationFailed(String),

    #[error("handler not registered for fault type: {0}")]
    HandlerNotFound(String),

    #[error("fault injection failed: {0}")]
    InjectionFailed(String),

    #[error("vault communication error: {0}")]
    VaultCommunicationError(String),

    #[error("fault recovery failed: {0}")]
    RecoveryFailed(String),

    #[error("timing error: {0}")]
    TimingError(String),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("timeout waiting for fault response")]
    ResponseTimeout,

    #[error("fault schedule not running")]
    ScheduleNotRunning,

    #[error("internal error: {0}")]
    InternalError(String),
}

impl FaultError {
    /// Check if this error is recoverable.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            FaultError::ResponseTimeout
                | FaultError::VaultCommunicationError(_)
                | FaultError::InjectionFailed(_)
        )
    }

    /// Get a severity level (0-10, higher = worse).
    pub fn severity(&self) -> u8 {
        match self {
            FaultError::ResponseTimeout => 3,
            FaultError::VaultCommunicationError(_) => 5,
            FaultError::InjectionFailed(_) => 6,
            FaultError::RecoveryFailed(_) => 8,
            FaultError::InvalidConfiguration(_) => 4,
            _ => 7,
        }
    }
}
