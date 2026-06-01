use thiserror::Error;

/// Android Bridge error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("Discovery error: {0}")]
    DiscoveryError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Capability check failed: {0}")]
    CapabilityError(String),

    #[error("Streaming error: {0}")]
    StreamingError(String),

    #[error("Input injection error: {0}")]
    InputError(String),

    #[error("File synchronization error: {0}")]
    FileSyncError(String),

    #[error("Device communication error: {0}")]
    CommunicationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Cryptography error: {0}")]
    CryptoError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;
