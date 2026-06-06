#![allow(non_camel_case_types)]
//! OpenCV 5 Rust FFI Bindings and Safe Wrappers
//!
//! Phase 2 of the OpenCV 5 Omnisystem Integration
//! Provides memory-safe, idiomatic Rust API over OpenCV 5 C++ library

pub mod sys;
pub mod mat;
pub mod error;
pub mod imgcodecs;
pub mod imgproc;

pub use error::{Error, Result};
pub use mat::Mat;
pub use imgcodecs::{imread, imwrite};
pub use imgproc::{blur, gaussian_blur, canny};

/// OpenCV module version
pub const VERSION: &str = "5.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "5.0.0");
    }
}
