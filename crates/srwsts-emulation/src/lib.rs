//! SRWSTS Emulation Engine
//!
//! Provides comprehensive hardware and peripheral emulation for stress testing.
//!
//! ## Key Components
//!
//! - **EmulationTarget**: Enum for supported ISAs (x86-64, ARMv8, RISC-V 64)
//! - **HardwareModel**: CPU specifications, memory hierarchy, cache config, NUMA topology
//! - **CPUEmulator**: Trait for cycle-level execution, interrupt delivery, state capture
//! - **MemoryEmulator**: Cache hierarchy with latency modeling and page table emulation
//! - **StorageEmulator**: NVMe, SATA, RAM disk simulation with IOPS/latency models
//! - **NetworkEmulator**: Ethernet, WiFi, cellular with loss/latency/jitter/partitioning
//! - **PeripheralEmulator**: GPU, USB, sensor simulation
//! - **EmulationEnvironment**: Orchestrates all emulators with unified interface
//! - **Deterministic Clock**: Ensures reproducible execution traces
//!
//! ## Architecture
//!
//! The emulation engine is organized in a hierarchical structure:
//! - Physical layer: CPU, Memory, Storage, Network, Peripherals
//! - Management layer: Environment, Clock, State capture
//! - Integration layer: BUSH/QEMU backend stubs, Interrupt delivery

pub mod cache;
pub mod clock;
pub mod cpu;
pub mod errors;
pub mod interrupt;
pub mod memory;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod target;

pub use cache::{CacheConfig, CacheHierarchy, CacheLevel};
pub use clock::{Clock, DeterministicClock};
pub use cpu::{CPUEmulator, CPUState, CoreConfig};
pub use errors::{EmulationError, EmulationResult};
pub use interrupt::{InterruptController, InterruptType};
pub use memory::{MemoryEmulator, MemoryModel, PageTableConfig};
pub use network::{
    NetworkConfig, NetworkEmulator, NetworkInterface, NetworkLoss, NetworkProfile,
};
pub use peripheral::{GPUEmulator, PeripheralEmulator, SensorEmulator, USBEmulator};
pub use storage::{StorageConfig, StorageEmulator, StorageProfile, StorageType};
pub use target::EmulationTarget;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Hardware model specification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HardwareModel {
    /// Emulation target ISA
    pub target: EmulationTarget,
    /// CPU configuration per core
    pub cores: Vec<CoreConfig>,
    /// Memory hierarchy configuration
    pub memory_config: MemoryModel,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Storage configuration
    pub storage_config: StorageConfig,
    /// Network configuration
    pub network_config: NetworkConfig,
    /// NUMA topology: core to NUMA node mapping
    pub numa_topology: Vec<usize>,
    /// CPU frequency in MHz
    pub cpu_frequency_mhz: u32,
    /// TLB configuration: entries per level
    pub tlb_entries: (usize, usize, usize), // L1, L2, L3
}

impl Default for HardwareModel {
    fn default() -> Self {
        Self {
            target: EmulationTarget::X86_64,
            cores: vec![CoreConfig::default(); 8],
            memory_config: MemoryModel::default(),
            cache_config: CacheConfig::default(),
            storage_config: StorageConfig::default(),
            network_config: NetworkConfig::default(),
            numa_topology: (0..8).map(|i| i % 2).collect(),
            cpu_frequency_mhz: 3600,
            tlb_entries: (64, 512, 4096),
        }
    }
}

/// Main emulation environment orchestrator
pub struct EmulationEnvironment {
    /// Hardware model specification
    pub hardware: Arc<HardwareModel>,
    /// CPU emulator instances per core
    cpu_emulators: Arc<RwLock<Vec<Arc<dyn CPUEmulator>>>>,
    /// Memory emulator
    memory: Arc<RwLock<Box<dyn MemoryEmulator>>>,
    /// Storage emulator
    storage: Arc<RwLock<Box<dyn StorageEmulator>>>,
    /// Network emulator
    network: Arc<RwLock<Box<dyn NetworkEmulator>>>,
    /// Interrupt controller
    interrupt_controller: Arc<RwLock<InterruptController>>,
    /// Deterministic clock
    clock: Arc<RwLock<DeterministicClock>>,
    /// Run state
    running: Arc<RwLock<bool>>,
}

impl EmulationEnvironment {
    /// Create a new emulation environment with the given hardware model
    pub async fn new(hardware: HardwareModel) -> EmulationResult<Self> {
        let hardware = Arc::new(hardware);
        let clock = Arc::new(RwLock::new(DeterministicClock::new()));
        let interrupt_controller = Arc::new(RwLock::new(InterruptController::new(
            hardware.cores.len(),
        )));

        // Initialize CPU emulators for each core
        let mut cpu_emulators = Vec::new();
        for (i, core_cfg) in hardware.cores.iter().enumerate() {
            let cpu = Arc::new(DefaultCPUEmulator::new(i, core_cfg.clone()));
            cpu_emulators.push(cpu as Arc<dyn CPUEmulator>);
        }

        Ok(Self {
            hardware: hardware.clone(),
            cpu_emulators: Arc::new(RwLock::new(cpu_emulators)),
            memory: Arc::new(RwLock::new(Box::new(DefaultMemoryEmulator::new(
                hardware.memory_config.clone(),
                hardware.cache_config.clone(),
            )))),
            storage: Arc::new(RwLock::new(Box::new(DefaultStorageEmulator::new(
                &hardware.storage_config,
            )))),
            network: Arc::new(RwLock::new(Box::new(DefaultNetworkEmulator::new(
                &hardware.network_config,
            )))),
            interrupt_controller,
            clock,
            running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the emulation environment
    pub async fn start(&self) -> EmulationResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        tracing::info!("Emulation environment started");
        Ok(())
    }

    /// Stop the emulation environment
    pub async fn stop(&self) -> EmulationResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("Emulation environment stopped");
        Ok(())
    }

    /// Check if emulation is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Execute one cycle of emulation on all cores
    pub async fn execute_cycle(&self) -> EmulationResult<()> {
        if !*self.running.read().await {
            return Err(EmulationError::NotRunning);
        }

        // Increment deterministic clock
        let mut clock = self.clock.write().await;
        clock.advance();
        drop(clock);

        // Execute CPU cycle on each core
        let cpu_emulators = self.cpu_emulators.read().await;
        for cpu in cpu_emulators.iter() {
            cpu.cycle_execute().await?;
        }

        Ok(())
    }

    /// Execute multiple cycles
    pub async fn execute_cycles(&self, count: u64) -> EmulationResult<()> {
        for _ in 0..count {
            self.execute_cycle().await?;
        }
        Ok(())
    }

    /// Deliver an interrupt to a specific core
    pub async fn deliver_interrupt(
        &self,
        core_id: usize,
        interrupt_type: InterruptType,
    ) -> EmulationResult<()> {
        let cpu_emulators = self.cpu_emulators.read().await;
        if core_id >= cpu_emulators.len() {
            return Err(EmulationError::InvalidCoreId(core_id));
        }

        cpu_emulators[core_id].interrupt_deliver(interrupt_type).await?;
        Ok(())
    }

    /// Capture CPU state from a specific core
    pub async fn capture_cpu_state(&self, core_id: usize) -> EmulationResult<CPUState> {
        let cpu_emulators = self.cpu_emulators.read().await;
        if core_id >= cpu_emulators.len() {
            return Err(EmulationError::InvalidCoreId(core_id));
        }

        cpu_emulators[core_id].state_capture().await
    }

    /// Capture state from all CPU cores
    pub async fn capture_all_cpu_states(&self) -> EmulationResult<Vec<CPUState>> {
        let cpu_emulators = self.cpu_emulators.read().await;
        let mut states = Vec::new();

        for cpu in cpu_emulators.iter() {
            states.push(cpu.state_capture().await?);
        }

        Ok(states)
    }

    /// Get the current cycle count from the deterministic clock
    pub async fn cycle_count(&self) -> u64 {
        self.clock.read().await.cycle_count()
    }

    /// Get memory emulator for direct operations
    pub async fn memory(&self) -> Arc<RwLock<Box<dyn MemoryEmulator>>> {
        self.memory.clone()
    }

    /// Get storage emulator for direct operations
    pub async fn storage(&self) -> Arc<RwLock<Box<dyn StorageEmulator>>> {
        self.storage.clone()
    }

    /// Get network emulator for direct operations
    pub async fn network(&self) -> Arc<RwLock<Box<dyn NetworkEmulator>>> {
        self.network.clone()
    }

    /// Get interrupt controller for direct operations
    pub async fn interrupt_controller(&self) -> Arc<RwLock<InterruptController>> {
        self.interrupt_controller.clone()
    }

    /// Reset the emulation environment to initial state
    pub async fn reset(&self) -> EmulationResult<()> {
        let mut clock = self.clock.write().await;
        clock.reset();
        drop(clock);

        let cpu_emulators = self.cpu_emulators.read().await;
        for cpu in cpu_emulators.iter() {
            cpu.reset().await?;
        }

        self.memory.write().await.reset().await?;
        self.storage.write().await.reset().await?;
        self.network.write().await.reset().await?;
        self.interrupt_controller.write().await.reset();

        tracing::info!("Emulation environment reset to initial state");
        Ok(())
    }

    /// Get current state snapshot for introspection
    pub async fn snapshot(&self) -> EmulationResult<EnvironmentSnapshot> {
        let cpu_states = self.capture_all_cpu_states().await?;
        let cycle_count = self.cycle_count().await;
        let running = *self.running.read().await;

        Ok(EnvironmentSnapshot {
            cpu_states,
            cycle_count,
            running,
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Snapshot of the emulation environment state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentSnapshot {
    /// CPU states for all cores
    pub cpu_states: Vec<CPUState>,
    /// Total cycle count
    pub cycle_count: u64,
    /// Whether emulation is running
    pub running: bool,
    /// Timestamp of snapshot
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Default CPU emulator implementation
#[derive(Debug)]
struct DefaultCPUEmulator {
    core_id: usize,
    config: CoreConfig,
    state: Arc<RwLock<CPUState>>,
}

impl DefaultCPUEmulator {
    fn new(core_id: usize, config: CoreConfig) -> Self {
        Self {
            core_id,
            config,
            state: Arc::new(RwLock::new(CPUState::default())),
        }
    }
}

#[async_trait]
impl CPUEmulator for DefaultCPUEmulator {
    async fn cycle_execute(&self) -> EmulationResult<()> {
        let mut state = self.state.write().await;
        // Simulate instruction fetch/execute/commit
        state.instruction_count += 1;
        state.cycle_count += 1;
        Ok(())
    }

    async fn interrupt_deliver(&self, interrupt_type: InterruptType) -> EmulationResult<()> {
        let mut state = self.state.write().await;
        state.interrupt_pending = Some(interrupt_type);
        tracing::debug!(
            "Core {}: interrupt delivered {:?}",
            self.core_id,
            interrupt_type
        );
        Ok(())
    }

    async fn state_capture(&self) -> EmulationResult<CPUState> {
        Ok(self.state.read().await.clone())
    }

    async fn reset(&self) -> EmulationResult<()> {
        let mut state = self.state.write().await;
        *state = CPUState::default();
        Ok(())
    }
}

/// Default memory emulator implementation
#[derive(Debug)]
struct DefaultMemoryEmulator {
    _config: MemoryModel,
    _cache: CacheHierarchy,
    memory: Arc<RwLock<Vec<u8>>>,
}

impl DefaultMemoryEmulator {
    fn new(config: MemoryModel, cache_config: CacheConfig) -> Self {
        // Use a smaller size for emulation (1MB instead of 16GB default)
        let size = std::cmp::min(config.total_size_bytes, 1 * 1024 * 1024) as usize;
        Self {
            _config: config,
            _cache: CacheHierarchy::new(cache_config),
            memory: Arc::new(RwLock::new(vec![0u8; size])),
        }
    }
}

#[async_trait]
impl MemoryEmulator for DefaultMemoryEmulator {
    async fn read(&self, address: u64, size: usize) -> EmulationResult<Vec<u8>> {
        let memory = self.memory.read().await;
        if address as usize + size > memory.len() {
            return Err(EmulationError::AddressOutOfBounds(address));
        }
        Ok(memory[address as usize..address as usize + size].to_vec())
    }

    async fn write(&self, address: u64, data: &[u8]) -> EmulationResult<()> {
        let mut memory = self.memory.write().await;
        if address as usize + data.len() > memory.len() {
            return Err(EmulationError::AddressOutOfBounds(address));
        }
        memory[address as usize..address as usize + data.len()].copy_from_slice(data);
        Ok(())
    }

    async fn reset(&self) -> EmulationResult<()> {
        let mut memory = self.memory.write().await;
        memory.fill(0);
        Ok(())
    }
}

/// Default storage emulator implementation
#[derive(Debug)]
struct DefaultStorageEmulator {
    _config: StorageConfig,
    storage: Arc<RwLock<Vec<u8>>>,
}

impl DefaultStorageEmulator {
    fn new(config: &StorageConfig) -> Self {
        // Use a smaller size for emulation (1MB instead of actual capacity)
        let total_size = std::cmp::min(
            config.devices.iter().map(|d| d.capacity_bytes).sum::<u64>(),
            1 * 1024 * 1024,
        ) as usize;
        Self {
            _config: config.clone(),
            storage: Arc::new(RwLock::new(vec![0u8; total_size])),
        }
    }
}

#[async_trait]
impl StorageEmulator for DefaultStorageEmulator {
    async fn read(&self, address: u64, size: usize) -> EmulationResult<Vec<u8>> {
        let storage = self.storage.read().await;
        if address as usize + size > storage.len() {
            return Err(EmulationError::AddressOutOfBounds(address));
        }
        Ok(storage[address as usize..address as usize + size].to_vec())
    }

    async fn write(&self, address: u64, data: &[u8]) -> EmulationResult<()> {
        let mut storage = self.storage.write().await;
        if address as usize + data.len() > storage.len() {
            return Err(EmulationError::AddressOutOfBounds(address));
        }
        storage[address as usize..address as usize + data.len()].copy_from_slice(data);
        Ok(())
    }

    async fn reset(&self) -> EmulationResult<()> {
        let mut storage = self.storage.write().await;
        storage.fill(0);
        Ok(())
    }
}

/// Default network emulator implementation
#[derive(Debug)]
struct DefaultNetworkEmulator {
    config: NetworkConfig,
}

impl DefaultNetworkEmulator {
    fn new(config: &NetworkConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl NetworkEmulator for DefaultNetworkEmulator {
    async fn send_packet(&self, interface: usize, data: &[u8]) -> EmulationResult<()> {
        if interface >= self.config.interfaces.len() {
            return Err(EmulationError::InvalidInterfaceId(interface));
        }

        tracing::debug!(
            "Network: sent {} bytes on interface {}",
            data.len(),
            interface
        );
        Ok(())
    }

    async fn receive_packet(&self, interface: usize) -> EmulationResult<Option<Vec<u8>>> {
        if interface >= self.config.interfaces.len() {
            return Err(EmulationError::InvalidInterfaceId(interface));
        }
        Ok(None)
    }

    async fn reset(&self) -> EmulationResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_emulation_environment_creation() {
        let hardware = HardwareModel::default();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        assert!(!env.is_running().await);
    }

    #[tokio::test]
    async fn test_emulation_start_stop() {
        let hardware = HardwareModel::default();
        let env = EmulationEnvironment::new(hardware).await.unwrap();

        env.start().await.unwrap();
        assert!(env.is_running().await);

        env.stop().await.unwrap();
        assert!(!env.is_running().await);
    }

    #[tokio::test]
    async fn test_cycle_execution() {
        let hardware = HardwareModel::default();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        env.start().await.unwrap();

        let initial_cycle = env.cycle_count().await;
        env.execute_cycle().await.unwrap();
        let after_one = env.cycle_count().await;
        assert_eq!(after_one, initial_cycle + 1);

        env.execute_cycles(99).await.unwrap();
        let after_hundred = env.cycle_count().await;
        assert_eq!(after_hundred, initial_cycle + 100);
    }

    #[tokio::test]
    async fn test_cpu_state_capture() {
        let hardware = HardwareModel::default();
        let num_cores = hardware.cores.len();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        env.start().await.unwrap();

        env.execute_cycles(10).await.unwrap();

        let state = env.capture_cpu_state(0).await.unwrap();
        assert_eq!(state.instruction_count, 10);

        let all_states = env.capture_all_cpu_states().await.unwrap();
        assert_eq!(all_states.len(), num_cores);
    }

    #[tokio::test]
    async fn test_interrupt_delivery() {
        let hardware = HardwareModel::default();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        env.start().await.unwrap();

        env.deliver_interrupt(0, InterruptType::Timer)
            .await
            .unwrap();

        let state = env.capture_cpu_state(0).await.unwrap();
        assert_eq!(state.interrupt_pending, Some(InterruptType::Timer));
    }

    #[tokio::test]
    async fn test_invalid_core_id() {
        let hardware = HardwareModel::default();
        let num_cores = hardware.cores.len();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        env.start().await.unwrap();

        let result = env.capture_cpu_state(num_cores + 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_environment_reset() {
        let hardware = HardwareModel::default();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        env.start().await.unwrap();

        env.execute_cycles(100).await.unwrap();
        let cycle_before_reset = env.cycle_count().await;
        assert!(cycle_before_reset > 0);

        env.reset().await.unwrap();
        let cycle_after_reset = env.cycle_count().await;
        assert_eq!(cycle_after_reset, 0);
    }

    #[tokio::test]
    async fn test_snapshot() {
        let hardware = HardwareModel::default();
        let env = EmulationEnvironment::new(hardware).await.unwrap();
        env.start().await.unwrap();

        env.execute_cycles(50).await.unwrap();
        let snapshot = env.snapshot().await.unwrap();

        assert!(snapshot.running);
        assert_eq!(snapshot.cycle_count, 50);
        assert!(snapshot.cpu_states.len() > 0);
    }
}
