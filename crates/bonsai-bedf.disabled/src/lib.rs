//! Core BEDF Orchestrator
//!
//! The Brute-Force Error & Debugger Finder (BEDF) system coordinates 7 core analysis engines:
//! 1. Fuzzing Engine (libFuzzer/AFL++)
//! 2. Concurrency Testing (loom/shuttle)
//! 3. Memory Sanitizers (ASAN/MSAN/TSAN/LSAN)
//! 4. Property-Based Testing (proptest)
//! 5. Penetration Testing (OWASP ZAP)
//! 6. Sandbox Orchestration (Sanctum vaults)
//! 7. Triage & AI (crash analysis + auto-fixes)

pub mod interfaces;
pub mod config;
pub mod orchestrator;
pub mod metrics;
pub mod ci_orchestrator;
pub mod ecosystem_integration;
pub mod unified_commands;
pub mod bonsai_bot;

pub use interfaces::*;
pub use orchestrator::{BEDFOrchestrator, OrchestrationConfig};
pub use metrics::MetricsCollector;
pub use ci_orchestrator::{BonsaiCIOrchestrator, CIOrchestratorConfig, CIPipeline, TeamJob, PipelineStatus};
pub use ecosystem_integration::{BonsaiEcosystemOrchestrator, EcosystemConfig, SystemStatus, UnifiedMetrics, EventBus};
pub use unified_commands::{UnifiedCommand, UnifiedCommandHandler, CommandResult};
pub use bonsai_bot::{BonsaiBot, BotTask, BotCapabilities, IntelligenceLevel, AutonomyLevel, AutomationType, TaskStatus};

use std::sync::Arc;
use dashmap::DashMap;
use uuid::Uuid;

/// Global orchestrator state
pub struct BEDFState {
    pub id: String,
    pub components: DashMap<String, bool>,
    pub metrics: Arc<MetricsCollector>,
    pub active_runs: DashMap<String, RunInfo>,
}

#[derive(Clone, Debug)]
pub struct RunInfo {
    pub run_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: RunStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

impl BEDFState {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            components: DashMap::new(),
            metrics: Arc::new(MetricsCollector::new()),
            active_runs: DashMap::new(),
        }
    }

    pub fn register_component(&self, name: impl Into<String>, ready: bool) {
        self.components.insert(name.into(), ready);
    }

    pub fn start_run(&self) -> String {
        let run_id = Uuid::new_v4().to_string();
        self.active_runs.insert(
            run_id.clone(),
            RunInfo {
                run_id: run_id.clone(),
                started_at: chrono::Utc::now(),
                status: RunStatus::Running,
            },
        );
        run_id
    }

    pub fn complete_run(&self, run_id: &str) {
        if let Some(mut run) = self.active_runs.get_mut(run_id) {
            run.status = RunStatus::Completed;
        }
    }

    pub fn fail_run(&self, run_id: &str, reason: impl Into<String>) {
        if let Some(mut run) = self.active_runs.get_mut(run_id) {
            run.status = RunStatus::Failed(reason.into());
        }
    }
}

impl Default for BEDFState {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize BEDF orchestrator with all components
pub async fn init() -> Result<BEDFState, anyhow::Error> {
    tracing::info!("Initializing BEDF Orchestrator");

    let state = BEDFState::new();

    // Register component slots
    state.register_component("fuzzing", false);
    state.register_component("concurrency", false);
    state.register_component("sanitizers", false);
    state.register_component("property", false);
    state.register_component("pentest", false);
    state.register_component("sandbox", false);
    state.register_component("triage", false);

    tracing::info!("BEDF Orchestrator initialized: {}", state.id);
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialization() {
        let state = init().await;
        assert!(state.is_ok());
    }

    #[tokio::test]
    async fn test_state_creation() {
        let state = BEDFState::new();
        assert!(!state.id.is_empty());
        assert_eq!(state.components.len(), 0);
    }

    #[tokio::test]
    async fn test_register_component() {
        let state = BEDFState::new();
        state.register_component("test_component", true);
        assert!(state.components.contains_key("test_component"));
    }

    #[tokio::test]
    async fn test_run_lifecycle() {
        let state = BEDFState::new();
        let run_id = state.start_run();
        assert!(state.active_runs.contains_key(&run_id));

        state.complete_run(&run_id);
        let run = state.active_runs.get(&run_id).unwrap();
        assert_eq!(run.status, RunStatus::Completed);
    }

    #[tokio::test]
    async fn test_fail_run() {
        let state = BEDFState::new();
        let run_id = state.start_run();
        state.fail_run(&run_id, "test failure");

        let run = state.active_runs.get(&run_id).unwrap();
        match &run.status {
            RunStatus::Failed(reason) => assert_eq!(reason, "test failure"),
            _ => panic!("Expected Failed status"),
        }
    }
}
