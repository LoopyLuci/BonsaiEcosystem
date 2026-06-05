//! Error types for UDC operations

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UdcError {
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Code generation failed: {0}")]
    CodeGenFailed(String),

    #[error("Device interface error: {0}")]
    DeviceInterfaceError(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, UdcError>;
