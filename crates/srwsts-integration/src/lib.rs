//! SRWSTS Integration
//!
//! Integrates SRWSTS with all Bonsai Ecosystem components:
//!
//! - **SanctumBridge**: Vault isolation and sandboxing
//! - **EnvironmentFabricBridge**: Test environment provisioning
//! - **SLMBridge**: Service lifecycle management
//! - **CASBridge**: Content-addressable storage for baselines
//! - **UMSBridge**: Universal Module System publishing
//! - **ValidationMeshBridge**: Baseline comparison and validation
//! - **AuditLogBridge**: Universe immutable audit logging
//! - **TransferDaemonBridge**: Distributed result transport
//! - **HDEBridge**: AI advisor for test prioritization

pub mod sanctum;
pub mod environment;
pub mod slm;
pub mod cas;
pub mod ums;
pub mod validation;
pub mod audit;
pub mod transfer;
pub mod hde;

pub use sanctum::SanctumBridge;
pub use environment::EnvironmentFabricBridge;
pub use slm::SLMBridge;
pub use cas::CASBridge;
pub use ums::UMSBridge;
pub use validation::ValidationMeshBridge;
pub use audit::AuditLogBridge;
pub use transfer::TransferDaemonBridge;
pub use hde::HDEBridge;

pub use srwsts_core::{SrwstsError, SrwstsResult};

use std::sync::Arc;

/// Central integration orchestrator
pub struct IntegrationOrchestrator {
    sanctum: Arc<SanctumBridge>,
    environment: Arc<EnvironmentFabricBridge>,
    slm: Arc<SLMBridge>,
    cas: Arc<CASBridge>,
    ums: Arc<UMSBridge>,
    validation: Arc<ValidationMeshBridge>,
    audit: Arc<AuditLogBridge>,
    transfer: Arc<TransferDaemonBridge>,
    hde: Arc<HDEBridge>,
}

impl IntegrationOrchestrator {
    /// Create a new integration orchestrator with all components
    pub async fn new() -> SrwstsResult<Self> {
        Ok(Self {
            sanctum: Arc::new(SanctumBridge::new().await?),
            environment: Arc::new(EnvironmentFabricBridge::new().await?),
            slm: Arc::new(SLMBridge::new().await?),
            cas: Arc::new(CASBridge::new().await?),
            ums: Arc::new(UMSBridge::new().await?),
            validation: Arc::new(ValidationMeshBridge::new().await?),
            audit: Arc::new(AuditLogBridge::new().await?),
            transfer: Arc::new(TransferDaemonBridge::new().await?),
            hde: Arc::new(HDEBridge::new().await?),
        })
    }

    /// Get Sanctum bridge
    pub fn sanctum(&self) -> &Arc<SanctumBridge> {
        &self.sanctum
    }

    /// Get environment fabric bridge
    pub fn environment(&self) -> &Arc<EnvironmentFabricBridge> {
        &self.environment
    }

    /// Get SLM bridge
    pub fn slm(&self) -> &Arc<SLMBridge> {
        &self.slm
    }

    /// Get CAS bridge
    pub fn cas(&self) -> &Arc<CASBridge> {
        &self.cas
    }

    /// Get UMS bridge
    pub fn ums(&self) -> &Arc<UMSBridge> {
        &self.ums
    }

    /// Get validation mesh bridge
    pub fn validation(&self) -> &Arc<ValidationMeshBridge> {
        &self.validation
    }

    /// Get audit log bridge
    pub fn audit(&self) -> &Arc<AuditLogBridge> {
        &self.audit
    }

    /// Get transfer daemon bridge
    pub fn transfer(&self) -> &Arc<TransferDaemonBridge> {
        &self.transfer
    }

    /// Get HDE bridge
    pub fn hde(&self) -> &Arc<HDEBridge> {
        &self.hde
    }

    /// Initialize all integrated components
    pub async fn initialize(&self) -> SrwstsResult<()> {
        tracing::info!("Initializing all SRWSTS integration bridges");

        self.sanctum.initialize().await?;
        self.environment.initialize().await?;
        self.slm.initialize().await?;
        self.cas.initialize().await?;
        self.ums.initialize().await?;
        self.validation.initialize().await?;
        self.audit.initialize().await?;
        self.transfer.initialize().await?;
        self.hde.initialize().await?;

        tracing::info!("All integration bridges initialized");
        Ok(())
    }

    /// Shutdown all integrated components
    pub async fn shutdown(&self) -> SrwstsResult<()> {
        tracing::info!("Shutting down all SRWSTS integration bridges");

        self.hde.shutdown().await?;
        self.transfer.shutdown().await?;
        self.audit.shutdown().await?;
        self.validation.shutdown().await?;
        self.ums.shutdown().await?;
        self.cas.shutdown().await?;
        self.slm.shutdown().await?;
        self.environment.shutdown().await?;
        self.sanctum.shutdown().await?;

        tracing::info!("All integration bridges shut down");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_orchestrator_creation() {
        let orchestrator = IntegrationOrchestrator::new().await;
        assert!(orchestrator.is_ok());
    }
}
