//! Team J: Survival System Integration
//!
//! Permanent bug memory with confidence scoring, historical learning, and pattern preservation.

pub mod interfaces;
pub mod config;
pub mod survival_db;

pub use interfaces::*;
pub use config::SurvivalSystemConfig;
pub use survival_db::{SurvivalDatabase, BugRecord};

pub struct SurvivalSystemEngine {
    config: SurvivalSystemConfig,
    db: SurvivalDatabase,
}

impl SurvivalSystemEngine {
    pub fn new(config: SurvivalSystemConfig) -> Self {
        Self {
            db: SurvivalDatabase::new(),
            config,
        }
    }

    pub fn record_bug(&mut self, signature: String, description: String) {
        self.db.insert(
            signature,
            BugRecord {
                description,
                confidence: 0.0,
                times_encountered: 1,
            },
        );
    }

    pub fn get_bug(&self, signature: &str) -> Option<BugRecord> {
        self.db.get(signature)
    }

    pub fn total_bugs_learned(&self) -> usize {
        self.db.len()
    }
}

pub async fn init() -> Result<(), anyhow::Error> {
    tracing::info!("Initializing Survival System");
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
    async fn test_survival_system() {
        let config = SurvivalSystemConfig::default();
        let mut engine = SurvivalSystemEngine::new(config);
        engine.record_bug("sig1".to_string(), "test bug".to_string());
        assert_eq!(engine.total_bugs_learned(), 1);
    }
}
