//! Universal Driver Compiler - CLI interface

use crate::engine::{ConversionEngine, ConversionResult};
use crate::registry::DriverRegistry;
use crate::rules::RuleDatabase;
use crate::error::Result;
use crate::dis::Instruction;
use crate::device_interface::DeviceInterface;
use std::fs;
use std::path::{Path, PathBuf};

/// CLI command parser
pub struct CliArgs {
    pub command: String,
    pub args: std::collections::HashMap<String, String>,
}

impl CliArgs {
    pub fn parse(args: &[String]) -> Result<Self> {
        if args.len() < 2 {
            return Err(crate::UdcError::InvalidInstruction(
                "No command provided".to_string(),
            ));
        }

        let command = args[1].clone();
        let mut parsed_args = std::collections::HashMap::new();

        let mut i = 2;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                let key = arg[2..].to_string();
                if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    parsed_args.insert(key, args[i + 1].clone());
                    i += 2;
                } else {
                    parsed_args.insert(key, "true".to_string());
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        Ok(Self {
            command,
            args: parsed_args,
        })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.args.get(key).map(|s| s.as_str())
    }

    pub fn get_or_error(&self, key: &str) -> Result<String> {
        self.get(key)
            .map(|s| s.to_string())
            .ok_or_else(|| {
                crate::UdcError::InvalidInstruction(format!("Missing required argument: --{}", key))
            })
    }
}

/// CLI Command executor
pub struct Cli {
    engine: ConversionEngine,
    registry: Option<DriverRegistry>,
}

impl Cli {
    pub fn new(engine: ConversionEngine) -> Self {
        Self {
            engine,
            registry: None,
        }
    }

    pub fn with_registry(mut self, registry: DriverRegistry) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Execute a CLI command
    pub fn execute(&mut self, args: &CliArgs) -> Result<String> {
        match args.command.as_str() {
            "convert" => self.cmd_convert(args),
            "install" => self.cmd_install(args),
            "rollback" => self.cmd_rollback(args),
            "list" => self.cmd_list(args),
            "help" => self.cmd_help(),
            _ => Err(crate::UdcError::InvalidInstruction(
                format!("Unknown command: {}", args.command),
            )),
        }
    }

    /// Convert driver command: convert --input file.json --target platform --output dir
    fn cmd_convert(&mut self, args: &CliArgs) -> Result<String> {
        let input_file = args.get_or_error("input")?;
        let target = args.get_or_error("target")?;
        let output_dir = args.get_or_error("output")?;

        // Read input JSON file
        let json_content = fs::read_to_string(&input_file).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;

        // Parse the device interface and instructions
        let (device, instructions) = self.parse_input(&json_content)?;

        // Create output directory
        let output_path = Path::new(&output_dir);
        fs::create_dir_all(output_path).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;

        // Perform conversion
        let result = self.engine.convert(&instructions, &device, &target)?;

        // Write output files
        let main_file = output_path.join(format!("{}.cpp", device.device_name.replace(" ", "_")));
        fs::write(&main_file, &result.source_code).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;

        // Write header files
        for (filename, content) in &result.header_files {
            let header_file = output_path.join(filename);
            fs::write(&header_file, content).map_err(|e| {
                crate::UdcError::IoError(e)
            })?;
        }

        // Write configuration files
        for (filename, content) in &result.configuration_files {
            let config_file = output_path.join(filename);
            fs::write(&config_file, content).map_err(|e| {
                crate::UdcError::IoError(e)
            })?;
        }

        // Write metadata
        let metadata = serde_json::json!({
            "platform": result.platform,
            "metrics": result.metrics,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        let metadata_file = output_path.join("metadata.json");
        fs::write(&metadata_file, serde_json::to_string_pretty(&metadata).unwrap())
            .map_err(|e| crate::UdcError::IoError(e))?;

        let mut output = format!(
            "Driver converted successfully to {}\n\
             Output directory: {}\n\
             Main source: {}\n\
             Conversion time: {}ms\n\
             Output size: {} bytes\n\
             Rules applied: {}",
            target,
            output_dir,
            main_file.display(),
            result.metrics.generation_time_ms,
            result.metrics.output_size_bytes,
            result.metrics.rules_applied,
        );

        if !result.warnings.is_empty() {
            output.push_str("\n\nWarnings:\n");
            for warning in &result.warnings {
                output.push_str(&format!("  - {}\n", warning));
            }
        }

        Ok(output)
    }

    /// Install driver command: install --vendor id --device id --target platform --source dir
    fn cmd_install(&mut self, args: &CliArgs) -> Result<String> {
        let vendor_str = args.get_or_error("vendor")?;
        let device_str = args.get_or_error("device")?;
        let target = args.get_or_error("target")?;
        let source_dir = args.get_or_error("source")?;

        // Parse vendor and device IDs
        let vendor_id = parse_hex_id(&vendor_str)?;
        let device_id = parse_hex_id(&device_str)?;

        // Get or initialize registry
        if self.registry.is_none() {
            self.registry = Some(DriverRegistry::with_default_path()?);
        }

        let registry = self.registry.as_mut().unwrap();

        // Create driver entry
        let name = format!(
            "Driver_{}_{}_{}",
            hex::encode([((vendor_id >> 8) & 0xFF) as u8, (vendor_id & 0xFF) as u8]),
            hex::encode([((device_id >> 8) & 0xFF) as u8, (device_id & 0xFF) as u8]),
            target
        );

        let driver = crate::registry::InstalledDriver::new(
            name,
            vendor_id,
            device_id,
            target.to_string(),
            source_dir.to_string(),
        );

        registry.register(driver)?;
        registry.save()?;

        Ok(format!(
            "Driver installed successfully\n\
             Vendor ID: {:#06x}\n\
             Device ID: {:#06x}\n\
             Target: {}\n\
             Registry saved",
            vendor_id, device_id, target
        ))
    }

    /// Rollback driver command: rollback --vendor id --device id --target platform --version version
    fn cmd_rollback(&mut self, args: &CliArgs) -> Result<String> {
        let vendor_str = args.get_or_error("vendor")?;
        let device_str = args.get_or_error("device")?;
        let target = args.get_or_error("target")?;
        let version = args.get_or_error("version")?;

        let vendor_id = parse_hex_id(&vendor_str)?;
        let device_id = parse_hex_id(&device_str)?;

        // Get or initialize registry
        if self.registry.is_none() {
            self.registry = Some(DriverRegistry::with_default_path()?);
        }

        let registry = self.registry.as_mut().unwrap();
        registry.rollback(vendor_id, device_id, &target, &version)?;
        registry.save()?;

        Ok(format!(
            "Driver rolled back to version {}\n\
             Vendor ID: {:#06x}\n\
             Device ID: {:#06x}\n\
             Target: {}",
            version, vendor_id, device_id, target
        ))
    }

    /// List drivers command: list [--os platform]
    fn cmd_list(&mut self, args: &CliArgs) -> Result<String> {
        // Get or initialize registry
        if self.registry.is_none() {
            self.registry = Some(DriverRegistry::with_default_path()?);
        }

        let registry = self.registry.as_ref().unwrap();

        let drivers = if let Some(os) = args.get("os") {
            registry.list_by_os(os)
        } else {
            registry.list_all()
        };

        if drivers.is_empty() {
            return Ok("No drivers found".to_string());
        }

        let mut output = "Installed drivers:\n".to_string();
        for driver in drivers {
            output.push_str(&format!(
                "\n  Name: {}\n  \
                Vendor ID: {:#06x}\n  \
                Device ID: {:#06x}\n  \
                Target: {}\n  \
                Version: {}\n  \
                Installed: {}\n",
                driver.name,
                driver.vendor_id,
                driver.device_id,
                driver.target_os,
                driver.current_version,
                driver.installed_at
            ));
        }

        Ok(output)
    }

    /// Help command
    fn cmd_help(&self) -> Result<String> {
        let help = r#"Universal Driver Compiler (UDC) - CLI

Usage: udc <command> [options]

Commands:
  convert   Convert a driver to a target platform
  install   Install a converted driver
  rollback  Rollback a driver to a previous version
  list      List installed drivers
  help      Show this help message

Examples:
  udc convert --input brother_fax.json --target linux-kernel --output ./output
  udc convert --input device.json --target macos-driverkit --output ./macos_driver
  udc install --vendor 0x04f9 --device 0x1917 --target linux-kernel --source ./output
  udc list --os linux-kernel
  udc rollback --vendor 0x04f9 --device 0x1917 --target linux-kernel --version 1.0.0

Supported Targets:
  - linux_kernel     : Linux kernel modules
  - macos_driverkit  : macOS DriverKit drivers
  - UOSC             : UOSC native drivers
"#;
        Ok(help.to_string())
    }

    /// Parse input JSON file containing device interface and instructions
    fn parse_input(&self, json: &str) -> Result<(DeviceInterface, Vec<Instruction>)> {
        // Try parsing as a full driver spec
        let spec: serde_json::Value = serde_json::from_str(json)?;

        // Extract device interface
        let device_data = &spec["device"];
        let device_json = serde_json::to_string(device_data).unwrap();
        let device: DeviceInterface = serde_json::from_str(&device_json)?;

        // Extract instructions
        let instructions_data = &spec["instructions"];
        let instructions_json = serde_json::to_string(instructions_data).unwrap();
        let instructions: Vec<Instruction> = serde_json::from_str(&instructions_json)?;

        Ok((device, instructions))
    }
}

/// Parse a hex ID string (e.g., "0x04f9")
fn parse_hex_id(s: &str) -> Result<u16> {
    let trimmed = s.trim();
    let num_str = if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        &trimmed[2..]
    } else {
        trimmed
    };

    u16::from_str_radix(num_str, 16).map_err(|_| {
        crate::UdcError::ValidationError(format!("Invalid hex ID: {}", s))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_id() {
        assert_eq!(parse_hex_id("0x04f9").unwrap(), 0x04f9);
        assert_eq!(parse_hex_id("1234").unwrap(), 0x1234);
    }

    #[test]
    fn test_cli_args_parsing() {
        let args = vec![
            "udc".to_string(),
            "convert".to_string(),
            "--input".to_string(),
            "file.json".to_string(),
            "--target".to_string(),
            "linux_kernel".to_string(),
        ];

        let parsed = CliArgs::parse(&args).unwrap();
        assert_eq!(parsed.command, "convert");
        assert_eq!(parsed.get("input"), Some("file.json"));
        assert_eq!(parsed.get("target"), Some("linux_kernel"));
    }
}
