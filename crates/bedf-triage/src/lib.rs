//! Team G: Triage & AI
//!
//! Crash deduplication, AI explanation, and automatic fix generation.

pub mod interfaces;
pub mod config;
pub mod crash_dedup;
pub mod fix_generator;

pub use interfaces::*;
pub use config::TriageConfig;
pub use crash_dedup::{CrashDeduplicator, CrashSignature};
pub use fix_generator::{FixGenerator, GeneratedFix};

pub struct TriageEngine {
    config: TriageConfig,
    dedup: CrashDeduplicator,
    fix_gen: FixGenerator,
}

impl TriageEngine {
    pub fn new(config: TriageConfig) -> Self {
        Self {
            dedup: CrashDeduplicator::new(),
            fix_gen: FixGenerator::new(),
            config,
        }
    }

    pub async fn triage_crash(&mut self, stack_trace: &str) -> TriageResult {
        let signature = self.dedup.compute_signature(stack_trace);

        let is_duplicate = self.dedup.is_duplicate(&signature);

        let fix = if !is_duplicate && self.config.enable_ai_fixes {
            self.fix_gen.generate_fix(stack_trace)
        } else {
            None
        };

        TriageResult {
            crash_signature: signature,
            is_duplicate,
            suggested_fix: fix,
            confidence: if is_duplicate { 0.0 } else { 0.85 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TriageResult {
    pub crash_signature: String,
    pub is_duplicate: bool,
    pub suggested_fix: Option<GeneratedFix>,
    pub confidence: f64,
}

pub async fn init() -> Result<(), anyhow::Error> {
    tracing::info!("Initializing Triage Engine");
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
    async fn test_triage_engine() {
        let config = TriageConfig::default();
        let mut engine = TriageEngine::new(config);
        let result = engine.triage_crash("thread 'main' panicked at 'index out of bounds'").await;
        assert!(!result.crash_signature.is_empty());
    }
}
