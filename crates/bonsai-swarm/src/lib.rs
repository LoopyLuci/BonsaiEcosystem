//! `bonsai-swarm` — Hierarchical multi-agent swarm orchestration.
//!
//! # Architecture
//!
//! ```text
//! SwarmRegistry (global)
//!   └── SwarmOrchestrator (per swarm)
//!         ├── TaskDag            — dependency graph of work units
//!         ├── AgentHierarchy     — PM → Manager → SubManager → Agent → SubAgent
//!         ├── SwarmLedger        — append-only audit trail
//!         ├── CapabilityRegistry — agent skill advertisement & matching
//!         └── AssistantRegistry  — per-agent learning advisors
//! ```

pub mod assistant;
pub mod dag;
pub mod hierarchy;
pub mod ledger;
pub mod orchestrator;
pub mod registry;
pub mod role;

pub use assistant::{AgentAssistant, AssistantRegistry, AssistantSuggestion};
pub use dag::{DagError, TaskDag, TaskNode, TaskStatus};
pub use hierarchy::{AgentHierarchy, HierarchyNode, HierarchyStats, NodeStatus};
pub use ledger::{LedgerEntry, LedgerEventKind, SwarmLedger};
pub use orchestrator::{SwarmCommand, SwarmOrchestrator, SwarmRegistry, SwarmSnapshot, SwarmSpec, SwarmStatus};
pub use registry::{AgentMatch, CapabilityQuery, CapabilityRegistry};
pub use role::{AgentProfile, Capability, SwarmRole};

/// Re-export for convenience in bonsai-workspace src-tauri.
pub use orchestrator::SwarmRegistry as Registry;
