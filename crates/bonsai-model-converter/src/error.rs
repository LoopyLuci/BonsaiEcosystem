//! Error types for model conversion operations

use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ConverterError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Zstd error: {0}")]
    Zstd(String),

    #[error("format error: {0}")]
    Format(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("conversion not supported: {from} → {to}")]
    ConversionNotSupported { from: String, to: String },

    #[error("file not found: {0}")]
    NotFound(PathBuf),

    #[error("invalid model: {0}")]
    InvalidModel(String),

    #[error("roundtrip verification failed: {0}")]
    RoundtripFailed(String),

    #[error("HuggingFace API error: {0}")]
    HuggingFaceApi(String),

    #[error("llama.cpp not found: {0}")]
    LlamaCppNotFound(String),

    #[error("subprocess error: {0}")]
    Subprocess(String),

    #[error("progress channel error: {0}")]
    ChannelError(String),

    #[error("signature verification failed: {0}")]
    SignatureVerification(String),

    #[error("timeout during {operation}")]
    Timeout { operation: String },

    #[error("concurrent operation limit exceeded")]
    TooManyConcurrent,

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("{context}: {source}")]
    WithContext {
        context: String,
        source: Box<ConverterError>,
    },
}

impl ConverterError {
    pub fn with_context<S: Into<String>>(self, context: S) -> Self {
        ConverterError::WithContext {
            context: context.into(),
            source: Box::new(self),
        }
    }

    pub fn zstd(msg: impl Into<String>) -> Self {
        ConverterError::Zstd(msg.into())
    }

    pub fn format(msg: impl Into<String>) -> Self {
        ConverterError::Format(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        ConverterError::Validation(msg.into())
    }

    pub fn invalid_model(msg: impl Into<String>) -> Self {
        ConverterError::InvalidModel(msg.into())
    }

    pub fn roundtrip_failed(msg: impl Into<String>) -> Self {
        ConverterError::RoundtripFailed(msg.into())
    }

    pub fn huggingface_api(msg: impl Into<String>) -> Self {
        ConverterError::HuggingFaceApi(msg.into())
    }

    pub fn llama_cpp_not_found(msg: impl Into<String>) -> Self {
        ConverterError::LlamaCppNotFound(msg.into())
    }

    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            ConverterError::Timeout { .. }
                | ConverterError::HuggingFaceApi(_)
                | ConverterError::Subprocess(_)
        )
    }
}

pub type ConverterResult<T> = Result<T, ConverterError>;
