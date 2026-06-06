use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;
use std::fmt;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum ModelFormat {
    #[serde(rename = "gguf")]
    GGUF,
    #[serde(rename = "safetensors")]
    SafeTensors,
    #[serde(rename = "pytorch")]
    PyTorch,
    #[serde(rename = "onnx")]
    ONNX,
    #[serde(rename = "bkp")]
    BonsaiPackage,
    #[serde(rename = "unknown")]
    Unknown,
}

impl fmt::Display for ModelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GGUF => "GGUF",
            Self::SafeTensors => "SafeTensors",
            Self::PyTorch => "PyTorch",
            Self::ONNX => "ONNX",
            Self::BonsaiPackage => "BonsaiPackage",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub id: String,
    pub filename: String,
    pub path: PathBuf,
    pub format: ModelFormat,
    pub size_bytes: u64,
    pub parameter_count: Option<u64>,
    pub quantization: Option<String>,
    pub context_length: Option<usize>,
    pub architecture: Option<String>,
    pub discovered_at: String,
}

pub struct ModelScanner {
    extensions: Vec<String>,
}

impl Default for ModelScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelScanner {
    pub fn new() -> Self {
        Self {
            extensions: vec![
                ".gguf".to_string(),
                ".safetensors".to_string(),
                ".bin".to_string(),
                ".pt".to_string(),
                ".pth".to_string(),
                ".onnx".to_string(),
                ".bkp".to_string(),
            ],
        }
    }

    /// Detect model format from file extension and signature
    pub fn detect_format(&self, path: &Path) -> ModelFormat {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext.to_lowercase().as_str() {
            "gguf" => ModelFormat::GGUF,
            "safetensors" => ModelFormat::SafeTensors,
            "bin" | "pt" | "pth" => {
                // PyTorch models are usually .bin, .pt, or .pth
                // Could also be ONNX depending on content
                if path.to_string_lossy().contains("onnx") {
                    ModelFormat::ONNX
                } else {
                    ModelFormat::PyTorch
                }
            }
            "onnx" => ModelFormat::ONNX,
            "bkp" => ModelFormat::BonsaiPackage,
            _ => ModelFormat::Unknown,
        }
    }

    /// Extract metadata from GGUF file (simplified – reads header only)
    fn extract_gguf_metadata(&self, path: &Path) -> Result<(Option<u64>, Option<String>, Option<usize>)> {
        // In production, use llama-cpp-rs or ggml-rs to read GGUF metadata
        // For now, we estimate from filename patterns
        let filename = path.file_name().unwrap_or_default().to_string_lossy();

        let param_count = if filename.contains("1b") || filename.contains("1.1b") {
            Some(1_100_000_000)
        } else if filename.contains("7b") {
            Some(7_000_000_000)
        } else if filename.contains("13b") {
            Some(13_000_000_000)
        } else if filename.contains("34b") {
            Some(34_000_000_000)
        } else if filename.contains("70b") {
            Some(70_000_000_000)
        } else if filename.contains("180b") {
            Some(180_000_000_000)
        } else {
            None
        };

        let quant = if filename.contains("q4_0") || filename.contains("Q4_0") {
            Some("Q4_0".to_string())
        } else if filename.contains("q5_0") || filename.contains("Q5_0") {
            Some("Q5_0".to_string())
        } else if filename.contains("q8_0") || filename.contains("Q8_0") {
            Some("Q8_0".to_string())
        } else if filename.contains("f16") || filename.contains("fp16") {
            Some("fp16".to_string())
        } else if filename.contains("f32") || filename.contains("fp32") {
            Some("fp32".to_string())
        } else {
            None
        };

        let ctx = Some(2048); // Default, could be detected from metadata

        Ok((param_count, quant, ctx))
    }

    /// Extract metadata from SafeTensors file
    fn extract_safetensors_metadata(&self, path: &Path) -> Result<(Option<u64>, Option<String>)> {
        // In production, use the safetensors crate to read header
        // For now, estimate from filename
        let filename = path.file_name().unwrap_or_default().to_string_lossy();

        let param_count = if filename.contains("1b") {
            Some(1_000_000_000)
        } else if filename.contains("7b") {
            Some(7_000_000_000)
        } else {
            None
        };

        Ok((param_count, Some("safetensors".to_string())))
    }

    /// Scan directory recursively and build model inventory
    pub fn scan(&self, directory: &Path) -> Result<Vec<ModelMetadata>> {
        let mut models = Vec::new();
        let mut id_counter = 0;

        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                self.extensions.iter().any(|ext| {
                    path.to_string_lossy()
                        .to_lowercase()
                        .ends_with(ext)
                })
            })
        {
            id_counter += 1;
            let path = entry.path().to_path_buf();
            let metadata = entry.metadata()?;
            let size_bytes = metadata.len();
            let format = self.detect_format(&path);

            let (param_count, quant, ctx) = match format {
                ModelFormat::GGUF => {
                    let (pc, q, c) = self.extract_gguf_metadata(&path)?;
                    (pc, q, c)
                }
                ModelFormat::SafeTensors => {
                    let (pc, q) = self.extract_safetensors_metadata(&path)?;
                    (pc, q, None)
                }
                _ => (None, None, None),
            };

            let filename = path
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("unknown")
                .to_string();

            models.push(ModelMetadata {
                id: format!("model_{:03}", id_counter),
                filename,
                path,
                format,
                size_bytes,
                parameter_count: param_count,
                quantization: quant,
                context_length: ctx,
                architecture: None,
                discovered_at: Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            });
        }

        // Sort by size (ascending) – smallest first for validation
        models.sort_by_key(|m| m.size_bytes);

        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        let scanner = ModelScanner::new();

        assert_eq!(
            scanner.detect_format(Path::new("model.gguf")),
            ModelFormat::GGUF
        );
        assert_eq!(
            scanner.detect_format(Path::new("model.safetensors")),
            ModelFormat::SafeTensors
        );
        assert_eq!(
            scanner.detect_format(Path::new("model.bin")),
            ModelFormat::PyTorch
        );
    }

    #[test]
    fn test_parameter_extraction() {
        let scanner = ModelScanner::new();
        let (pc, _, _) = scanner
            .extract_gguf_metadata(Path::new("tinyllama-1.1b.Q4_0.gguf"))
            .unwrap();

        assert_eq!(pc, Some(1_100_000_000));
    }
}
