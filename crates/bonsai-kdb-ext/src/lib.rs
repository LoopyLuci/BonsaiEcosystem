//! Team K: Knowledge Database Integration
//!
//! Cross-project vulnerability patterns, vector embeddings, and pattern matching.

pub mod interfaces;
pub mod config;
pub mod kdb_engine;

pub use interfaces::*;
pub use config::KDBConfig;
pub use kdb_engine::{KnowledgeDatabase, VulnerabilityPattern};

pub struct KDBIntegration {
    config: KDBConfig,
    db: KnowledgeDatabase,
}

impl KDBIntegration {
    pub fn new(config: KDBConfig) -> Self {
        Self {
            db: KnowledgeDatabase::new(),
            config,
        }
    }

    pub fn store_pattern(&mut self, pattern: VulnerabilityPattern) {
        self.db.add_pattern(pattern);
    }

    pub fn search_similar(&self, query: &str) -> Vec<VulnerabilityPattern> {
        self.db.search(query)
    }

    pub fn total_patterns(&self) -> usize {
        self.db.len()
    }
}

pub async fn init() -> Result<(), anyhow::Error> {
    tracing::info!("Initializing Knowledge Database");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialization() {
        assert!(init().await.is_ok());
    }

    #[tokio::test]
    async fn test_kdb_integration() {
        let config = KDBConfig::default();
        let mut integration = KDBIntegration::new(config);
        let pattern = VulnerabilityPattern {
            vulnerability_type: "SQL Injection".to_string(),
            cve: Some("CVE-2024-0001".to_string()),
            description: "SQL injection in user input".to_string(),
        };
        integration.store_pattern(pattern);
        assert!(integration.total_patterns() > 0);
    }
}
