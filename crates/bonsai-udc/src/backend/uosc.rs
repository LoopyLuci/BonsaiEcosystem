//! USOS native driver backend code generator

use crate::dis::Instruction;
use crate::device_interface::DeviceInterface;
use crate::error::Result;
use super::{Backend, CodeGenerationOutput};
use crate::dis::ConvertedInstruction;

pub struct UsosBackend;

impl UsosBackend {
    pub fn new() -> Self {
        UsosBackend
    }

    fn instruction_to_code(&self, instruction: &Instruction) -> String {
        match instruction {
            Instruction::MMIORead32 { addr } => {
                format!("let value: u32 = unsafe {{ ptr::read_volatile(0x{:x} as *const u32) }};", addr)
            }
            Instruction::MMIOWrite32 { addr, value } => {
                format!(
                    "unsafe {{ ptr::write_volatile(0x{:x} as *mut u32, 0x{:x}); }}",
                    addr, value
                )
            }
            Instruction::MMIORead64 { addr } => {
                format!("let value: u64 = unsafe {{ ptr::read_volatile(0x{:x} as *const u64) }};", addr)
            }
            Instruction::MMIOWrite64 { addr, value } => {
                format!(
                    "unsafe {{ ptr::write_volatile(0x{:x} as *mut u64, 0x{:x}); }}",
                    addr, value
                )
            }
            Instruction::USBBulkWrite {
                endpoint,
                buffer_size,
                timeout_ms,
            } => {
                format!(
                    "let result = self.device.bulk_write(0x{:02x}, buffer.as_ref(), {}, {}).await;",
                    endpoint, buffer_size, timeout_ms
                )
            }
            Instruction::USBBulkRead {
                endpoint,
                buffer_size,
                timeout_ms,
            } => {
                format!(
                    "let result = self.device.bulk_read(0x{:02x}, &mut buffer, {}).await;",
                    endpoint, buffer_size
                )
            }
            Instruction::USBControlRead {
                request_type,
                request,
                value,
                index,
                length,
                timeout_ms,
            } => {
                format!(
                    "let result = self.device.control_read(\n        \
                    {}, {}, 0x{:04x}, 0x{:04x},\n        \
                    &mut buffer, {}\n    ).await;",
                    request_type, request, value, index
                )
            }
            Instruction::Delay { milliseconds } => {
                format!(
                    "async_sleep(std::time::Duration::from_millis({})).await;",
                    milliseconds
                )
            }
            Instruction::SetupInterrupt {
                irq_number,
                handler_name,
            } => {
                format!(
                    "self.register_interrupt_handler({}, |_ctx| {{ {} }});",
                    irq_number, handler_name
                )
            }
            Instruction::EnableInterrupt { irq_number } => {
                format!("self.enable_interrupt({});", irq_number)
            }
            Instruction::DisableInterrupt { irq_number } => {
                format!("self.disable_interrupt({});", irq_number)
            }
            Instruction::HandleError {
                error_code,
                handler,
            } => {
                format!(
                    "if let Err(e) = result {{\n        \
                    eprintln!(\"Driver error 0x{:x}: {{}}\", e);\n        \
                    {}().await;\n    }}",
                    error_code, handler
                )
            }
            _ => "// Unsupported instruction".to_string(),
        }
    }
}

impl Default for UsosBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for UsosBackend {
    fn platform(&self) -> &str {
        "usos"
    }

    fn generate(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
    ) -> Result<CodeGenerationOutput> {
        self.validate(instructions)?;

        let mut code = String::new();

        // Imports
        code.push_str("use std::ptr;\n");
        code.push_str("use std::sync::Arc;\n");
        code.push_str("use async_trait::async_trait;\n");
        code.push_str("use usos_driver_framework::*;\n\n");

        // Driver structure
        code.push_str(&format!(
            "pub struct {} {{\n",
            device.device_name.replace(" ", "") + "Driver"
        ));
        code.push_str("    device: Arc<USBDevice>,\n");
        code.push_str("    buffer: [u8; 4096],\n");
        code.push_str("    _phantom: std::marker::PhantomData<()>,\n");
        code.push_str("}\n\n");

        // Trait implementation
        code.push_str("#[async_trait]\n");
        code.push_str("impl Driver for {} {{\n", device.device_name.replace(" ", "") + "Driver"));
        code.push_str("    fn name(&self) -> &'static str {\n");
        code.push_str(&format!("        \"{}\"\n", device.device_name));
        code.push_str("    }\n\n");

        code.push_str("    fn vendor_id(&self) -> u16 {\n");
        code.push_str(&format!("        0x{:04x}\n", device.vendor_id));
        code.push_str("    }\n\n");

        code.push_str("    fn product_id(&self) -> u16 {\n");
        code.push_str(&format!("        0x{:04x}\n", device.product_id));
        code.push_str("    }\n\n");

        code.push_str("    async fn initialize(&mut self) -> Result<(), DriverError> {\n");
        code.push_str("        // Initialize device\n");

        for instr in instructions {
            let line = self.instruction_to_code(instr);
            code.push_str("        ");
            code.push_str(&line);
            code.push('\n');
        }

        code.push_str("        Ok(())\n");
        code.push_str("    }\n\n");

        code.push_str("    async fn shutdown(&mut self) -> Result<(), DriverError> {\n");
        code.push_str("        // Cleanup\n");
        code.push_str("        Ok(())\n");
        code.push_str("    }\n\n");

        code.push_str("    fn capabilities(&self) -> DriverCapabilities {\n");
        code.push_str("        DriverCapabilities {\n");
        code.push_str("            supports_hotplug: true,\n");
        code.push_str("            supports_power_management: true,\n");
        code.push_str("            supports_dma: true,\n");
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        // Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "usos-{}-driver"
version = "0.1.0"
edition = "2021"

[dependencies]
usos_driver_framework = "0.1"
async-trait = "0.1"
tokio = {{ version = "1", features = ["full"] }}

[lib]
crate-type = ["cdylib"]
"#,
            device.device_name.to_lowercase().replace(" ", "-")
        );

        let output = CodeGenerationOutput::new(
            "usos".to_string(),
            code,
        )
        .with_config_files(vec![("Cargo.toml".to_string(), cargo_toml)])
        .with_build_instructions("cargo build --release --lib".to_string());

        Ok(output)
    }

    fn validate(&self, instructions: &[Instruction]) -> Result<()> {
        for instr in instructions {
            match instr {
                Instruction::Comment { .. } => {
                    // Always valid
                }
                _ => {
                    // All other instructions are valid for USOS
                }
            }
        }
        Ok(())
    }

    fn convert_instruction(
        &self,
        instruction: &Instruction,
        _device: &DeviceInterface,
    ) -> Result<ConvertedInstruction> {
        let code = self.instruction_to_code(instruction);
        Ok(ConvertedInstruction {
            instruction: instruction.clone(),
            platform: "usos".to_string(),
            generated_code: code,
            required_includes: self.get_includes_for_instruction(instruction),
            error_handling: self.get_error_handling(instruction),
            comment: None,
        })
    }

    fn get_includes_for_instruction(&self, instruction: &Instruction) -> Vec<String> {
        match instruction {
            Instruction::USBBulkWrite { .. }
            | Instruction::USBBulkRead { .. }
            | Instruction::USBControlRead { .. }
            | Instruction::USBControlWrite { .. } => {
                vec!["use usos_driver_framework::usb::*;".to_string()]
            }
            Instruction::SetupInterrupt { .. }
            | Instruction::EnableInterrupt { .. }
            | Instruction::DisableInterrupt { .. } => {
                vec!["use usos_driver_framework::interrupt::*;".to_string()]
            }
            _ => vec![],
        }
    }

    fn get_error_handling(&self, instruction: &Instruction) -> Option<String> {
        match instruction {
            Instruction::USBBulkWrite { .. } | Instruction::USBBulkRead { .. } => {
                Some("result?;".to_string())
            }
            _ => None,
        }
    }

    fn generate_capability_manifest(&self, device: &DeviceInterface) -> Result<String> {
        let mut manifest = String::new();
        manifest.push_str("{\n");
        manifest.push_str(&format!(
            "  \"driver_name\": \"{}\",\n",
            device.device_name
        ));
        manifest.push_str(&format!(
            "  \"vendor_id\": {:#06x},\n",
            device.vendor_id
        ));
        manifest.push_str(&format!(
            "  \"product_id\": {:#06x},\n",
            device.product_id
        ));
        manifest.push_str("  \"platform\": \"usos\",\n");
        manifest.push_str("  \"capabilities\": [\n");
        manifest.push_str("    \"usb_device_access\",\n");
        manifest.push_str("    \"interrupt_handling\",\n");
        manifest.push_str("    \"dma_access\"\n");
        manifest.push_str("  ],\n");
        manifest.push_str("  \"endpoints\": [\n");
        for endpoint in &device.endpoints {
            manifest.push_str(&format!(
                "    {{ \"number\": {}, \"transfer_type\": \"{}\" }},\n",
                endpoint.endpoint_number,
                match endpoint.transfer_type {
                    crate::device_interface::TransferType::Bulk => "bulk",
                    crate::device_interface::TransferType::Interrupt => "interrupt",
                    crate::device_interface::TransferType::Control => "control",
                    crate::device_interface::TransferType::Isochronous => "isoc",
                }
            ));
        }
        manifest.push_str("  ]\n");
        manifest.push_str("}\n");
        Ok(manifest)
    }
}
