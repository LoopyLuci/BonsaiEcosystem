//! Bonsai EternalTrainingLoop (ETL) - Self-improving rules system
//!
//! ETL processes user feedback to continuously improve linting rules:
//! 1. Collect feedback events (accept, reject, dismiss)
//! 2. Calculate rule confidence scores
//! 3. Adjust rule severities dynamically
//! 4. Propose and refine AI-generated rules
//! 5. Store metrics in KDB for cross-project learning

pub mod feedback;
pub mod confidence;
pub mod adjuster;
pub mod refiner;
pub mod storage;
pub mod events;
pub mod lint_integration;
pub mod universe_bridge;

#[cfg(feature = "sqlx")]
pub mod storage_sqlx;

pub use feedback::FeedbackCollector;
pub use confidence::{RuleConfidenceMetrics, RuleConfidenceCalculator};
pub use adjuster::RuleConfidenceAdjuster;
pub use refiner::{RuleRefiner, RuleMutationProposal};
pub use storage::ETLStorage;
pub use events::{FeedbackEvent, FeedbackEventType, UniverseEventEmitter, UniverseEvent};
pub use crate::{RuleConfidenceUpdate, ETLCycleResult};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Main EternalTrainingLoop orchestrator
pub struct EternalTrainingLoop {
    storage: Arc<ETLStorage>,
    calculator: Arc<RuleConfidenceCalculator>,
    adjuster: Arc<RuleConfidenceAdjuster>,
    refiner: Arc<RuleRefiner>,
    event_emitter: Arc<UniverseEventEmitter>,
}

impl EternalTrainingLoop {
    pub fn new(
        storage: Arc<ETLStorage>,
        calculator: Arc<RuleConfidenceCalculator>,
        adjuster: Arc<RuleConfidenceAdjuster>,
        refiner: Arc<RuleRefiner>,
        event_emitter: Arc<UniverseEventEmitter>,
    ) -> Self {
        Self {
            storage,
            calculator,
            adjuster,
            refiner,
            event_emitter,
        }
    }

    /// Run the complete ETL pipeline (typically nightly)
    pub async fn run_cycle(&self) -> anyhow::Result<ETLCycleResult> {
        tracing::info!("Starting EternalTrainingLoop cycle");
        let start = std::time::Instant::now();

        // Stage 1: Collect feedback from last 24 hours
        tracing::info!("Stage 1: Collecting feedback events");
        let feedback_events = self
            .storage
            .get_feedback_events_since(Utc::now() - chrono::Duration::days(1))
            .await?;

        let feedback_count = feedback_events.len();
        tracing::info!("Collected {} feedback events", feedback_count);

        // Stage 2: Aggregate metrics per rule
        tracing::info!("Stage 2: Aggregating metrics");
        let metrics = self.calculator.aggregate_metrics(&feedback_events).await?;

        let rules_analyzed = metrics.len();
        tracing::info!("Analyzed metrics for {} rules", rules_analyzed);

        // Stage 3: Calculate confidence and recommend actions
        tracing::info!("Stage 3: Calculating confidence scores");
        let mut confidence_updates = Vec::new();
        for (rule_id, metric) in &metrics {
            let confidence = self.calculator.calculate_confidence(metric)?;
            let action = self.calculator.recommend_action(confidence)?;

            confidence_updates.push(RuleConfidenceUpdate {
                rule_id: rule_id.clone(),
                old_confidence: 0.0, // TODO: fetch from registry
                new_confidence: confidence,
                action: action.clone(),
                true_positives: metric.true_positives,
                false_positives: metric.false_positives,
                dismissed_count: metric.dismissed_count,
                timestamp: Utc::now(),
            });
        }

        // Stage 4: Apply confidence updates
        tracing::info!("Stage 4: Applying confidence updates");
        for update in &confidence_updates {
            self.adjuster.apply_update(update).await?;

            // Emit event to Universe
            self.event_emitter.emit_confidence_update(update).await?;
        }

        // Stage 5: Refine AI-generated rules
        tracing::info!("Stage 5: Refining AI-generated rules");
        let refinement_proposals = self
            .refiner
            .propose_refinements(&metrics)
            .await?;

        tracing::info!(
            "Proposed {} rule refinements",
            refinement_proposals.len()
        );

        for proposal in &refinement_proposals {
            self.event_emitter.emit_mutation_proposal(proposal).await?;
        }

        // Stage 6: Store metrics for persistence
        tracing::info!("Stage 6: Storing metrics in KDB");
        self.storage.store_metrics(&metrics).await?;

        let duration = start.elapsed();
        tracing::info!("EternalTrainingLoop cycle completed in {:?}", duration);

        Ok(ETLCycleResult {
            feedback_events_processed: feedback_count,
            rules_analyzed,
            confidence_updates_applied: confidence_updates.len(),
            refinement_proposals: refinement_proposals.len(),
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfidenceUpdate {
    pub rule_id: String,
    pub old_confidence: f32,
    pub new_confidence: f32,
    pub action: String,
    pub true_positives: u32,
    pub false_positives: u32,
    pub dismissed_count: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETLCycleResult {
    pub feedback_events_processed: usize,
    pub rules_analyzed: usize,
    pub confidence_updates_applied: usize,
    pub refinement_proposals: usize,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etl_creation() {
        // Placeholder - full tests in submodules
        assert!(true);
    }
}
