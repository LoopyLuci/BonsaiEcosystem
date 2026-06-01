//! Bonsai Knowledge Package (BKP) builder and loader
//!
//! Provides high-level APIs for creating and extracting .bkp packages,
//! which are zstd-compressed ZIP archives containing:
//! - manifest.json (package metadata and file registry)
//! - base_model/ (GGUF or other base model files)
//! - modules/ (KMOD knowledge modules)
//! - adapters/ (LoRA/QLoRA adapters)
//!
//! # Creating a BKP
//!
//! ```no_run
//! use bonsai_bkp::BkpBuilder;
//! # async fn example() -> anyhow::Result<()> {
//! let mut builder = BkpBuilder::new("my-model", "1.0.0")?;
//! builder.add_base_model("model.gguf")?;
//! builder.add_kmod_module("knowledge.kmod", "knowledge")?;
//! builder.set_description("My custom model package");
//! builder.finalize("output.bkp")?;
//! # Ok(())
//! # }
//! ```
//!
//! # Loading a BKP
//!
//! ```no_run
//! use bonsai_bkp::BkpLoader;
//! # async fn example() -> anyhow::Result<()> {
//! let loader = BkpLoader::new("model.bkp")?;
//! let manifest = loader.manifest()?;
//! println!("Model: {} v{}", manifest.name, manifest.version);
//! loader.extract_to("/tmp/model")?;
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod loader;
pub mod error;
pub mod manifest;

pub use builder::BkpBuilder;
pub use loader::BkpLoader;
pub use error::{BkpError, BkpResult};
pub use manifest::BkpManifest;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a KMOD module within a BKP package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmodInfo {
    pub name: String,
    pub path_in_package: String,
    pub hash: String,
    pub size_bytes: u64,
}

/// Metadata for an adapter within a BKP package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    pub name: String,
    pub adapter_type: String,
    pub path_in_package: String,
    pub hash: String,
    pub size_bytes: u64,
}

/// File entry in a BKP package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub hash: String,
    pub size_bytes: u64,
    pub entry_type: String, // "model", "kmod", "adapter", "metadata"
}
