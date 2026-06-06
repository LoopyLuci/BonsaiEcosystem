//! Titan Core Implementation of OpenCV 5
//!
//! Phase 3: Complete rewrite in Titan systems language semantics
//! This is the definitive, sovereign implementation with capability-based access

pub mod core;
pub mod capabilities;

pub use core::Mat;
pub use capabilities::{Capability, GpuCapability, CpuCapability};

/// Titan OpenCV version
pub const VERSION: &str = "5.0.0-titan";

/// Matrix data types (matches OpenCV)
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MatDepth {
    CV_8U = 0,
    CV_8S = 1,
    CV_16U = 2,
    CV_16S = 3,
    CV_32S = 4,
    CV_32F = 5,
    CV_64F = 6,
}

/// Color space conversion codes
#[repr(i32)]
pub enum ColorCode {
    BGR2GRAY = 6,
    RGB2GRAY = 7,
    GRAY2BGR = 8,
    GRAY2RGB = 9,
    BGR2HSV = 40,
    RGB2HSV = 41,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "5.0.0-titan");
    }

    #[test]
    fn test_depth_constants() {
        assert_eq!(MatDepth::CV_8U as u8, 0);
        assert_eq!(MatDepth::CV_32F as u8, 5);
    }
}
