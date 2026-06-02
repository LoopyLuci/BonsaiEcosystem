//! Bonsai Performance Profiler
//!
//! Comprehensive profiling infrastructure for production optimization:
//! - Flamegraph CPU profiling
//! - Memory allocation tracking
//! - Async overhead measurement
//! - Hot-path benchmarking
//! - Continuous performance monitoring
//! - Historical trend analysis

pub mod flamegraph;
pub mod allocation;
pub mod async_metrics;
pub mod hotpath;
pub mod benchmarks;
pub mod metrics;
pub mod trending;

pub use flamegraph::FlameGraphProfiler;
pub use allocation::AllocationTracker;
pub use async_metrics::AsyncMetrics;
pub use hotpath::HotPathDetector;
pub use metrics::{ProfiledOperation, OperationMetrics};
pub use trending::PerformanceTrend;

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Central profiler for all ecosystem operations
pub struct BonsaiProfiler {
    flamegraph: Arc<FlameGraphProfiler>,
    allocations: Arc<AllocationTracker>,
    async_metrics: Arc<AsyncMetrics>,
    hotpath_detector: Arc<HotPathDetector>,
    operations: Arc<RwLock<Vec<OperationMetrics>>>,
    trends: Arc<RwLock<Vec<PerformanceTrend>>>,
}

/// Profile snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSnapshot {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub latency_ms: f64,
    pub memory_mb: f64,
    pub allocations: u64,
    pub deallocations: u64,
    pub async_overhead_percent: f64,
    pub is_hot_path: bool,
}

/// Performance report for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub generated_at: DateTime<Utc>,
    pub snapshots: Vec<ProfileSnapshot>,
    pub hottest_paths: Vec<(String, f64)>,  // (path, time_percent)
    pub memory_trends: Vec<(String, f64)>,   // (operation, avg_mb)
    pub recommendations: Vec<String>,
}

impl BonsaiProfiler {
    pub fn new() -> Self {
        Self {
            flamegraph: Arc::new(FlameGraphProfiler::new()),
            allocations: Arc::new(AllocationTracker::new()),
            async_metrics: Arc::new(AsyncMetrics::new()),
            hotpath_detector: Arc::new(HotPathDetector::new()),
            operations: Arc::new(RwLock::new(Vec::new())),
            trends: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Profile a synchronous operation
    pub fn profile_sync<F, T>(&self, operation: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = std::time::Instant::now();
        let start_alloc = self.allocations.current_bytes();

        let result = f();

        let duration = start.elapsed();
        let alloc_delta = self.allocations.current_bytes() - start_alloc;

        let metrics = OperationMetrics {
            operation: operation.to_string(),
            latency_ms: duration.as_secs_f64() * 1000.0,
            memory_mb: alloc_delta as f64 / 1_000_000.0,
            timestamp: Utc::now(),
        };

        // Check if hot path
        if metrics.latency_ms > 10.0 {
            let _ = self.hotpath_detector.record(operation, metrics.latency_ms);
        }

        result
    }

    /// Profile an async operation
    pub async fn profile_async<F, T>(&self, operation: &str, f: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        let start = std::time::Instant::now();
        let result = f.await;
        let duration = start.elapsed();

        self.async_metrics.record(operation, duration.as_secs_f64());
        result
    }

    /// Generate performance report
    pub async fn generate_report(&self) -> PerformanceReport {
        let operations = self.operations.read().await.clone();

        let hottest_paths = self.hotpath_detector
            .get_hottest_paths(10)
            .into_iter()
            .map(|(path, time)| (path, time))
            .collect();

        let memory_trends = operations
            .iter()
            .fold(
                std::collections::HashMap::new(),
                |mut map, op| {
                    map.entry(op.operation.clone())
                        .and_modify(|sum: &mut (f64, usize)| {
                            sum.0 += op.memory_mb;
                            sum.1 += 1;
                        })
                        .or_insert((op.memory_mb, 1));
                    map
                },
            )
            .into_iter()
            .map(|(op, (sum, count))| (op, sum / count as f64))
            .collect();

        let mut recommendations = Vec::new();

        // Analyze hot paths
        for (path, time_percent) in &hottest_paths {
            if time_percent > &50.0 {
                recommendations.push(format!(
                    "Critical: {} takes {:.1}% of time - prioritize optimization",
                    path, time_percent
                ));
            }
        }

        // Analyze memory usage
        for (op, memory_mb) in &memory_trends {
            if memory_mb > &100.0 {
                recommendations.push(format!(
                    "Warning: {} uses {:.1}MB average - consider optimization",
                    op, memory_mb
                ));
            }
        }

        let snapshots = operations
            .iter()
            .map(|op| ProfileSnapshot {
                timestamp: op.timestamp,
                operation: op.operation.clone(),
                latency_ms: op.latency_ms,
                memory_mb: op.memory_mb,
                allocations: 0,      // Would come from allocation tracker
                deallocations: 0,     // Would come from allocation tracker
                async_overhead_percent: 0.0,  // Would come from async metrics
                is_hot_path: self.hotpath_detector.is_hot_path(&op.operation),
            })
            .collect();

        PerformanceReport {
            generated_at: Utc::now(),
            snapshots,
            hottest_paths,
            memory_trends,
            recommendations,
        }
    }

    /// Get all operations recorded
    pub async fn get_operations(&self) -> Vec<OperationMetrics> {
        self.operations.read().await.clone()
    }

    /// Clear recorded operations
    pub async fn clear_operations(&self) {
        self.operations.write().await.clear();
    }

    /// Export report as JSON
    pub async fn export_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let report = self.generate_report().await;
        let json = serde_json::to_string_pretty(&report)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl Default for BonsaiProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_creation() {
        let _profiler = BonsaiProfiler::new();
    }

    #[test]
    fn test_sync_profiling() {
        let profiler = BonsaiProfiler::new();
        let result = profiler.profile_sync("test_op", || 42);
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_async_profiling() {
        let profiler = BonsaiProfiler::new();
        let result = profiler.profile_async("test_async", async { 42 }).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_report_generation() {
        let profiler = BonsaiProfiler::new();
        profiler.profile_sync("op1", || std::thread::sleep(std::time::Duration::from_millis(10)));
        let report = profiler.generate_report().await;
        assert!(!report.snapshots.is_empty());
    }
}
