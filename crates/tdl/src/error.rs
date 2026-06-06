//! Error types for the Training Data Library.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TdlError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("invalid version: {0}")]
    InvalidVersion(String),

    #[error("example not found: {0}")]
    ExampleNotFound(String),

    #[error("version not found: {0}")]
    VersionNotFound(String),

    #[error("invalid quality score: {0}")]
    InvalidQualityScore(f32),

    #[error("export error: {0}")]
    Export(String),

    #[error("invalid format: {0}")]
    InvalidFormat(String),

    #[error("merge conflict: {0}")]
    MergeConflict(String),

    #[error("transaction error: {0}")]
    Transaction(String),
}

pub type Result<T> = std::result::Result<T, TdlError>;
