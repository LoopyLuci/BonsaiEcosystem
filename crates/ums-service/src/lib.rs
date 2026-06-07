//! Phase 3: UMS Service Integration
//! Enables service manifest discovery and management via Universal Module System

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Service manifest for UMS integration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceManifest {
    pub name: String,
    pub version: String,
    pub binary_hash: String,
    pub capabilities_required: Vec<String>,
    pub resources: ResourceSpec,
    pub idle_timeout_secs: u32,
    pub archive_after_hours: u32,
}

/// Resource specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub memory_mb: u32,
    pub cpu_cores: f32,
    pub cpu_percent_max: u32,
    pub iops_limit: u32,
}

/// Service registry backed by UMS
pub struct ServiceRegistry {
    manifests: Arc<DashMap<String, ServiceManifest>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            manifests: Arc::new(DashMap::new()),
        }
    }

    pub fn register(&self, manifest: ServiceManifest) -> Result<(), String> {
        if manifest.name.is_empty() {
            return Err("Service name required".to_string());
        }
        self.manifests.insert(manifest.name.clone(), manifest);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<ServiceManifest> {
        self.manifests.get(name).map(|entry| entry.clone())
    }

    pub fn list(&self) -> Vec<String> {
        self.manifests.iter().map(|entry| entry.key().clone()).collect()
    }

    pub fn unregister(&self, name: &str) -> Result<(), String> {
        self.manifests.remove(name)
            .ok_or_else(|| format!("Service {} not found", name))?;
        Ok(())
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_get() {
        let registry = ServiceRegistry::new();
        let manifest = ServiceManifest {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            binary_hash: "hash123".to_string(),
            capabilities_required: vec!["USB".to_string()],
            resources: ResourceSpec {
                memory_mb: 512,
                cpu_cores: 1.0,
                cpu_percent_max: 50,
                iops_limit: 1000,
            },
            idle_timeout_secs: 300,
            archive_after_hours: 24,
        };

        assert!(registry.register(manifest.clone()).is_ok());
        assert!(registry.get("test-service").is_some());
        assert!(registry.unregister("test-service").is_ok());
    }
}
