//! Error types for Service Lifecycle Manager

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SLMError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Capability violation: {0}")]
    CapabilityViolation(String),

    #[error("Service already running: {0}")]
    ServiceAlreadyRunning(String),

    #[error("Service not running: {0}")]
    ServiceNotRunning(String),

    #[error("Vault creation failed: {0}")]
    VaultCreationFailed(String),

    #[error("Snapshot failed: {0}")]
    SnapshotFailed(String),

    #[error("Restore failed: {0}")]
    RestoreFailed(String),

    #[error("Resource quota exceeded: {0}")]
    ResourceQuotaExceeded(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Service crashed: {0}")]
    ServiceCrashed(String),

    #[error("Manifest error: {0}")]
    ManifestError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("JSON error: {0}")]
    JsonError(String),
}

pub type Result<T> = std::result::Result<T, SLMError>;
