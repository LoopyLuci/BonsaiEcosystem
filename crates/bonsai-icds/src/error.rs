//! Error types for ICDS

use thiserror::Error;

/// Result type for ICDS operations
pub type Result<T> = std::result::Result<T, IcdsError>;

/// ICDS error types
#[derive(Error, Debug)]
pub enum IcdsError {
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Index error
    #[error("Index error: {0}")]
    Index(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// System time error
    #[error("System time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid argument
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IcdsError {
    /// Create storage error
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }

    /// Create index error
    pub fn index(msg: impl Into<String>) -> Self {
        Self::Index(msg.into())
    }

    /// Create not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create invalid argument error
    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }

    /// Create internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
