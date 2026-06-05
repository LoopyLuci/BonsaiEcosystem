//! High-level integration hub for driver conversion

use crate::dis::Instruction;
use crate::device_interface::DeviceInterface;
use crate::engine::{ConversionEngine, ConversionResult};
use crate::rules::RuleDatabase;
use crate::registry::DriverRegistry;
use crate::error::Result;
use std::path::Path;
use std::fs;

/// Complete driver conversion context
pub struct DriverConversionContext {
    pub device: DeviceInterface,
    pub instructions: Vec<Instruction>,
    pub target_platform: String,
    pub source_platform: Option<String>,
}

impl DriverConversionContext {
    pub fn new(
        device: DeviceInterface,
        instructions: Vec<Instruction>,
        target_platform: String,
    ) -> Self {
        Self {
            device,
            instructions,
            target_platform,
            source_platform: None,
        }
    }

    pub fn with_source_platform(mut self, platform: String) -> Self {
        self.source_platform = Some(platform);
        self
    }
}

/// High-level driver conversion orchestrator
pub struct DriverConverter {
    engine: ConversionEngine,
    registry: Option<DriverRegistry>,
}

impl DriverConverter {
    pub fn new(engine: ConversionEngine) -> Self {
        Self {
            engine,
            registry: None,
        }
    }

    pub fn with_default_engine() -> Self {
        Self::new(ConversionEngine::with_default_rules())
    }

    pub fn with_registry(mut self, registry: DriverRegistry) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Main conversion entry point
    /// Orchestrates: parse DIS → generate IR → apply rules → call backend → write output
    pub fn convert_driver(
        &self,
        context: &DriverConversionContext,
    ) -> Result<DriverConversionOutput> {
        // Step 1: Validate input
        if context.instructions.is_empty() {
            return Err(crate::UdcError::ValidationError(
                "No instructions provided".to_string(),
            ));
        }

        // Step 2: Perform conversion
        let conversion_result = if let Some(source_platform) = &context.source_platform {
            self.engine.convert_with_rules(
                &context.instructions,
                &context.device,
                source_platform,
                &context.target_platform,
            )?
        } else {
            self.engine.convert(
                &context.instructions,
                &context.device,
                &context.target_platform,
            )?
        };

        // Step 3: Generate output
        let output = DriverConversionOutput {
            conversion_result,
            device_name: context.device.device_name.clone(),
            vendor_id: context.device.vendor_id,
            device_id: context.device.product_id,
        };

        Ok(output)
    }

    /// Convert for multiple platforms simultaneously
    pub fn convert_all_platforms(
        &self,
        context: &DriverConversionContext,
    ) -> Result<Vec<DriverConversionOutput>> {
        let platforms = vec!["linux_kernel", "macos_driverkit", "UOSC"];
        let mut outputs = Vec::new();

        for platform in platforms {
            let mut platform_context = context.clone();
            platform_context.target_platform = platform.to_string();

            match self.convert_driver(&platform_context) {
                Ok(output) => outputs.push(output),
                Err(e) => {
                    eprintln!("Failed to convert for {}: {}", platform, e);
                }
            }
        }

        Ok(outputs)
    }

    /// Convert and save to disk
    pub fn convert_and_save(
        &self,
        context: &DriverConversionContext,
        output_dir: &Path,
    ) -> Result<DriverConversionOutput> {
        // Create output directory
        fs::create_dir_all(output_dir).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;

        // Perform conversion
        let output = self.convert_driver(context)?;

        // Write files
        output.save_to_disk(output_dir)?;

        Ok(output)
    }

    /// Convert, save, and register driver
    pub fn convert_save_and_register(
        &self,
        context: &DriverConversionContext,
        output_dir: &Path,
    ) -> Result<DriverConversionOutput> {
        let output = self.convert_and_save(context, output_dir)?;

        // Register if we have a registry
        if let Some(mut registry) = self.registry.clone() {
            let driver = crate::registry::InstalledDriver::new(
                context.device.device_name.clone(),
                context.device.vendor_id,
                context.device.product_id,
                context.target_platform.clone(),
                output_dir.to_string_lossy().to_string(),
            );

            registry.register(driver)?;
            registry.save()?;
        }

        Ok(output)
    }

    /// Get the underlying conversion engine
    pub fn engine(&self) -> &ConversionEngine {
        &self.engine
    }

    /// Update the rule database (hot-reload)
    pub fn update_rules(&mut self, rules: RuleDatabase) {
        // Note: We can't mutate engine directly due to ownership, so this is advisory
        // In a real implementation, engine would be behind a RwLock
    }
}

/// Output from driver conversion
#[derive(Debug, Clone)]
pub struct DriverConversionOutput {
    pub conversion_result: ConversionResult,
    pub device_name: String,
    pub vendor_id: u16,
    pub device_id: u16,
}

impl DriverConversionOutput {
    /// Save the converted driver to disk
    pub fn save_to_disk(&self, output_dir: &Path) -> Result<()> {
        // Create directory
        fs::create_dir_all(output_dir).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;

        // Write main source code
        let main_file = output_dir.join(format!(
            "{}.cpp",
            self.device_name.replace(" ", "_")
        ));
        fs::write(&main_file, &self.conversion_result.source_code).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;

        // Write header files
        for (filename, content) in &self.conversion_result.header_files {
            let header_file = output_dir.join(filename);
            fs::write(&header_file, content).map_err(|e| {
                crate::UdcError::IoError(e)
            })?;
        }

        // Write configuration files
        for (filename, content) in &self.conversion_result.configuration_files {
            let config_file = output_dir.join(filename);
            fs::write(&config_file, content).map_err(|e| {
                crate::UdcError::IoError(e)
            })?;
        }

        // Write metadata
        let metadata = serde_json::json!({
            "device_name": self.device_name,
            "vendor_id": format!("{:#06x}", self.vendor_id),
            "device_id": format!("{:#06x}", self.device_id),
            "platform": self.conversion_result.platform,
            "metrics": self.conversion_result.metrics,
            "success": self.conversion_result.success,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let metadata_file = output_dir.join("metadata.json");
        fs::write(
            &metadata_file,
            serde_json::to_string_pretty(&metadata).unwrap(),
        )
        .map_err(|e| crate::UdcError::IoError(e))?;

        Ok(())
    }

    /// Get a summary of the conversion
    pub fn summary(&self) -> String {
        format!(
            "Device: {} ({:#06x}:{:#06x})\n\
             Platform: {}\n\
             Instructions: {}\n\
             Generation time: {}ms\n\
             Output size: {} bytes\n\
             Status: {}",
            self.device_name,
            self.vendor_id,
            self.device_id,
            self.conversion_result.platform,
            self.conversion_result.metrics.total_instructions,
            self.conversion_result.metrics.generation_time_ms,
            self.conversion_result.metrics.output_size_bytes,
            if self.conversion_result.success {
                "SUCCESS"
            } else {
                "FAILED"
            }
        )
    }
}

impl DriverConversionContext {
    /// Create from JSON
    pub fn from_json(json: &str, target_platform: &str) -> Result<Self> {
        let spec: serde_json::Value = serde_json::from_str(json)?;

        // Parse device
        let device_data = &spec["device"];
        let device_json = serde_json::to_string(device_data)
            .map_err(|e| crate::UdcError::SerializationError(e))?;
        let device: DeviceInterface =
            serde_json::from_str(&device_json).map_err(|e| crate::UdcError::SerializationError(e))?;

        // Parse instructions
        let instructions_data = &spec["instructions"];
        let instructions_json = serde_json::to_string(instructions_data)
            .map_err(|e| crate::UdcError::SerializationError(e))?;
        let instructions: Vec<Instruction> = serde_json::from_str(&instructions_json)
            .map_err(|e| crate::UdcError::SerializationError(e))?;

        Ok(Self::new(device, instructions, target_platform.to_string()))
    }

    /// Load from JSON file
    pub fn from_file(path: &Path, target_platform: &str) -> Result<Self> {
        let json = fs::read_to_string(path).map_err(|e| crate::UdcError::IoError(e))?;
        Self::from_json(&json, target_platform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_converter_creation() {
        let converter = DriverConverter::with_default_engine();
        assert!(converter.engine().rules().get_rules().len() > 0);
    }
}
