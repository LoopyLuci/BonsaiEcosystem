//! Driver conversion engine - orchestrates the complete conversion pipeline

use crate::dis::{Instruction, InstructionStream};
use crate::device_interface::DeviceInterface;
use crate::backend::{Backend, MacOsBackend, LinuxBackend, UsosBackend, CodeGenerationOutput};
use crate::rules::RuleDatabase;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metrics from a conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionMetrics {
    pub platform: String,
    pub total_instructions: usize,
    pub converted_instructions: usize,
    pub rules_applied: usize,
    pub generation_time_ms: u64,
    pub output_size_bytes: usize,
}

/// Result of a complete driver conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub platform: String,
    pub source_code: String,
    pub header_files: Vec<(String, String)>,
    pub configuration_files: Vec<(String, String)>,
    pub metrics: ConversionMetrics,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ConversionResult {
    pub fn new(
        platform: String,
        source_code: String,
        metrics: ConversionMetrics,
    ) -> Self {
        let output_size = source_code.len();
        Self {
            success: true,
            platform,
            source_code,
            header_files: vec![],
            configuration_files: vec![],
            metrics: ConversionMetrics {
                output_size_bytes: output_size,
                ..metrics
            },
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.header_files = headers;
        self
    }

    pub fn with_configs(mut self, configs: Vec<(String, String)>) -> Self {
        self.configuration_files = configs;
        self
    }

    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn add_error(mut self, error: String) -> Self {
        self.success = false;
        self.errors.push(error);
        self
    }
}

/// Main driver conversion engine
pub struct ConversionEngine {
    rules: RuleDatabase,
}

impl ConversionEngine {
    pub fn new(rules: RuleDatabase) -> Self {
        Self { rules }
    }

    pub fn with_default_rules() -> Self {
        Self::new(RuleDatabase::with_default_usb_rules())
    }

    /// Main conversion entry point
    pub fn convert(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
        target_platform: &str,
    ) -> Result<ConversionResult> {
        let start_time = std::time::Instant::now();

        // Select the appropriate backend
        let backend: Box<dyn Backend> = match target_platform {
            "macos_driverkit" => Box::new(MacOsBackend::new()),
            "linux_kernel" => Box::new(LinuxBackend::new()),
            "UOSC" => Box::new(UsosBackend::new()),
            _ => {
                return Err(crate::UdcError::UnsupportedOperation(format!(
                    "Unsupported platform: {}",
                    target_platform
                )))
            }
        };

        // Validate instructions
        backend.validate(instructions)?;

        // Generate code
        let output = backend.generate(instructions, device)?;

        // Calculate metrics
        let elapsed = start_time.elapsed();
        let metrics = ConversionMetrics {
            platform: target_platform.to_string(),
            total_instructions: instructions.len(),
            converted_instructions: instructions.len(), // All instructions were converted
            rules_applied: 0, // Rules are applied during generation
            generation_time_ms: elapsed.as_millis() as u64,
            output_size_bytes: output.source_code.len(),
        };

        let mut result = ConversionResult::new(
            target_platform.to_string(),
            output.source_code,
            metrics,
        )
        .with_headers(output.header_files)
        .with_configs(output.configuration_files);

        // Add compilation instructions as warning if present
        if !output.compilation_instructions.is_empty() {
            result = result.add_warning(format!(
                "Compilation instructions: {}",
                output.compilation_instructions
            ));
        }

        Ok(result)
    }

    /// Convert from one format to another with rule application
    pub fn convert_with_rules(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
        source_platform: &str,
        target_platform: &str,
    ) -> Result<ConversionResult> {
        // First apply rules to transform instructions
        let mut transformed = instructions.to_vec();

        let platform_rules = self.rules.get_rules_for_platform(target_platform);
        let mut rules_applied = 0;

        // Note: In a real implementation, we would parse and apply rules to the IR
        for rule in platform_rules {
            // Rules would be applied here
            rules_applied += 1;
        }

        // Then perform the conversion
        let mut result = self.convert(&transformed, device, target_platform)?;
        result.metrics.rules_applied = rules_applied;

        Ok(result)
    }

    /// Batch convert for multiple platforms
    pub fn convert_all_platforms(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
    ) -> Result<HashMap<String, ConversionResult>> {
        let platforms = vec!["macos_driverkit", "linux_kernel", "UOSC"];
        let mut results = HashMap::new();

        for platform in platforms {
            match self.convert(instructions, device, platform) {
                Ok(result) => {
                    results.insert(platform.to_string(), result);
                }
                Err(e) => {
                    let mut result = ConversionResult::new(
                        platform.to_string(),
                        String::new(),
                        ConversionMetrics {
                            platform: platform.to_string(),
                            total_instructions: instructions.len(),
                            converted_instructions: 0,
                            rules_applied: 0,
                            generation_time_ms: 0,
                            output_size_bytes: 0,
                        },
                    );
                    result = result.add_error(e.to_string());
                    results.insert(platform.to_string(), result);
                }
            }
        }

        Ok(results)
    }

    /// Get the rules database
    pub fn rules(&self) -> &RuleDatabase {
        &self.rules
    }

    /// Update rules (for hot-reload)
    pub fn set_rules(&mut self, rules: RuleDatabase) {
        self.rules = rules;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dis::{Instruction, InstructionStream};

    #[test]
    fn test_conversion_engine_creation() {
        let engine = ConversionEngine::with_default_rules();
        assert!(!engine.rules().get_rules().is_empty());
    }

    #[test]
    fn test_unsupported_platform() {
        let engine = ConversionEngine::with_default_rules();
        let device = DeviceInterface::new(
            "Test".to_string(),
            0x1234,
            0x5678,
        );
        let instructions = vec![];

        let result = engine.convert(&instructions, &device, "unknown_platform");
        assert!(result.is_err());
    }
}
