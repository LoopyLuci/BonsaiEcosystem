//! Universal Validation Mesh (UVM)
//!
//! Phase 7: Cross-language validation with deterministic output comparison

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Canonical test case from OpenCV test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalTest {
    pub id: String,
    pub operation: String,
    pub seed: u64,
    pub inputs: TestInputs,
    pub expected_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestInputs {
    pub width: usize,
    pub height: usize,
    pub channels: u8,
    pub data: Vec<u8>,
}

impl CanonicalTest {
    pub fn new(operation: String, width: usize, height: usize, channels: u8) -> Self {
        CanonicalTest {
            id: Uuid::new_v4().to_string(),
            operation,
            seed: 12345,
            inputs: TestInputs {
                width,
                height,
                channels,
                data: vec![0u8; width * height * channels as usize],
            },
            expected_hash: String::new(),
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_expected_hash(mut self, hash: String) -> Self {
        self.expected_hash = hash;
        self
    }
}

/// Test result for a single implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_id: String,
    pub implementation: String,
    pub passed: bool,
    pub output_hash: String,
    pub expected_hash: String,
    pub duration_ms: u64,
    pub error: Option<String>,
}

/// Validation report across all implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub timestamp: u64,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub results: Vec<TestResult>,
    pub implementations_tested: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        ValidationReport {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            results: Vec::new(),
            implementations_tested: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        if result.passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
        }
        self.total_tests += 1;

        if !self.implementations_tested.contains(&result.implementation) {
            self.implementations_tested.push(result.implementation.clone());
        }

        self.results.push(result);
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64) / (self.total_tests as f64)
        }
    }

    pub fn is_fully_passing(&self) -> bool {
        self.failed_tests == 0 && self.total_tests > 0
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal Validation Mesh coordinator
pub struct ValidationMesh {
    test_suite: Vec<CanonicalTest>,
    reports: HashMap<String, ValidationReport>,
}

impl ValidationMesh {
    pub fn new() -> Self {
        ValidationMesh {
            test_suite: Vec::new(),
            reports: HashMap::new(),
        }
    }

    pub fn load_test_suite(mut self, tests: Vec<CanonicalTest>) -> Self {
        self.test_suite = tests;
        self
    }

    pub fn add_test(&mut self, test: CanonicalTest) {
        self.test_suite.push(test);
    }

    pub fn test_count(&self) -> usize {
        self.test_suite.len()
    }

    pub fn get_test(&self, index: usize) -> Option<&CanonicalTest> {
        self.test_suite.get(index)
    }

    pub fn record_validation(&mut self, session_id: String, report: ValidationReport) {
        self.reports.insert(session_id, report);
    }

    pub fn get_validation_report(&self, session_id: &str) -> Option<&ValidationReport> {
        self.reports.get(session_id)
    }

    pub fn canonical_test_suite() -> Vec<CanonicalTest> {
        vec![
            CanonicalTest::new("gaussian_blur".to_string(), 100, 100, 3)
                .with_expected_hash("abc123def456".to_string()),
            CanonicalTest::new("canny".to_string(), 100, 100, 1)
                .with_expected_hash("def456ghi789".to_string()),
            CanonicalTest::new("resize".to_string(), 100, 100, 3)
                .with_expected_hash("ghi789jkl012".to_string()),
            CanonicalTest::new("cvt_color".to_string(), 100, 100, 3)
                .with_expected_hash("jkl012mno345".to_string()),
            CanonicalTest::new("blur".to_string(), 100, 100, 3)
                .with_expected_hash("mno345pqr678".to_string()),
        ]
    }
}

impl Default for ValidationMesh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_test_creation() {
        let test = CanonicalTest::new("blur".to_string(), 100, 100, 3);
        assert_eq!(test.operation, "blur");
        assert_eq!(test.inputs.width, 100);
        assert_eq!(test.inputs.channels, 3);
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        assert_eq!(report.total_tests, 0);
        assert_eq!(report.success_rate(), 0.0);

        report.add_result(TestResult {
            test_id: "test_1".to_string(),
            implementation: "rust".to_string(),
            passed: true,
            output_hash: "abc123".to_string(),
            expected_hash: "abc123".to_string(),
            duration_ms: 10,
            error: None,
        });

        assert_eq!(report.total_tests, 1);
        assert_eq!(report.passed_tests, 1);
        assert_eq!(report.success_rate(), 1.0);
        assert!(report.is_fully_passing());
    }

    #[test]
    fn test_validation_mesh() {
        let mesh = ValidationMesh::new()
            .load_test_suite(ValidationMesh::canonical_test_suite());

        assert_eq!(mesh.test_count(), 5);
        assert!(mesh.get_test(0).is_some());
    }

    #[test]
    fn test_canonical_suite_has_5_core_operations() {
        let suite = ValidationMesh::canonical_test_suite();
        assert_eq!(suite.len(), 5);

        let operations: Vec<_> = suite.iter().map(|t| &t.operation).collect();
        assert!(operations.contains(&&"gaussian_blur".to_string()));
        assert!(operations.contains(&&"canny".to_string()));
        assert!(operations.contains(&&"resize".to_string()));
        assert!(operations.contains(&&"cvt_color".to_string()));
        assert!(operations.contains(&&"blur".to_string()));
    }

    #[test]
    fn test_validation_report_mixed_results() {
        let mut report = ValidationReport::new();

        report.add_result(TestResult {
            test_id: "test_1".to_string(),
            implementation: "rust".to_string(),
            passed: true,
            output_hash: "abc".to_string(),
            expected_hash: "abc".to_string(),
            duration_ms: 10,
            error: None,
        });

        report.add_result(TestResult {
            test_id: "test_2".to_string(),
            implementation: "titan".to_string(),
            passed: false,
            output_hash: "xyz".to_string(),
            expected_hash: "abc".to_string(),
            duration_ms: 15,
            error: Some("Hash mismatch".to_string()),
        });

        assert_eq!(report.total_tests, 2);
        assert_eq!(report.passed_tests, 1);
        assert_eq!(report.failed_tests, 1);
        assert!(!report.is_fully_passing());
        assert!(report.success_rate() > 0.4 && report.success_rate() < 0.6);
    }
}
