//! Error types for BKP operations

use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum BkpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Zstd error: {0}")]
    Zstd(String),

    #[error("Invalid BKP: {0}")]
    Invalid(String),

    #[error("File not found: {0}")]
    NotFound(PathBuf),

    #[error("Signature verification failed: {0}")]
    SignatureVerification(String),

    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),

    #[error("Hash mismatch for {file}: expected {expected}, got {actual}")]
    HashMismatch {
        file: String,
        expected: String,
        actual: String,
    },

    #[error("{context}: {source}")]
    WithContext {
        context: String,
        source: Box<BkpError>,
    },
}

impl BkpError {
    pub fn with_context<S: Into<String>>(self, context: S) -> Self {
        BkpError::WithContext {
            context: context.into(),
            source: Box::new(self),
        }
    }

    pub fn zstd(msg: impl Into<String>) -> Self {
        BkpError::Zstd(msg.into())
    }

    pub fn invalid(msg: impl Into<String>) -> Self {
        BkpError::Invalid(msg.into())
    }
}

pub type BkpResult<T> = Result<T, BkpError>;
