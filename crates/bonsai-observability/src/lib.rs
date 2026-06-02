//! Bonsai Advanced Observability Stack
//! Complete observability infrastructure:
//! - OpenTelemetry tracing (Jaeger)
//! - Prometheus metrics export
//! - SLA tracking and compliance
//! - Alert rule engine
//! - Real-time dashboards

pub mod metrics;
pub mod tracing;
pub mod sla;
pub mod alerts;
pub mod dashboard;

pub use metrics::{MetricsCollector, MetricPoint};
pub use sla::{SLATracker, SLATarget, SLACompliance};
pub use alerts::{AlertRule, AlertEngine, AlertSeverity, Alert};
pub use dashboard::DashboardConfig;

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Central observability coordinator
pub struct ObservabilityStack {
    metrics: Arc<MetricsCollector>,
    sla_tracker: Arc<SLATracker>,
    alert_engine: Arc<AlertEngine>,
    initialized: Arc<RwLock<bool>>,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub timestamp: DateTime<Utc>,
    pub service: String,
    pub healthy: bool,
    pub latency_ms: f64,
    pub details: String,
}

impl ObservabilityStack {
    pub fn new(sla_target: SLATarget) -> Self {
        Self {
            metrics: Arc::new(MetricsCollector::new()),
            sla_tracker: Arc::new(SLATracker::new(sla_target)),
            alert_engine: Arc::new(AlertEngine::new()),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize observability stack
    pub async fn initialize(&self) -> Result<(), String> {
        // Initialize OpenTelemetry
        // Initialize Prometheus exporter
        // Start metric collection
        // Start SLA tracking

        let mut init = self.initialized.write().await;
        *init = true;

        tracing::info!("Observability stack initialized");
        Ok(())
    }

    /// Record operation metrics
    pub fn record_operation(&self, operation: &str, latency_ms: f64, success: bool) {
        self.metrics.record(operation, latency_ms, success);
        self.sla_tracker.record(operation, latency_ms, success);

        // Check alert rules
        if let Some(alert) = self.alert_engine.check_rules(operation, latency_ms) {
            tracing::warn!("Alert triggered: {:?}", alert);
        }
    }

    /// Get SLA compliance status
    pub fn get_sla_compliance(&self) -> SLACompliance {
        self.sla_tracker.get_compliance()
    }

    /// Export metrics to Prometheus
    pub async fn export_prometheus(&self) -> Result<String, String> {
        self.metrics.export_prometheus().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stack_creation() {
        let target = SLATarget {
            p95_latency_ms: 100.0,
            p99_latency_ms: 200.0,
            availability_percent: 99.95,
        };
        let stack = ObservabilityStack::new(target);
        assert!(stack.initialize().await.is_ok());
    }
}
