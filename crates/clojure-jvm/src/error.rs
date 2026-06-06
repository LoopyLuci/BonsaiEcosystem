//! Error types for Clojure JVM runtime

use thiserror::Error;

/// Result type for Clojure runtime operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for Clojure JVM operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("JVM initialization failed: {0}")]
    JvmInitializationError(String),

    #[error("Capability violation: {0}")]
    CapabilityViolation(String),

    #[error("File access denied: {0}")]
    FileAccessDenied(String),

    #[error("Network access denied: {0}")]
    NetworkAccessDenied(String),

    #[error("JNI error: {0}")]
    JniError(String),

    #[error("UABI bridge error: {0}")]
    UABIBridgeError(String),

    #[error("Clojure execution error: {0}")]
    ExecutionError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Timeout: operation took too long")]
    Timeout,

    #[error("Runtime not initialized")]
    RuntimeNotInitialized,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::InternalError(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::InternalError(msg.to_string())
    }
}
