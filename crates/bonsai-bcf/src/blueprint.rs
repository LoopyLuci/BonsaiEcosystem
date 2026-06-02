use serde::{Deserialize, Serialize};
use crate::{Result, BcfError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blueprint {
    pub name: String,
    pub version: String,
    pub containers: Vec<ContainerSpec>,
    pub services: Vec<ServiceSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSpec {
    pub id: String,
    pub name: String,
    pub image: String,
    pub replicas: u32,
    pub resources: ResourceSpec,
    pub storage: StorageSpec,
    pub network: NetworkSpec,
    pub capability_tokens: Vec<String>,
    pub overlay_size_mib: Option<usize>,
    pub deadline: Option<std::time::Duration>,
    pub period: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub name: String,
    pub ports: Vec<PortMapping>,
    pub load_balancing: LoadBalancingPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub cpu_cores: f64,
    pub memory_mib: u64,
    pub gpu: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpec {
    pub volumes: Vec<VolumeSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeSpec {
    pub id: String,
    pub name: String,
    pub mount_path: String,
    pub volume_type: VolumeType,
    pub size_mib: usize,
    pub replicated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeType {
    Ephemeral,
    CasPersistent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSpec {
    pub ports: Vec<PortMapping>,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: u16,
    pub service_port: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingPolicy {
    RoundRobin,
    LeastLoaded,
    LatencyBased,
}

pub struct BlueprintManager;

impl BlueprintManager {
    pub fn new() -> Self {
        Self
    }
}

impl Blueprint {
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(BcfError::BlueprintValidation("Name is required".to_string()));
        }
        Ok(())
    }
}
