pub mod session;
pub mod config;

use hdl::DeviceDefinition;
use buse_core::{CpuState, MemoryBus, interpreter::Interpreter};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct EmulatorSession {
    pub device_name: String,
    pub arch: String,
    pub interpreter: Arc<RwLock<Interpreter>>,
    pub is_running: bool,
    pub cycle_count: u64,
}

impl EmulatorSession {
    pub fn new(def: &DeviceDefinition) -> Result<Self> {
        let num_regs = def.registers.iter().map(|r| r.count as usize).sum();
        let ram_size = def.memory.ram.iter().map(|r| r.size_bytes).sum();
        let state = CpuState::new(num_regs);
        let memory = MemoryBus::new(ram_size);
        let interpreter = Interpreter::new(state, memory);

        Ok(Self {
            device_name: def.name.clone(),
            arch: def.arch.clone(),
            interpreter: Arc::new(RwLock::new(interpreter)),
            is_running: false,
            cycle_count: 0,
        })
    }

    pub async fn run_cycles(&mut self, cycles: u64) -> Result<Vec<buse_core::ExecutionResult>> {
        self.is_running = true;
        let mut results = vec![];
        let mut interpreter = self.interpreter.write().await;

        for _ in 0..cycles {
            let result = interpreter.step();
            if result.exception.is_some() {
                results.push(result);
                break;
            }
            results.push(result);
        }

        self.cycle_count += cycles;
        self.is_running = false;
        Ok(results)
    }

    pub async fn snapshot(&self) -> Result<Vec<u8>> {
        let interpreter = self.interpreter.read().await;
        let state = interpreter.state();
        let snapshot = serde_json::to_vec(&state)?;
        Ok(snapshot)
    }

    pub async fn restore(&mut self, snapshot: &[u8]) -> Result<()> {
        let state: CpuState = serde_json::from_slice(snapshot)?;
        let mut interpreter = self.interpreter.write().await;
        *interpreter.state_mut() = state;
        Ok(())
    }
}
