/// Result Storage - AriaDB and Universe Integration Stubs
use serde::{Deserialize, Serialize};

/// A stored test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredResult {
    pub run_id: String,
    pub spec_name: String,
    pub test_case_name: String,
    pub language: String,
    pub passed: bool,
    pub fidelity: f64,
    pub actual_output: String,
    pub expected_output: String,
    pub execution_time_ms: u64,
    pub timestamp: String,
}

/// Result storage backend (stub for now, will be replaced with AriaDB client)
pub struct ResultStore {
    results: Vec<StoredResult>,
}

impl ResultStore {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Store a test result in memory (and eventually AriaDB)
    pub async fn store(&mut self, result: StoredResult) -> anyhow::Result<()> {
        tracing::info!(
            "Storing result: run={}, spec={}, lang={}, passed={}",
            result.run_id,
            result.spec_name,
            result.language,
            result.passed
        );
        self.results.push(result);
        // TODO: In production, insert into AriaDB here
        Ok(())
    }

    /// Get all results for a spec
    pub fn get_results_for_spec(&self, spec_name: &str) -> Vec<&StoredResult> {
        self.results
            .iter()
            .filter(|r| r.spec_name == spec_name)
            .collect()
    }

    /// Get all results for a language
    pub fn get_results_for_language(&self, lang: &str) -> Vec<&StoredResult> {
        self.results
            .iter()
            .filter(|r| r.language == lang)
            .collect()
    }

    /// Compute statistics for a spec
    pub fn compute_stats(&self, spec_name: &str) -> SpecStats {
        let results: Vec<_> = self.get_results_for_spec(spec_name);
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let avg_fidelity = if !results.is_empty() {
            results.iter().map(|r| r.fidelity).sum::<f64>() / results.len() as f64
        } else {
            0.0
        };
        let total_time_ms: u64 = results.iter().map(|r| r.execution_time_ms).sum();

        SpecStats {
            spec_name: spec_name.to_string(),
            total_tests: total,
            passed,
            failed: total - passed,
            success_rate: if total == 0 {
                0.0
            } else {
                (passed as f64 / total as f64) * 100.0
            },
            avg_fidelity,
            total_execution_time_ms: total_time_ms,
        }
    }

    /// Export results as JSON
    pub fn export_json(&self) -> serde_json::Value {
        serde_json::json!({
            "results": self.results,
            "count": self.results.len(),
        })
    }

    /// Export results as CSV (returns as string)
    pub fn export_csv(&self) -> String {
        let mut csv = "run_id,spec_name,test_case,language,passed,fidelity,execution_time_ms,timestamp\n".to_string();
        for result in &self.results {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                result.run_id,
                result.spec_name,
                result.test_case_name,
                result.language,
                result.passed,
                result.fidelity,
                result.execution_time_ms,
                result.timestamp,
            ));
        }
        csv
    }

    /// Clear all stored results
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

impl Default for ResultStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for a test spec run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecStats {
    pub spec_name: String,
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub success_rate: f64,
    pub avg_fidelity: f64,
    pub total_execution_time_ms: u64,
}

/// Log an event to Universe (stub)
pub async fn log_event_to_universe(event: &str) -> anyhow::Result<()> {
    tracing::info!("[Universe] {}", event);
    // TODO: In production, call bonsai_universe::log_event() here
    Ok(())
}

/// Store a BLAKE3 hash of test artifacts (for content-addressed storage)
pub async fn store_artifact_hash(hash: &str, _content: &[u8]) -> anyhow::Result<()> {
    tracing::debug!("[CAS] Stored artifact with hash: {}", hash);
    // TODO: In production, call bonsai_cas::store(hash, _content) here
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_result_store() {
        let mut store = ResultStore::new();
        let result = StoredResult {
            run_id: uuid::Uuid::new_v4().to_string(),
            spec_name: "TestSpec".to_string(),
            test_case_name: "case1".to_string(),
            language: "rust".to_string(),
            passed: true,
            fidelity: 1.0,
            actual_output: "5".to_string(),
            expected_output: "5".to_string(),
            execution_time_ms: 10,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        store.store(result).await.unwrap();
        assert_eq!(store.results.len(), 1);
    }

    #[test]
    fn test_stats_computation() {
        let mut store = ResultStore::new();
        for i in 0..3 {
            let result = StoredResult {
                run_id: format!("run-{}", i),
                spec_name: "TestSpec".to_string(),
                test_case_name: format!("case{}", i),
                language: "rust".to_string(),
                passed: i < 2,
                fidelity: 1.0,
                actual_output: "out".to_string(),
                expected_output: "out".to_string(),
                execution_time_ms: 10,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            store.results.push(result);
        }
        let stats = store.compute_stats("TestSpec");
        assert_eq!(stats.total_tests, 3);
        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 1);
    }

    #[test]
    fn test_csv_export() {
        let mut store = ResultStore::new();
        let result = StoredResult {
            run_id: "run1".to_string(),
            spec_name: "TestSpec".to_string(),
            test_case_name: "case1".to_string(),
            language: "rust".to_string(),
            passed: true,
            fidelity: 1.0,
            actual_output: "5".to_string(),
            expected_output: "5".to_string(),
            execution_time_ms: 10,
            timestamp: "2026-06-04T00:00:00Z".to_string(),
        };
        store.results.push(result);
        let csv = store.export_csv();
        assert!(csv.contains("run1"));
        assert!(csv.contains("TestSpec"));
    }
}
