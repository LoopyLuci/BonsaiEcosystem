//! Error types for chaos engineering and fault injection operations.

use thiserror::Error;

/// Result type for chaos operations.
pub type Result<T> = std::result::Result<T, ChaosError>;

/// Comprehensive error type for chaos engineering operations.
#[derive(Error, Debug, Clone)]
pub enum ChaosError {
    #[error("Scenario configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid fault schedule: {0}")]
    InvalidSchedule(String),

    #[error("Fault injection failed: {0}")]
    InjectionError(String),

    #[error("Recovery failed: {0}")]
    RecoveryError(String),

    #[error("Scenario execution error: {0}")]
    ExecutionError(String),

    #[error("Invalid scenario state: {0}")]
    StateError(String),

    #[error("Clock synchronization error: {0}")]
    ClockError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Timeout during operation")]
    Timeout,

    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("Resource exhaustion: {0}")]
    ResourceExhausted(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Cascading fault detected: {0}")]
    CascadingFault(String),

    #[error("Byzantine behavior detected: {0}")]
    ByzantineDetected(String),

    #[error("Data corruption detected: {0}")]
    DataCorruptionDetected(String),

    #[error("Silent failure detected: {0}")]
    SilentFailure(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ChaosError {
    /// Check if this is a recoverable error.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ChaosError::Timeout
                | ChaosError::ChannelError(_)
                | ChaosError::InjectionError(_)
                | ChaosError::SilentFailure(_)
        )
    }

    /// Get severity level (0-10).
    pub fn severity(&self) -> u8 {
        match self {
            ChaosError::ConfigurationError(_) => 3,
            ChaosError::InvalidSchedule(_) => 3,
            ChaosError::InjectionError(_) => 5,
            ChaosError::RecoveryError(_) => 7,
            ChaosError::ExecutionError(_) => 6,
            ChaosError::StateError(_) => 6,
            ChaosError::ClockError(_) => 8,
            ChaosError::SerializationError(_) => 4,
            ChaosError::Timeout => 5,
            ChaosError::ChannelError(_) => 5,
            ChaosError::ResourceExhausted(_) => 8,
            ChaosError::InvalidParameter(_) => 3,
            ChaosError::CascadingFault(_) => 9,
            ChaosError::ByzantineDetected(_) => 9,
            ChaosError::DataCorruptionDetected(_) => 10,
            ChaosError::SilentFailure(_) => 9,
            ChaosError::Internal(_) => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity() {
        let timeout_err = ChaosError::Timeout;
        assert_eq!(timeout_err.severity(), 5);

        let corruption_err = ChaosError::DataCorruptionDetected("data".to_string());
        assert_eq!(corruption_err.severity(), 10);
    }

    #[test]
    fn test_error_recoverability() {
        let timeout_err = ChaosError::Timeout;
        assert!(timeout_err.is_recoverable());

        let corruption_err = ChaosError::DataCorruptionDetected("data".to_string());
        assert!(!corruption_err.is_recoverable());
    }
}
