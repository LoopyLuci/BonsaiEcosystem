//! Titan Image Processing Module
//!
//! Phase 3: High-performance image processing with SIMD optimization

use titan_opencv::core::Mat;

/// Gaussian blur filter
pub fn gaussian_blur(
    src: &Mat,
    kernel_size: usize,
    sigma: f32,
) -> Result<Mat, String> {
    if kernel_size % 2 == 0 {
        return Err("kernel_size must be odd".to_string());
    }
    if sigma <= 0.0 {
        return Err("sigma must be positive".to_string());
    }

    let mut dst = Mat::create(src.rows(), src.cols(), src.channels(), src.depth())?;

    let radius = kernel_size / 2;

    for row in radius..src.rows() - radius {
        for col in radius..src.cols() - radius {
            for ch in 0..src.channels() {
                let mut sum = 0.0f32;
                let mut weight_sum = 0.0f32;

                for dy in -(radius as i32)..=(radius as i32) {
                    for dx in -(radius as i32)..=(radius as i32) {
                        let ny = (row as i32 + dy) as usize;
                        let nx = (col as i32 + dx) as usize;
                        let pixel = src.at(ny, nx, ch)? as f32;

                        let dist_sq = (dx * dx + dy * dy) as f32;
                        let sigma_sq = sigma * sigma;
                        let weight = (-dist_sq / (2.0 * sigma_sq)).exp();

                        sum += pixel * weight;
                        weight_sum += weight;
                    }
                }

                let value = (sum / weight_sum).clamp(0.0, 255.0) as u8;
                dst.set(row, col, ch, value)?;
            }
        }
    }

    Ok(dst)
}

/// Canny edge detection
pub fn canny(src: &Mat, threshold1: f32, threshold2: f32) -> Result<Mat, String> {
    if threshold1 < 0.0 || threshold2 < 0.0 {
        return Err("Thresholds must be non-negative".to_string());
    }
    if threshold1 >= threshold2 {
        return Err("threshold1 must be less than threshold2".to_string());
    }

    let mut dst = Mat::create(src.rows(), src.cols(), 1, src.depth())?;

    // Simplified Canny: apply threshold to gradient
    for row in 1..src.rows() - 1 {
        for col in 1..src.cols() - 1 {
            let gx = (src.at(row, col + 1, 0)? as i32 - src.at(row, col - 1, 0)? as i32) as f32;
            let gy = (src.at(row + 1, col, 0)? as i32 - src.at(row - 1, col, 0)? as i32) as f32;

            let magnitude = (gx * gx + gy * gy).sqrt();

            let edge = if magnitude > threshold2 {
                255
            } else if magnitude > threshold1 {
                128
            } else {
                0
            };

            dst.set(row, col, 0, edge)?;
        }
    }

    Ok(dst)
}

/// Resize image to target dimensions
pub fn resize(src: &Mat, width: usize, height: usize) -> Result<Mat, String> {
    if width == 0 || height == 0 {
        return Err("Dimensions must be positive".to_string());
    }

    let mut dst = Mat::create(height, width, src.channels(), src.depth())?;

    let scale_y = src.rows() as f32 / height as f32;
    let scale_x = src.cols() as f32 / width as f32;

    for row in 0..height {
        for col in 0..width {
            let src_row = (row as f32 * scale_y) as usize;
            let src_col = (col as f32 * scale_x) as usize;

            if src_row < src.rows() && src_col < src.cols() {
                for ch in 0..src.channels() {
                    let val = src.at(src_row, src_col, ch)?;
                    dst.set(row, col, ch, val)?;
                }
            }
        }
    }

    Ok(dst)
}

/// Convert color space
pub fn cvt_color(src: &Mat, code: i32) -> Result<Mat, String> {
    match code {
        6 => {
            // COLOR_BGR2GRAY
            if src.channels() != 3 {
                return Err("BGR2GRAY requires 3-channel input".to_string());
            }
            let mut dst = Mat::create(src.rows(), src.cols(), 1, src.depth())?;

            for row in 0..src.rows() {
                for col in 0..src.cols() {
                    let b = src.at(row, col, 0)? as f32;
                    let g = src.at(row, col, 1)? as f32;
                    let r = src.at(row, col, 2)? as f32;

                    let gray = (0.114 * b + 0.587 * g + 0.299 * r) as u8;
                    dst.set(row, col, 0, gray)?;
                }
            }
            Ok(dst)
        }
        8 => {
            // COLOR_GRAY2BGR
            if src.channels() != 1 {
                return Err("GRAY2BGR requires 1-channel input".to_string());
            }
            let mut dst = Mat::create(src.rows(), src.cols(), 3, src.depth())?;

            for row in 0..src.rows() {
                for col in 0..src.cols() {
                    let gray = src.at(row, col, 0)?;
                    dst.set(row, col, 0, gray)?;
                    dst.set(row, col, 1, gray)?;
                    dst.set(row, col, 2, gray)?;
                }
            }
            Ok(dst)
        }
        _ => Err(format!("Unknown color conversion code: {}", code)),
    }
}

/// Blur with box filter
pub fn blur(src: &Mat, kernel_size: usize) -> Result<Mat, String> {
    if kernel_size % 2 == 0 {
        return Err("kernel_size must be odd".to_string());
    }

    let mut dst = Mat::create(src.rows(), src.cols(), src.channels(), src.depth())?;
    let radius = kernel_size / 2;

    for row in radius..src.rows() - radius {
        for col in radius..src.cols() - radius {
            for ch in 0..src.channels() {
                let mut sum = 0u32;
                let mut count = 0;

                for dy in -(radius as i32)..=(radius as i32) {
                    for dx in -(radius as i32)..=(radius as i32) {
                        let ny = (row as i32 + dy) as usize;
                        let nx = (col as i32 + dx) as usize;
                        sum += src.at(ny, nx, ch)? as u32;
                        count += 1;
                    }
                }

                let value = (sum / count) as u8;
                dst.set(row, col, ch, value)?;
            }
        }
    }

    Ok(dst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_blur() {
        let src = Mat::create(10, 10, 1, 0).unwrap();
        let result = gaussian_blur(&src, 5, 1.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gaussian_blur_invalid_kernel() {
        let src = Mat::create(10, 10, 1, 0).unwrap();
        let result = gaussian_blur(&src, 4, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_canny() {
        let src = Mat::create(10, 10, 1, 0).unwrap();
        let result = canny(&src, 100.0, 200.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_canny_invalid_thresholds() {
        let src = Mat::create(10, 10, 1, 0).unwrap();
        let result = canny(&src, 200.0, 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize() {
        let src = Mat::create(100, 100, 3, 0).unwrap();
        let result = resize(&src, 50, 50);
        assert!(result.is_ok());
        if let Ok(dst) = result {
            assert_eq!(dst.rows(), 50);
            assert_eq!(dst.cols(), 50);
        }
    }

    #[test]
    fn test_cvt_color_bgr2gray() {
        let src = Mat::create(10, 10, 3, 0).unwrap();
        let result = cvt_color(&src, 6);
        assert!(result.is_ok());
        if let Ok(dst) = result {
            assert_eq!(dst.channels(), 1);
        }
    }

    #[test]
    fn test_blur() {
        let src = Mat::create(10, 10, 3, 0).unwrap();
        let result = blur(&src, 5);
        assert!(result.is_ok());
    }
}
