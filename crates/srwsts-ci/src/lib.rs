//! # SRWSTS-CI: Software Regression & Workload Statistics Tracking Service (CI)
//!
//! Production-grade continuous integration regression detection system with:
//! - Baseline management and verification from CAS
//! - Comprehensive regression detection with configurable thresholds
//! - Performance metrics tracking with statistical analysis
//! - Multi-tier CI pipeline (smoke tests + full suite + nightly)
//! - Detailed report generation with trend analysis
//! - Artifact collection and deterministic replay
//! - Baseline approval workflow with audit trail
//! - Health dashboard and alerting integration
//! - Test priority adaptation with AI advisor suggestions

pub mod baseline;
pub mod detection;
pub mod metrics;
pub mod pipeline;
pub mod reporting;
pub mod artifacts;
pub mod approval;
pub mod dashboard;
pub mod alerts;
pub mod priority;
pub mod errors;

// Re-export public API
pub use baseline::{BaselineManager, BaselineVersion, BaselineIntegrity};
pub use detection::{RegressionDetector, RegressionFinding, RegressionSeverity};
pub use metrics::{PerformanceMetrics, MetricsSnapshot, MetricsBucket};
pub use pipeline::{CIPipeline, PipelineStage, PipelineConfig};
pub use reporting::{RegressionReport, ReportGenerator, MetricComparison};
pub use artifacts::{ArtifactCollector, TestArtifact, ArtifactMetadata};
pub use approval::{ApprovalManager, ApprovalDecision, AuditEntry};
pub use dashboard::{HealthDashboard, HealthMetrics};
pub use alerts::{AlertManager, AlertLevel, AlertEvent};
pub use priority::{PriorityAdvisor, TestPriority};
pub use errors::{CIError, CIResult};

/// SRWSTS-CI version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
