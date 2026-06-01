//! Bonsai Model Converter — Production-grade model format conversion
//!
//! Supports conversion between:
//! - GGUF (quantized, llama.cpp format)
//! - safetensors (Hugging Face format)
//! - .bkp (Bonsai Knowledge Package)
//! - HuggingFace Hub (remote download/upload)
//!
//! All converters operate streaming where possible to avoid loading entire models into memory.
//!
//! # Example
//!
//! ```no_run
//! use bonsai_model_converter::*;
//!
//! # async fn example() -> Result<()> {
//! // Convert GGUF to BKP
//! let config = ConversionConfig {
//!     context_length: 4096,
//!     ..Default::default()
//! };
//!
//! convert_gguf_to_bkp(
//!     "model.gguf",
//!     "model.bkp",
//!     config,
//! ).await?;
//! # Ok(())
//! # }
//! ```

pub mod converters;
pub mod format;
pub mod progress;
pub mod validation;
pub mod error;

pub use converters::{
    convert_gguf_to_bkp, convert_bkp_to_gguf, convert_gguf_to_safetensors,
    convert_safetensors_to_gguf, convert_bkp_to_safetensors, convert_safetensors_to_bkp,
    convert_huggingface_to_bkp, convert_bkp_to_huggingface, convert_batch,
};
pub use format::{ModelFormat, FormatDetector, detect_format};
pub use progress::{ProgressReporter, ConversionProgress};
pub use validation::{validate_model, validate_roundtrip};
pub use error::{ConverterError, ConverterResult};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for model conversion operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    /// Model context length (tokens)
    pub context_length: u32,

    /// Model name for metadata
    pub model_name: Option<String>,

    /// Author/creator name
    pub author: Option<String>,

    /// License identifier
    pub license: Option<String>,

    /// Description
    pub description: Option<String>,

    /// Tags/keywords
    pub tags: Vec<String>,

    /// Whether to verify conversion roundtrip
    pub verify_roundtrip: bool,

    /// Whether to compress BKP output
    pub compress_bkp: bool,

    /// Number of concurrent threads for batch operations
    pub parallel_jobs: usize,

    /// Timeout in seconds for remote operations (HF downloads)
    pub timeout_secs: u64,

    /// Whether to skip signature verification on BKP files
    pub skip_signature_check: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            context_length: 4096,
            model_name: None,
            author: None,
            license: None,
            description: None,
            tags: Vec::new(),
            verify_roundtrip: false,
            compress_bkp: true,
            parallel_jobs: num_cpus::get(),
            timeout_secs: 300,
            skip_signature_check: false,
        }
    }
}

/// Batch conversion specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConversionSpec {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub from_format: ModelFormat,
    pub to_format: ModelFormat,
    pub config: ConversionConfig,
}
