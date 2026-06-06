//! Error types for OpenCV operations

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to allocate matrix")]
    AllocationFailed,

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid image format: {0}")]
    InvalidFormat(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("GPU operation failed: {0}")]
    GpuError(String),

    #[error("Unsupported depth value: {0}")]
    UnsupportedDepth(i32),

    #[error("Null pointer returned from OpenCV")]
    NullPointer,

    #[error("OpenCV error: {0}")]
    OpenCvError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::ffi::NulError),
}

pub type Result<T> = std::result::Result<T, Error>;

/// OpenCV depth constants
#[repr(i32)]
pub enum MatDepth {
    CV_8U = 0,
    CV_8S = 1,
    CV_16U = 2,
    CV_16S = 3,
    CV_32S = 4,
    CV_32F = 5,
    CV_64F = 6,
}

/// OpenCV color conversion codes
#[repr(i32)]
pub enum ColorConversionCode {
    COLOR_BGR2GRAY = 6,
    COLOR_RGB2GRAY = 7,
    COLOR_GRAY2BGR = 8,
    COLOR_GRAY2RGB = 9,
    COLOR_BGR2HSV = 40,
    COLOR_RGB2HSV = 41,
}

/// Image read flags
#[repr(i32)]
pub enum ImreadModes {
    IMREAD_COLOR = 1,
    IMREAD_GRAYSCALE = 0,
    IMREAD_UNCHANGED = -1,
}
