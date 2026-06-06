pub mod ast;
pub mod parser;
pub mod compiler;
pub mod validator;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinition {
    pub name: String,
    pub arch: String,
    pub isa: String,
    pub registers: Vec<RegisterDef>,
    pub memory: MemoryMap,
    pub pipeline: PipelineConfig,
    pub instruction_set: Vec<InstructionDef>,
    pub csr: Vec<CsrDef>,
    pub interrupts: InterruptConfig,
    pub timing: TimingTable,
    pub peripherals: Vec<PeripheralDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterDef {
    pub name: String,
    pub width_bits: u32,
    pub count: u32,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMap {
    pub ram: Vec<RamRegion>,
    pub mmio: Vec<MmioRegion>,
    pub rom: Vec<RomRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamRegion {
    pub name: String,
    pub size_bytes: u64,
    pub base_address: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MmioRegion {
    pub name: String,
    pub base_address: u64,
    pub size_bytes: u64,
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomRegion {
    pub name: String,
    pub base_address: u64,
    pub size_bytes: u64,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub stages: Vec<String>,
    pub superscalar: u32,
    pub branch_predictor: String,
    pub cache: CacheConfig,
    pub out_of_order: bool,
    pub speculation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub l1i: CacheLevel,
    pub l1d: CacheLevel,
    pub l2: Option<CacheLevel>,
    pub l3: Option<CacheLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLevel {
    pub size_kb: u32,
    pub associativity: u32,
    pub line_size_bytes: u32,
    pub latency_cycles: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionDef {
    pub mnemonic: String,
    pub encoding: String,
    pub funct3: Option<u8>,
    pub funct7: Option<u8>,
    pub opcode: Option<u8>,
    pub semantics: String,
    pub timing_cycles: u32,
    pub pipelined: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsrDef {
    pub name: String,
    pub address: u32,
    pub width_bits: u32,
    pub access: CsrAccess,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CsrAccess {
    ReadOnly,
    ReadWrite,
    WriteOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptConfig {
    pub external_pins: u32,
    pub timer_source: String,
    pub software_source: String,
    pub priority_levels: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingTable {
    pub entries: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeripheralDef {
    pub name: String,
    pub device_type: PeripheralType,
    pub base_address: u64,
    pub interrupt_line: Option<u32>,
    pub dma_channel: Option<u32>,
    pub config: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeripheralType {
    Uart,
    Spi,
    I2c,
    Gpio,
    Pcie,
    Usb,
    Ethernet,
    Audio,
    Gpu,
    Npu,
    Custom(String),
}

pub fn parse_hdl(source: &str) -> anyhow::Result<DeviceDefinition> {
    parser::parse(source)
}

pub fn compile_device(def: &DeviceDefinition, output_path: &str) -> anyhow::Result<()> {
    compiler::compile(def, output_path)
}

pub fn validate_device(def: &DeviceDefinition) -> anyhow::Result<()> {
    validator::validate(def)
}
