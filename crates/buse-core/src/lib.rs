pub mod cpu;
pub mod memory;
pub mod pipeline;
pub mod device;
pub mod jit;
pub mod interpreter;
pub mod snapshot;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuState {
    pub registers: Vec<u64>,
    pub pc: u64,
    pub flags: u64,
    pub mode: CpuMode,
    pub cycle_count: u64,
}

impl CpuState {
    pub fn new(num_registers: usize) -> Self {
        Self {
            registers: vec![0; num_registers],
            pc: 0,
            flags: 0,
            mode: CpuMode::User,
            cycle_count: 0,
        }
    }

    pub fn with_registers(num: usize) -> Self {
        Self::new(num)
    }

    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.pc = 0;
        self.flags = 0;
        self.mode = CpuMode::User;
        self.cycle_count = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CpuMode {
    User,
    Supervisor,
    Machine,
    Hypervisor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub cycles: u64,
    pub exception: Option<Exception>,
    pub branch_taken: bool,
    pub branch_target: Option<u64>,
    pub memory_accesses: Vec<MemoryAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    pub cause: ExceptionCause,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExceptionCause {
    IllegalInstruction,
    PageFault,
    DivideByZero,
    Interrupt(u32),
    Breakpoint,
    Syscall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccess {
    pub address: u64,
    pub size_bytes: u32,
    pub is_write: bool,
    pub value: u64,
}

pub trait Device: Send + Sync {
    fn step(&mut self) -> ExecutionResult;
    fn reset(&mut self);
    fn read_register(&self, index: usize) -> u64;
    fn write_register(&mut self, index: usize, value: u64);
    fn get_pc(&self) -> u64;
    fn set_pc(&mut self, pc: u64);
    fn get_state(&self) -> &CpuState;
    fn get_state_mut(&mut self) -> &mut CpuState;
}
