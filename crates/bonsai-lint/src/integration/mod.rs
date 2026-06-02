pub mod mcp_tools;
pub mod universe;
pub mod bug_hunt_feed;
pub mod bug_hunt_orchestrator;
pub mod cli;
pub mod survival_feedback;

use crate::diagnostics::Diagnostic;
use anyhow::Result;

/// Integration hooks for various Bonsai systems.
pub struct BonsaiIntegration;

impl BonsaiIntegration {
    /// Emit a lint result to the Universe event bus.
    pub async fn emit_to_universe(diagnostics: &[Diagnostic]) -> Result<()> {
        // TODO: Integrate with bonsai-universe crate
        tracing::info!("Emitting {} diagnostics to Universe", diagnostics.len());
        Ok(())
    }

    /// Feed diagnostics to the Bug Hunt system.
    pub async fn feed_to_bug_hunt(diagnostics: &[Diagnostic]) -> Result<()> {
        // TODO: Integrate with bonsai-bug-hunt crate
        tracing::info!("Feeding {} diagnostics to Bug Hunt", diagnostics.len());
        Ok(())
    }

    /// Emit metrics to observability system.
    pub async fn emit_metrics(metrics: &LintMetrics) -> Result<()> {
        // TODO: Send to telemetry/observability backend
        tracing::info!("Emitting lint metrics: {:?}", metrics);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LintMetrics {
    pub duration_ms: u128,
    pub files_scanned: usize,
    pub total_diagnostics: usize,
    pub diagnostics_by_severity: std::collections::HashMap<String, usize>,
}
