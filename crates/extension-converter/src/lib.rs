//! `bonsai-extension-converter` — Bidirectional IDE extension gateway.
//!
//! # Supported directions
//!
//! | Source      | Destination  | Status  |
//! |-------------|--------------|---------|
//! | VSCode VSIX | Bonsai IR    | Phase 1 |
//! | Bonsai      | MCP Server   | Phase 1 |
//! | Bonsai      | VSCode VSIX  | Phase 2 |
//! | JetBrains   | Bonsai IR    | Phase 4 |
//!
//! All conversions go through the Unified Extension IR (see [`ir`]).

pub mod export;
pub mod import;
pub mod ir;

pub use ir::ExtensionIr;

/// Errors returned by conversion operations.
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("manifest not found: {0}")]
    ManifestNotFound(String),
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("conversion failed: {0}")]
    Failed(String),
}

impl From<ConversionError> for String {
    fn from(e: ConversionError) -> Self {
        e.to_string()
    }
}

/// Detect the source format from a file path.
pub fn detect_format(path: &std::path::Path) -> ir::SourceFormat {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "vsix" => {
            // Could be VSCode or Visual Studio — peek inside to distinguish
            // For now, treat all .vsix as VSCode (most common case)
            ir::SourceFormat::VsCode
        }
        "jar" | "zip" => ir::SourceFormat::JetBrains,
        _ => ir::SourceFormat::Unknown,
    }
}
