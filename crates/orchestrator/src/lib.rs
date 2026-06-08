//! System Orchestrator - Central coordination hub

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Central orchestrator for all system services
pub struct Orchestrator {
    services: Arc<RwLock<HashMap<String, ServiceHandle>>>,
    observability: Arc<ObservabilityHandle>,
}

pub struct ServiceHandle {
    pub name: String,
    pub status: ServiceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed(String),
}

pub struct ObservabilityHandle {
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            observability: Arc::new(ObservabilityHandle {
                metrics_enabled: true,
                tracing_enabled: true,
            }),
        }
    }

    pub async fn register_service(&self, name: String) -> anyhow::Result<()> {
        let mut services = self.services.write().await;
        services.insert(
            name.clone(),
            ServiceHandle {
                name,
                status: ServiceStatus::Starting,
            },
        );
        Ok(())
    }

    pub async fn update_service_status(&self, name: &str, status: ServiceStatus) -> anyhow::Result<()> {
        let mut services = self.services.write().await;
        if let Some(service) = services.get_mut(name) {
            service.status = status;
        }
        Ok(())
    }

    pub async fn get_service_status(&self, name: &str) -> Option<ServiceStatus> {
        self.services
            .read()
            .await
            .get(name)
            .map(|s| s.status.clone())
    }

    pub async fn list_services(&self) -> Vec<String> {
        self.services
            .read()
            .await
            .keys()
            .cloned()
            .collect()
    }

    pub async fn shutdown_all(&self) -> anyhow::Result<()> {
        let mut services = self.services.write().await;
        for service in services.values_mut() {
            service.status = ServiceStatus::Stopped;
        }
        Ok(())
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_lifecycle() {
        let orch = Orchestrator::new();

        orch.register_service("test-service".to_string()).await.unwrap();
        orch.update_service_status("test-service", ServiceStatus::Running).await.unwrap();

        assert_eq!(
            orch.get_service_status("test-service").await,
            Some(ServiceStatus::Running)
        );

        orch.shutdown_all().await.unwrap();
        assert_eq!(
            orch.get_service_status("test-service").await,
            Some(ServiceStatus::Stopped)
        );
    }

    #[tokio::test]
    async fn test_multiple_services() {
        let orch = Orchestrator::new();

        orch.register_service("service-1".to_string()).await.unwrap();
        orch.register_service("service-2".to_string()).await.unwrap();

        let services = orch.list_services().await;
        assert_eq!(services.len(), 2);
    }
}
