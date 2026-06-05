//! Base trait for all backend code generators

use crate::error::Result;
use crate::dis::{ConvertedInstruction, Instruction, InstructionStream};
use crate::device_interface::DeviceInterface;
use super::CodeGenerationOutput;

/// Trait implemented by all backend code generators
pub trait Backend {
    /// Platform identifier (e.g., "macos_driverkit", "linux_kernel", "UOSC")
    fn platform(&self) -> &str;

    /// Generate code from a stream of instructions
    fn generate(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
    ) -> Result<CodeGenerationOutput>;

    /// Validate that all instructions can be generated for this platform
    fn validate(&self, instructions: &[Instruction]) -> Result<()>;

    /// Convert a single instruction to platform code
    fn convert_instruction(
        &self,
        instruction: &Instruction,
        device: &DeviceInterface,
    ) -> Result<ConvertedInstruction>;

    /// Get required includes/dependencies for this instruction
    fn get_includes_for_instruction(&self, instruction: &Instruction) -> Vec<String>;

    /// Get error handling code for this instruction
    fn get_error_handling(&self, instruction: &Instruction) -> Option<String>;

    /// Generate capability manifest (for platforms that need it)
    fn generate_capability_manifest(&self, device: &DeviceInterface) -> Result<String> {
        Ok(String::new())
    }

    /// Generate build configuration (Makefile, CMakeLists.txt, etc.)
    fn generate_build_config(&self, device: &DeviceInterface) -> Result<String> {
        Ok(String::new())
    }
}
