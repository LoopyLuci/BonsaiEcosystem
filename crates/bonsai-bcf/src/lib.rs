//! Bonsai Container Fabric (BCF) – Next-Generation Container Platform
//!
//! A sovereign, hardware-isolated, self-healing container platform that replaces
//! Docker & Kubernetes with deep integration into USOS, Sentinel Core, and Echo fabric.
//!
//! # Core Components
//!
//! - **Blueprint** – Declarative, immutable deployment specifications
//! - **Pulse Scheduler** – Real-time, energy-aware, distributed scheduling
//! - **Sanctum Vault Manager** – Hardware-isolated container runtime
//! - **TransferDaemon Service Mesh** – P2P networking, no sidecars
//! - **CAS Image Store** – Content-addressed, deduplicated image storage
//! - **Survival System** – Self-healing, auto-recovery, rollback
//! - **Universe Integration** – Complete observability and audit trail

pub mod blueprint;
pub mod scheduler;
pub mod vault;
pub mod mesh;
pub mod image;
pub mod healing;
pub mod api;
pub mod cli;
pub mod config;
pub mod errors;
pub mod events;

pub use blueprint::{Blueprint, ContainerSpec, ServiceSpec};
pub use scheduler::PulseScheduler;
pub use vault::{VaultManager, ContainerVault, VaultState};
pub use mesh::ServiceMesh;
pub use image::{CrystalImage, ImageManager};
pub use healing::SurvivalSystem;
pub use api::BcfClient;
pub use errors::{BcfError, Result};
pub use events::{Event, EventBus};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Main BCF orchestrator
pub struct BonsaiContainerFabric {
    pub blueprint_manager: Arc<blueprint::BlueprintManager>,
    pub scheduler: Arc<PulseScheduler>,
    pub vault_manager: Arc<VaultManager>,
    pub service_mesh: Arc<ServiceMesh>,
    pub image_manager: Arc<ImageManager>,
    pub survival_system: Arc<SurvivalSystem>,
    pub event_bus: Arc<EventBus>,
    config: config::BcfConfig,
}

impl BonsaiContainerFabric {
    /// Initialize BCF with default configuration
    pub async fn new() -> Result<Self> {
        let config = config::BcfConfig::default();
        Self::with_config(config).await
    }

    /// Initialize BCF with custom configuration
    pub async fn with_config(config: config::BcfConfig) -> Result<Self> {
        tracing::info!("Initializing Bonsai Container Fabric");

        let event_bus = Arc::new(EventBus::new());
        let blueprint_manager = Arc::new(blueprint::BlueprintManager::new());
        let scheduler = Arc::new(PulseScheduler::new());
        let vault_manager = Arc::new(VaultManager::new());
        let service_mesh = Arc::new(ServiceMesh::new());
        let image_manager = Arc::new(ImageManager::new());
        let survival_system = Arc::new(SurvivalSystem::new());

        let bcf = Self {
            blueprint_manager,
            scheduler,
            vault_manager,
            service_mesh,
            image_manager,
            survival_system,
            event_bus,
            config,
        };

        // Start monitoring loops
        bcf.start_monitoring().await?;

        tracing::info!("Bonsai Container Fabric initialized successfully");
        Ok(bcf)
    }

    /// Start all monitoring and healing loops
    async fn start_monitoring(&self) -> Result<()> {
        // Start survival system monitoring
        let survival = self.survival_system.clone();
        tokio::spawn(async move {
            survival.monitor_loop().await;
        });

        // Start scheduler rounds
        let scheduler = self.scheduler.clone();
        tokio::spawn(async move {
            scheduler.scheduling_loop().await;
        });

        Ok(())
    }

    /// Deploy a blueprint
    pub async fn deploy(&self, blueprint: Blueprint) -> Result<String> {
        tracing::info!("Deploying blueprint: {}", blueprint.name);

        // 1. Validate blueprint
        blueprint.validate()?;

        // 2. Build Crystal image
        let image = self.image_manager.build_from_blueprint(&blueprint).await?;

        // Emit event
        self.event_bus.emit(Event::DeploymentStarted {
            deployment_id: blueprint.name.clone(),
            timestamp: chrono::Utc::now(),
        }).await?;

        // 3. Schedule containers
        for container_spec in &blueprint.containers {
            let decisions = self.scheduler.schedule_container(container_spec).await?;

            for decision in decisions {
                // 4. Create vaults
                let vault = self.vault_manager.create_vault(
                    container_spec.clone(),
                    &decision,
                    &image,
                ).await?;

                // 5. Register in service mesh
                self.service_mesh.register_container(
                    &container_spec.name,
                    &vault,
                ).await?;

                // 6. Start container
                vault.start().await?;
            }
        }

        // Emit success event
        self.event_bus.emit(Event::DeploymentSucceeded {
            deployment_id: blueprint.name.clone(),
            timestamp: chrono::Utc::now(),
        }).await?;

        Ok(blueprint.name)
    }

    /// Scale a service
    pub async fn scale_service(&self, service_name: &str, replicas: u32) -> Result<()> {
        tracing::info!("Scaling service {} to {} replicas", service_name, replicas);

        self.scheduler.scale_service(service_name, replicas).await?;

        self.event_bus.emit(Event::ScaleUp {
            service_name: service_name.to_string(),
            new_replicas: replicas,
            timestamp: chrono::Utc::now(),
        }).await?;

        Ok(())
    }

    /// Get service status
    pub async fn get_service_status(&self, service_name: &str) -> Result<ServiceStatus> {
        self.vault_manager.get_service_status(service_name).await
    }

    /// Get container logs
    pub async fn get_logs(&self, service_name: &str, lines: usize) -> Result<Vec<String>> {
        self.vault_manager.get_container_logs(service_name, lines).await
    }
}

/// Service status information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceStatus {
    pub service_name: String,
    pub replicas: u32,
    pub ready_replicas: u32,
    pub updated_replicas: u32,
    pub containers: Vec<ContainerStatus>,
}

/// Container status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContainerStatus {
    pub container_id: String,
    pub state: String,  // "Running", "Crashed", "Paused", etc.
    pub restart_count: u32,
    pub ready: bool,
    pub cpu_usage_percent: f64,
    pub memory_usage_mib: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bcf_creation() {
        let bcf = BonsaiContainerFabric::new().await;
        assert!(bcf.is_ok());
    }
}
