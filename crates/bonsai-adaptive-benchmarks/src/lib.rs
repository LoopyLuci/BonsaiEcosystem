pub mod correctness;
pub mod performance;
pub mod regression;
pub mod unit_tests;
pub mod test_fixtures;
pub mod benchmarking;
pub mod formal_verification;
pub mod observability;

pub use correctness::{CorrectnessTest, CorrectnessResult};
pub use performance::{PerformanceBenchmark, PerformanceMetrics};
pub use regression::{RegressionDetector, RegressionReport};
pub use test_fixtures::{TestDataset, TestFixture};
pub use benchmarking::{BenchmarkRunner, BenchmarkConfig};
pub use formal_verification::{FormalVerifier, VerificationResult};
pub use observability::{MetricsCollector, BenchmarkLogger};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Central configuration for the entire benchmarking framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveBenchmarkConfig {
    /// Model scale ranges: (min, max, step)
    pub scales: Vec<(u32, u32, u32)>,
    /// Batch sizes to test
    pub batch_sizes: Vec<usize>,
    /// Sequence lengths to test
    pub sequence_lengths: Vec<usize>,
    /// Device targets: cpu, gpu, tpu
    pub device_targets: Vec<String>,
    /// Number of samples for statistical significance
    pub num_samples: usize,
    /// Regression threshold (percentage)
    pub regression_threshold_pct: f32,
    /// Enable distributed testing
    pub distributed: bool,
    /// Baseline version to compare against
    pub baseline_version: String,
    /// Output directory for results
    pub output_dir: String,
}

impl Default for AdaptiveBenchmarkConfig {
    fn default() -> Self {
        Self {
            scales: vec![
                (100_000_000, 100_000_000, 1),       // 100M
                (500_000_000, 500_000_000, 1),       // 500M
                (1_000_000_000, 1_000_000_000, 1),   // 1B
                (7_000_000_000, 7_000_000_000, 1),   // 7B
                (30_000_000_000, 30_000_000_000, 1), // 30B
                (100_000_000_000, 100_000_000_000, 1), // 100B
            ],
            batch_sizes: vec![1, 8, 32, 128],
            sequence_lengths: vec![128, 512, 2048, 4096],
            device_targets: vec!["cpu".to_string(), "gpu".to_string()],
            num_samples: 100,
            regression_threshold_pct: 5.0,
            distributed: false,
            baseline_version: "v0.1.0".to_string(),
            output_dir: "./benchmark_results".to_string(),
        }
    }
}

/// Comprehensive test result combining all aspects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveTestResult {
    pub timestamp: String,
    pub version: String,
    pub correctness: CorrectnessResult,
    pub performance: PerformanceMetrics,
    pub regressions: RegressionReport,
    pub formal_verification: VerificationResult,
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AdaptiveBenchmarkConfig::default();
        assert_eq!(config.scales.len(), 6);
        assert_eq!(config.regression_threshold_pct, 5.0);
        assert!(!config.distributed);
    }

    #[test]
    fn test_scales_are_ordered() {
        let config = AdaptiveBenchmarkConfig::default();
        for i in 0..config.scales.len() - 1 {
            assert!(
                config.scales[i].0 <= config.scales[i + 1].0,
                "Scales should be ordered"
            );
        }
    }
}
