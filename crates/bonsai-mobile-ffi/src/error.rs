//! Error handling for the mobile FFI layer

use thiserror::Error;

/// Result type for mobile FFI operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for mobile FFI operations
#[derive(Debug, Error)]
pub enum Error {
    /// JNI initialization failed
    #[error("JNI initialization failed")]
    JniInitFailed,

    /// Invalid JNI call
    #[error("JNI error: {0}")]
    JniError(String),

    /// MediaCodec codec not available
    #[error("Codec not available: {0}")]
    CodecNotAvailable(String),

    /// Invalid codec configuration
    #[error("Invalid codec configuration: {0}")]
    InvalidConfiguration(String),

    /// Decoder not initialized
    #[error("Decoder not initialized")]
    DecoderNotInitialized,

    /// Encoding/decoding error
    #[error("Codec error: {0}")]
    CodecError(String),

    /// Input buffer error
    #[error("Input buffer error: {0}")]
    InputBufferError(String),

    /// Output buffer error
    #[error("Output buffer error: {0}")]
    OutputBufferError(String),

    /// Invalid frame data
    #[error("Invalid frame data: {0}")]
    InvalidFrameData(String),

    /// Timeout waiting for frame
    #[error("Timeout waiting for decoded frame")]
    Timeout,

    /// Frame format error
    #[error("Frame format error: {0}")]
    FrameFormatError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Low memory condition
    #[error("Low memory: {0}")]
    LowMemory(String),

    /// Device not supported
    #[error("Device not supported: {0}")]
    UnsupportedDevice(String),

    /// Generic error
    #[error("Error: {0}")]
    Other(String),
}

impl From<jni::errors::Error> for Error {
    fn from(e: jni::errors::Error) -> Self {
        Error::JniError(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerializationError(e.to_string())
    }
}

/// Helper macro to wrap JNI errors with context
#[macro_export]
macro_rules! jni_try {
    ($expr:expr, $msg:expr) => {
        $expr.map_err(|e| Error::JniError(format!("{}: {}", $msg, e)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::CodecNotAvailable("video/avc".to_string());
        assert!(err.to_string().contains("video/avc"));
    }

    #[test]
    fn test_error_from_string() {
        let err = Error::Other("test error".to_string());
        assert_eq!(err.to_string(), "Error: test error");
    }
}
