//! Error types for the Knowledge Extraction Fabric

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KefError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("model scanner error: {0}")]
    ModelScan(String),

    #[error("model not supported: {0}")]
    UnsupportedModel(String),

    #[error("extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("embedding error: {0}")]
    EmbeddingFailed(String),

    #[error("clustering error: {0}")]
    ClusteringFailed(String),

    #[error("curation failed: {0}")]
    CurationFailed(String),

    #[error("ingestion failed: {0}")]
    IngestionFailed(String),

    #[error("pii redaction failed: {0}")]
    RedactionFailed(String),

    #[error("quality scoring failed: {0}")]
    ScoringFailed(String),

    #[error("kdb error: {0}")]
    Kdb(#[from] bonsai_kdb::KdbError),

    #[error("tdl error: {0}")]
    Tdl(String),

    #[error("hnsw error: {0}")]
    Hnsw(#[from] bonsai_hnsw::HnswError),

    #[error("invalid chunk: {0}")]
    InvalidChunk(String),

    #[error("invalid embedding dimension: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("extraction cancelled")]
    Cancelled,

    #[error("compression error: {0}")]
    Compression(String),

    #[error("model loading failed: {0}")]
    ModelLoading(String),

    #[error("unicode normalization error")]
    UnicodeNormalization,

    #[error("unknown error: {0}")]
    Other(String),
}

/// Result type for Knowledge Extraction Fabric operations
pub type Result<T> = std::result::Result<T, KefError>;
