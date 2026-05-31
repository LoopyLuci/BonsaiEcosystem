pub mod module;
pub mod retriever;
pub mod store;

pub use module::{LoadedModule, ModuleInfo, ModuleManifest};
pub use retriever::KdbRetriever;
pub use store::KdbStore;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdbError {
    #[error("module not found: {0}")]
    NotFound(String),
    #[error("dimension mismatch: expected {expected}, got {got}")]
    DimMismatch { expected: usize, got: usize },
    #[error("hnsw error: {0}")]
    Hnsw(#[from] bonsai_hnsw::HnswError),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid module: {0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, KdbError>;
