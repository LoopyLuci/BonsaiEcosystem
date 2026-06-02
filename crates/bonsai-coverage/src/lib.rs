//! Bonsai Test Coverage Infrastructure
//! Complete coverage management:
//! - Coverage data collection from tarpaulin
//! - Enforcement gates and thresholds
//! - Coverage trend tracking and reporting
//! - Per-crate and per-file analysis

pub mod collector;
pub mod enforcer;
pub mod reporting;
pub mod history;
pub mod integration;

pub use collector::{CoverageCollector, CoverageResult};
pub use enforcer::{CoverageEnforcer, CoverageGate};
pub use reporting::CoverageReporter;
pub use history::CoverageHistory;
pub use integration::CICoverageIntegration;

use serde::{Deserialize, Serialize};

/// Coverage metrics for a crate or file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    pub lines_covered: usize,
    pub lines_total: usize,
    pub branch_coverage_percent: f64,
    pub coverage_percent: f64,
}

impl CoverageMetrics {
    pub fn new(lines_covered: usize, lines_total: usize) -> Self {
        let coverage_percent = if lines_total == 0 {
            100.0
        } else {
            (lines_covered as f64 / lines_total as f64) * 100.0
        };

        Self {
            lines_covered,
            lines_total,
            branch_coverage_percent: 0.0,
            coverage_percent,
        }
    }

    pub fn meets_target(&self, target: f64) -> bool {
        self.coverage_percent >= target
    }

    pub fn regression_from(&self, baseline: f64) -> f64 {
        baseline - self.coverage_percent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_metrics() {
        let metrics = CoverageMetrics::new(80, 100);
        assert_eq!(metrics.coverage_percent, 80.0);
        assert!(metrics.meets_target(75.0));
        assert!(!metrics.meets_target(85.0));
    }

    #[test]
    fn test_regression_calculation() {
        let metrics = CoverageMetrics::new(80, 100);
        assert_eq!(metrics.regression_from(85.0), 5.0);
    }
}
