//! Error types for Omni-Bot

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Authorization failed: {0}")]
    Unauthorized(String),
    
    #[error("Capability validation failed: {0}")]
    CapabilityError(String),
    
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Service execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
