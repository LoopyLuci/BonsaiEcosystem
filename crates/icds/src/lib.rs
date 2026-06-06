//! # Bonsai Infinite Context Database System (ICDS)
//!
//! Provides AI agents with effectively unbounded, associative external memory through:
//! - Hierarchical semantic atom storage (content-addressed)
//! - Multi-resolution retrieval (O(log N) time)
//! - Deterministic-first architecture (AI enhancements optional)
//! - Sovereign, verifiable context provenance
//!
//! See [`ICDS_DESIGN.md`](docs/ICDS_DESIGN.md) for architecture details.

#![warn(missing_docs)]

pub mod atom;
pub mod index;
pub mod storage;
pub mod retrieval;
pub mod context;
pub mod api;
pub mod error;

pub use atom::{SemanticAtom, AtomId, Resolution, ResolutionLevel};
pub use error::{Result, IcdsError};
pub use retrieval::{QueryResult, RetrievalEngine};
pub use storage::AtomStore;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Main ICDS engine that coordinates ingestion, indexing, and retrieval.
#[derive(Clone)]
pub struct InfiniteContextEngine {
    /// Content-addressed storage for atoms
    store: Arc<dyn AtomStore>,
    /// Vector index for semantic search
    index: Arc<index::HierarchicalIndex>,
    /// Retrieval engine with resolution cascade
    retrieval: Arc<RetrievalEngine>,
    /// Metadata and config
    config: IcdsConfig,
}

/// Configuration for the ICDS engine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IcdsConfig {
    /// Maximum segment size in tokens
    pub max_segment_tokens: usize,
    /// Number of resolution levels (default: 3)
    pub resolution_levels: usize,
    /// HNSW index parameters
    pub hnsw_m: usize,
    pub hnsw_ef_construction: usize,
    pub hnsw_ef_search: usize,
    /// Cache size (atoms)
    pub hot_cache_size: usize,
    /// Enable deduplication via CAS
    pub enable_dedup: bool,
    /// Enable full-text search index
    pub enable_full_text: bool,
}

impl Default for IcdsConfig {
    fn default() -> Self {
        Self {
            max_segment_tokens: 512,
            resolution_levels: 3,
            hnsw_m: 16,
            hnsw_ef_construction: 200,
            hnsw_ef_search: 50,
            hot_cache_size: 100_000,
            enable_dedup: true,
            enable_full_text: true,
        }
    }
}

impl InfiniteContextEngine {
    /// Create a new ICDS engine with default configuration
    pub async fn new() -> Result<Self> {
        Self::with_config(IcdsConfig::default()).await
    }

    /// Create with custom configuration
    pub async fn with_config(config: IcdsConfig) -> Result<Self> {
        let store = Arc::new(storage::MemoryAtomStore::new());
        let index = Arc::new(index::HierarchicalIndex::new(
            config.hnsw_m,
            config.hnsw_ef_construction,
        )?);
        let retrieval = Arc::new(RetrievalEngine::new(
            store.clone(),
            index.clone(),
            config.clone(),
        ));

        Ok(Self {
            store,
            index,
            retrieval,
            config,
        })
    }

    /// Ingest raw text as semantic atoms
    ///
    /// This is the primary entry point for adding context to the system.
    /// The deterministic pipeline:
    /// 1. Chunks input into atomic units (sentences, paragraphs, code blocks)
    /// 2. Deduplicates via CAS (if enabled)
    /// 3. Generates embeddings (deterministic sparse or optional AI dense)
    /// 4. Creates multi-resolution representations
    /// 5. Indexes for retrieval
    pub async fn ingest(
        &self,
        text: &str,
        metadata: atom::AtomMetadata,
    ) -> Result<Vec<AtomId>> {
        let chunks = chunk_deterministic(text, self.config.max_segment_tokens);
        let mut atom_ids = Vec::new();

        for chunk in chunks {
            let atom = SemanticAtom::from_text(
                chunk.clone(),
                metadata.clone(),
                self.config.resolution_levels,
            )?;

            let atom_id = atom.id.clone();
            self.store.store(&atom).await?;
            self.index.insert(&atom).await?;

            atom_ids.push(atom_id);
        }

        Ok(atom_ids)
    }

    /// Query for semantically relevant atoms
    ///
    /// Uses hierarchical retrieval: keywords → summaries → full text
    pub async fn query(&self, text: &str, limit: usize) -> Result<QueryResult> {
        self.retrieval.query(text, limit).await
    }

    /// Retrieve full text of a specific atom
    pub async fn recall(&self, atom_id: &AtomId) -> Result<Option<SemanticAtom>> {
        self.store.get(atom_id).await
    }

    /// Assemble context for an AI model
    ///
    /// Returns a hierarchical, compressed context string suitable for
    /// feeding to an LLM, with markers indicating source and resolution level.
    pub async fn assemble_context(&self, query: &str, max_tokens: usize) -> Result<String> {
        let results = self.query(query, 100).await?;
        context::assemble_hierarchical(&results, max_tokens).await
    }

    /// Mark atoms for deletion (right to be forgotten)
    pub async fn forget(&self, atom_ids: &[AtomId]) -> Result<()> {
        for id in atom_ids {
            self.store.delete(id).await?;
            self.index.remove(id).await?;
        }
        Ok(())
    }

    /// Get statistics about the system state
    pub async fn stats(&self) -> Result<SystemStats> {
        Ok(SystemStats {
            total_atoms: self.store.count().await?,
            indexed_atoms: self.index.size().await?,
            cache_hit_rate: self.retrieval.cache_hit_rate().await,
        })
    }
}

/// System statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// Total atoms in storage
    pub total_atoms: u64,
    /// Atoms in vector index
    pub indexed_atoms: u64,
    /// Cache hit rate (0.0-1.0)
    pub cache_hit_rate: f64,
}

/// Deterministic chunking of text into atomic units
///
/// Uses language-aware boundaries (sentences, paragraphs, code blocks).
/// Guarantees: identical input produces identical chunks.
fn chunk_deterministic(text: &str, max_tokens: usize) -> Vec<String> {
    // Simple sentence-based chunking for MVP
    // In production: use rule-based splitter (pysbd-style)
    text.split('.')
        .filter(|s| !s.trim().is_empty())
        .map(|s| format!("{}.", s.trim()))
        .collect::<Vec<_>>()
        .windows(1)
        .map(|w| w.to_vec().join(" "))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let engine = InfiniteContextEngine::new().await.unwrap();
        let stats = engine.stats().await.unwrap();
        assert_eq!(stats.total_atoms, 0);
    }

    #[tokio::test]
    async fn test_ingest_single_atom() {
        let engine = InfiniteContextEngine::new().await.unwrap();
        let metadata = atom::AtomMetadata {
            source: atom::SourceType::UserInput,
            agent_id: Uuid::nil(),
            conversation_id: Some(Uuid::nil()),
            tags: vec![],
            importance: 1.0,
        };

        let ids = engine
            .ingest("This is a test sentence.", metadata)
            .await
            .unwrap();

        assert!(!ids.is_empty());

        let stats = engine.stats().await.unwrap();
        assert!(stats.total_atoms > 0);
    }

    #[tokio::test]
    async fn test_chunking() {
        let text = "First sentence. Second sentence. Third sentence.";
        let chunks = chunk_deterministic(text, 512);
        assert!(!chunks.is_empty());
    }
}
