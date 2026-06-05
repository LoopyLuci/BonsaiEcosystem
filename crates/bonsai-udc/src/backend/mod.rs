//! Backend code generators for different platforms

mod base;
// Backends will be added here
pub mod macos;
pub mod linux;
pub mod UOSC;

pub use base::Backend;
pub use macos::MacOsBackend;
pub use linux::LinuxBackend;
pub use UOSC::UsosBackend;

use crate::error::Result;
use crate::dis::ConvertedInstruction;
use crate::device_interface::DeviceInterface;
use crate::dis::Instruction;

/// Output from a backend code generation
#[derive(Debug, Clone)]
pub struct CodeGenerationOutput {
    pub platform: String,
    pub source_code: String,
    pub header_files: Vec<(String, String)>, // (filename, content)
    pub configuration_files: Vec<(String, String)>,
    pub build_artifacts: Vec<String>,
    pub compilation_instructions: String,
}

impl CodeGenerationOutput {
    pub fn new(platform: String, source_code: String) -> Self {
        Self {
            platform,
            source_code,
            header_files: Vec::new(),
            configuration_files: Vec::new(),
            build_artifacts: Vec::new(),
            compilation_instructions: String::new(),
        }
    }

    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.header_files = headers;
        self
    }

    pub fn with_config_files(mut self, configs: Vec<(String, String)>) -> Self {
        self.configuration_files = configs;
        self
    }

    pub fn with_build_instructions(mut self, instructions: String) -> Self {
        self.compilation_instructions = instructions;
        self
    }
}
