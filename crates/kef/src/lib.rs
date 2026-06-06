//! Bonsai Knowledge Extraction Fabric (KEF)
//!
//! Production-grade knowledge extraction from AI models with multiple extraction methods:
//! - Synthetic data generation from model outputs
//! - Activation vector extraction and clustering
//! - Attention weight analysis for triplet extraction
//! - Membership inference for high-confidence data
//! - Multi-stage deduplication and PII redaction
//!
//! All extracted knowledge is curated, deduplicated, and ingested into KDB modules
//! with full provenance tracking via TDL.

pub mod activation_extractor;
pub mod attention_extractor;
pub mod curator;
pub mod error;
pub mod ingestion;
pub mod kef_service;
pub mod membership_inference;
pub mod model_scanner;
pub mod quality_scorer;
pub mod redaction;
pub mod synthetic_generator;
pub mod types;

pub use error::{KefError, Result};
pub use kef_service::KefService;
pub use model_scanner::{ModelReport, ModelScanner, ModelType};
pub use types::{
    CuratedChunk, ExtractionMethod, ExtractionReport, KmodPackage, QualityScores,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // Verify all modules compile
        let _methods = vec![
            ExtractionMethod::Synthetic,
            ExtractionMethod::Activation,
            ExtractionMethod::Attention,
            ExtractionMethod::MembershipInference,
        ];
    }
}
