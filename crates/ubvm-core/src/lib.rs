/// UBVM Core Types and Traits
pub mod discovery;

use serde::{Serialize, Deserialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestId(pub Uuid);

impl TestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TestId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestJob {
    pub id: TestId,
    pub suite: String,
    pub case: String,
    pub input: serde_json::Value,
    pub expected: serde_json::Value,
    pub language: Option<String>,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: TestId,
    pub passed: bool,
    pub fidelity: f64,
    pub duration_ms: u64,
    pub output: String,
    pub error: Option<String>,
}

impl TestResult {
    pub fn success(id: TestId, output: String) -> Self {
        Self {
            id,
            passed: true,
            fidelity: 1.0,
            duration_ms: 0,
            output,
            error: None,
        }
    }

    pub fn error(id: TestId, error: String) -> Self {
        Self {
            id,
            passed: false,
            fidelity: 0.0,
            duration_ms: 0,
            output: String::new(),
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMetrics {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub total_duration_ms: u64,
}

pub fn parse_output(output: &str, id: TestId) -> TestResult {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(output) {
        let passed = v.get("passed").and_then(|p| p.as_bool()).unwrap_or(false);
        let fidelity = v.get("fidelity").and_then(|f| f.as_f64()).unwrap_or(0.0);
        TestResult {
            id,
            passed,
            fidelity,
            duration_ms: 0,
            output: output.to_string(),
            error: None,
        }
    } else {
        TestResult::error(id, "Invalid JSON output".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id1 = TestId::new();
        let id2 = TestId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_result_success() {
        let id = TestId::new();
        let result = TestResult::success(id, "output".into());
        assert!(result.passed);
        assert_eq!(result.fidelity, 1.0);
    }

    #[test]
    fn test_parse_output() {
        let id = TestId::new();
        let json = r#"{"passed": true, "fidelity": 0.95}"#;
        let result = parse_output(json, id);
        assert!(result.passed);
        assert_eq!(result.fidelity, 0.95);
    }
}
