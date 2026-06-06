//! Bonsai KDB Sync - Knowledge Database Integration
//!
//! Synchronizes rule metrics across deployments and aggregates them for cross-project learning.
//! Enables rule variants (different performance in different domains) and consensus scoring.

pub mod aggregator;
pub mod metrics;

pub use aggregator::{RuleMetricsAggregator, AggregatedMetrics};
pub use metrics::{RuleMetric, MetricsCollector};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Configuration for KDB sync.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdbConfig {
    /// Central KDB service URL (e.g., "https://kdb.bonsai.ai/api")
    pub service_url: String,

    /// Local cache directory for .kmod files
    pub cache_dir: PathBuf,

    /// Sync interval (hours)
    pub sync_interval_hours: u32,

    /// Whether to upload metrics to central service
    pub upload_enabled: bool,

    /// Anonymization key (salt for hashing identifiable info)
    pub anonymization_key: String,
}

impl Default for KdbConfig {
    fn default() -> Self {
        Self {
            service_url: "https://kdb.bonsai.ai/api".to_string(),
            cache_dir: PathBuf::from(".bonsai/kdb"),
            sync_interval_hours: 24,
            upload_enabled: true,
            anonymization_key: "bonsai-default-salt".to_string(),
        }
    }
}

/// Knowledge Database client for syncing metrics.
pub struct KdbClient {
    config: KdbConfig,
    aggregator: Arc<RuleMetricsAggregator>,
}

impl KdbClient {
    pub fn new(config: KdbConfig, aggregator: Arc<RuleMetricsAggregator>) -> Self {
        Self { config, aggregator }
    }

    /// Download latest rule-performance.kmod from KDB.
    pub async fn download_kdb_snapshot(&self, snapshot_id: &str) -> Result<KdbSnapshot> {
        tracing::info!("Downloading KDB snapshot: {}", snapshot_id);

        // TODO: Implement actual HTTP download from KDB service
        // For now, return a placeholder

        Ok(KdbSnapshot {
            snapshot_id: snapshot_id.to_string(),
            generated_at: Utc::now(),
            rule_metrics: HashMap::new(),
            project_count: 0,
            languages: vec![],
        })
    }

    /// Upload local metrics to KDB (anonymized).
    pub async fn upload_metrics(
        &self,
        metrics: Vec<RuleMetric>,
    ) -> Result<()> {
        if !self.config.upload_enabled {
            tracing::debug!("Metrics upload disabled");
            return Ok(());
        }

        tracing::info!("Uploading {} metrics to KDB", metrics.len());

        // TODO: Implement actual HTTP upload to KDB service
        // Data should be anonymized before sending:
        // - No user IDs, file paths, or code snippets
        // - Only aggregated metrics and rule performance data

        Ok(())
    }

    /// Get aggregated metrics for a rule across all projects.
    pub async fn get_rule_metrics(&self, rule_id: &str) -> Result<Option<AggregatedMetrics>> {
        self.aggregator.get_aggregated_metrics(rule_id).await
    }

    /// Sync with KDB (download + upload in one call).
    pub async fn sync(&self) -> Result<SyncResult> {
        let start = std::time::Instant::now();

        // Download latest KDB snapshot
        let snapshot = self
            .download_kdb_snapshot("latest")
            .await
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to download KDB snapshot: {}", e);
                KdbSnapshot::default()
            });

        // Upload local metrics
        // (In real deployment, would collect from Phase A ETL)
        let metrics = vec![];
        self.upload_metrics(metrics).await.ok();

        let duration = start.elapsed();
        Ok(SyncResult {
            snapshot_id: snapshot.snapshot_id,
            rules_updated: snapshot.rule_metrics.len(),
            upload_successful: true,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }
}

/// KDB snapshot (rule-performance.kmod equivalent).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KdbSnapshot {
    pub snapshot_id: String,
    pub generated_at: DateTime<Utc>,
    pub rule_metrics: HashMap<String, AggregatedMetrics>,
    pub project_count: usize,
    pub languages: Vec<String>,
}

/// Result of a KDB sync operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub snapshot_id: String,
    pub rules_updated: usize,
    pub upload_successful: bool,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdb_config_default() {
        let config = KdbConfig::default();
        assert!(!config.service_url.is_empty());
        assert!(config.upload_enabled);
    }

    #[tokio::test]
    async fn test_kdb_client_creation() {
        let config = KdbConfig::default();
        let aggregator = Arc::new(RuleMetricsAggregator::new());
        let client = KdbClient::new(config, aggregator);

        // Just verify it can be created
        assert!(!client.config.service_url.is_empty());
    }
}
