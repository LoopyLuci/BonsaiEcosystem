/// Planet-Scale UBVM Mesh – Distributed test execution
pub mod worker;
pub mod coordinator;

use ubvm_core::{TestJob, TestResult};
use std::sync::Arc;

/// Mesh configuration
#[derive(Debug, Clone)]
pub struct MeshConfig {
    pub max_workers: usize,
    pub worker_timeout_secs: u64,
    pub enable_redistribution: bool,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            max_workers: 10,
            worker_timeout_secs: 300,
            enable_redistribution: true,
        }
    }
}

/// Mesh status
#[derive(Debug, Clone)]
pub struct MeshStatus {
    pub active_workers: usize,
    pub total_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
}

/// Main mesh coordinator
pub struct Mesh {
    config: MeshConfig,
    workers: dashmap::DashMap<String, worker::WorkerState>,
    job_queue: std::sync::Arc<tokio::sync::Mutex<Vec<TestJob>>>,
    results: dashmap::DashMap<ubvm_core::TestId, TestResult>,
}

impl Mesh {
    pub fn new(config: MeshConfig) -> Self {
        Self {
            config,
            workers: dashmap::DashMap::new(),
            job_queue: Arc::new(tokio::sync::Mutex::new(Vec::new())),
            results: dashmap::DashMap::new(),
        }
    }

    /// Register a worker with the mesh
    pub async fn register_worker(&self, id: &str, capabilities: Vec<String>) -> anyhow::Result<()> {
        self.workers.insert(
            id.to_string(),
            worker::WorkerState {
                id: id.to_string(),
                capabilities,
                healthy: true,
                assigned_jobs: 0,
            },
        );
        Ok(())
    }

    /// Get mesh status
    pub fn status(&self) -> MeshStatus {
        let completed = self.results.len();
        let total = self.results.len(); // simplified
        MeshStatus {
            active_workers: self.workers.len(),
            total_jobs: total,
            completed_jobs: completed,
            failed_jobs: 0,
        }
    }

    /// Get available workers
    pub fn get_available_workers(&self) -> Vec<String> {
        self.workers
            .iter()
            .filter(|entry| entry.value().healthy)
            .map(|entry| entry.key().clone())
            .collect()
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new(MeshConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mesh_creation() {
        let mesh = Mesh::new(MeshConfig::default());
        assert_eq!(mesh.get_available_workers().len(), 0);
    }

    #[tokio::test]
    async fn test_worker_registration() {
        let mesh = Mesh::new(MeshConfig::default());
        mesh.register_worker("worker-1", vec!["rust".into(), "python".into()])
            .await
            .unwrap();
        assert_eq!(mesh.get_available_workers().len(), 1);
    }

    #[test]
    fn test_mesh_status() {
        let mesh = Mesh::new(MeshConfig::default());
        let status = mesh.status();
        assert_eq!(status.active_workers, 0);
    }
}
