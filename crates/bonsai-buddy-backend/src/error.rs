//! Error types for Bonsai Buddy Backend

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API connection failed: {0}")]
    ApiConnection(String),

    #[error("API request failed: {0}")]
    ApiRequest(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("State error: {0}")]
    State(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Queue error: {0}")]
    Queue(String),

    #[error("Sync error: {0}")]
    Sync(String),

    #[error("Offline error: {0}")]
    Offline(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Timeout")]
    Timeout,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
