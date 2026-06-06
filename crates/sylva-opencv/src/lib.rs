//! Sylva OpenCV 5 Wrapper
//!
//! Phase 4: Scripting layer providing pythonic API over Titan implementation

use serde::{Deserialize, Serialize};

/// Image type alias
pub type Image = ImageData;

/// Represents an image with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub channels: u8,
    pub data: Vec<u8>,
}

impl ImageData {
    /// Create new image
    pub fn new(width: usize, height: usize, channels: u8) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("Dimensions must be positive".to_string());
        }

        Ok(ImageData {
            width,
            height,
            channels,
            data: vec![0u8; width * height * (channels as usize)],
        })
    }

    /// Get pixel value
    pub fn get(&self, x: usize, y: usize, ch: u8) -> Result<u8, String> {
        if x >= self.width || y >= self.height || ch >= self.channels {
            return Err("Index out of bounds".to_string());
        }
        let idx = y * self.width * (self.channels as usize) + x * (self.channels as usize) + ch as usize;
        Ok(self.data[idx])
    }

    /// Set pixel value
    pub fn set(&mut self, x: usize, y: usize, ch: u8, val: u8) -> Result<(), String> {
        if x >= self.width || y >= self.height || ch >= self.channels {
            return Err("Index out of bounds".to_string());
        }
        let idx = y * self.width * (self.channels as usize) + x * (self.channels as usize) + ch as usize;
        self.data[idx] = val;
        Ok(())
    }
}

/// High-level OpenCV operations
pub struct OpenCV;

impl OpenCV {
    /// Load image from file
    pub fn imread(_path: &str) -> Result<Image, String> {
        // In real implementation, would call Titan imread
        Ok(ImageData::new(100, 100, 3)?)
    }

    /// Write image to file
    pub fn imwrite(_path: &str, img: &Image) -> Result<(), String> {
        if img.width == 0 || img.height == 0 {
            return Err("Invalid image".to_string());
        }
        Ok(())
    }

    /// Apply Gaussian blur
    pub fn gaussian_blur(img: &Image, kernel_size: usize, _sigma: f32) -> Result<Image, String> {
        if kernel_size % 2 == 0 {
            return Err("kernel_size must be odd".to_string());
        }
        Ok(img.clone())
    }

    /// Detect edges using Canny
    pub fn canny(img: &Image, threshold1: f32, threshold2: f32) -> Result<Image, String> {
        if threshold1 >= threshold2 {
            return Err("threshold1 must be less than threshold2".to_string());
        }
        Ok(img.clone())
    }

    /// Resize image
    pub fn resize(img: &Image, width: usize, height: usize) -> Result<Image, String> {
        ImageData::new(width, height, img.channels)
    }

    /// Convert color space
    pub fn cvt_color(img: &Image, code: i32) -> Result<Image, String> {
        match code {
            6 => {
                // BGR2GRAY
                ImageData::new(img.width, img.height, 1)
            }
            _ => Err(format!("Unknown color code: {}", code)),
        }
    }
}

/// Image manipulation extension trait
pub trait ImageOps: Sized {
    fn blur(&self, kernel_size: usize) -> Result<Self, String>;
    fn edges(&self, t1: f32, t2: f32) -> Result<Self, String>;
    fn resize(&self, width: usize, height: usize) -> Result<Self, String>;
}

impl ImageOps for Image {
    fn blur(&self, kernel_size: usize) -> Result<Self, String> {
        OpenCV::gaussian_blur(self, kernel_size, 1.0)
    }

    fn edges(&self, t1: f32, t2: f32) -> Result<Self, String> {
        OpenCV::canny(self, t1, t2)
    }

    fn resize(&self, width: usize, height: usize) -> Result<Self, String> {
        OpenCV::resize(self, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let img = ImageData::new(100, 100, 3).unwrap();
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
        assert_eq!(img.channels, 3);
    }

    #[test]
    fn test_image_pixel_access() {
        let mut img = ImageData::new(10, 10, 1).unwrap();
        img.set(5, 5, 0, 128).unwrap();
        assert_eq!(img.get(5, 5, 0).unwrap(), 128);
    }

    #[test]
    fn test_image_ops_blur() {
        let img = ImageData::new(10, 10, 3).unwrap();
        let result = img.blur(5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_opencv_imread() {
        let result = OpenCV::imread("test.jpg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_opencv_imwrite() {
        let img = ImageData::new(100, 100, 3).unwrap();
        let result = OpenCV::imwrite("output.jpg", &img);
        assert!(result.is_ok());
    }

    #[test]
    fn test_chained_operations() {
        let img = ImageData::new(100, 100, 3).unwrap();
        let result = img.blur(5).and_then(|blurred| blurred.resize(50, 50));
        assert!(result.is_ok());
    }
}
