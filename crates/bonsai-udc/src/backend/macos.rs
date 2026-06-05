//! macOS DriverKit backend code generator

use crate::dis::Instruction;
use crate::device_interface::DeviceInterface;
use crate::error::Result;
use super::{Backend, CodeGenerationOutput};
use crate::dis::ConvertedInstruction;

pub struct MacOsBackend;

impl MacOsBackend {
    pub fn new() -> Self {
        MacOsBackend
    }

    fn instruction_to_code(&self, instruction: &Instruction) -> String {
        match instruction {
            Instruction::MMIORead32 { addr } => {
                format!("uint32_t value = OSReadLittleInt32((void*)0x{:x}, 0);", addr)
            }
            Instruction::MMIOWrite32 { addr, value } => {
                format!(
                    "OSWriteLittleInt32((void*)0x{:x}, 0, 0x{:x});",
                    addr, value
                )
            }
            Instruction::MMIORead64 { addr } => {
                format!("uint64_t value = OSReadLittleInt64((void*)0x{:x}, 0);", addr)
            }
            Instruction::MMIOWrite64 { addr, value } => {
                format!(
                    "OSWriteLittleInt64((void*)0x{:x}, 0, 0x{:x});",
                    addr, value
                )
            }
            Instruction::USBBulkWrite {
                endpoint,
                buffer_size,
                timeout_ms,
            } => {
                format!(
                    "IOReturn ret = device->CreateIOMemoryDescriptor(buffer, {}, kIODirectionOut, &descriptor);\n    \
                    ret = bulkPipe->Send(descriptor, {}, nullptr, nullptr);",
                    buffer_size, timeout_ms
                )
            }
            Instruction::USBBulkRead {
                endpoint,
                buffer_size,
                timeout_ms,
            } => {
                format!(
                    "IOReturn ret = device->CreateIOMemoryDescriptor(buffer, {}, kIODirectionIn, &descriptor);\n    \
                    ret = bulkPipe->Recv(descriptor, {}, nullptr, nullptr);",
                    buffer_size, timeout_ms
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
                    "IOUSBDeviceRequest req = {{ \n        .bmRequestType = 0x{:x},\n        .bRequest = 0x{:x},\n        \
                    .wValue = 0x{:x},\n        .wIndex = 0x{:x},\n        .wLength = 0x{:x},\n        .wTimeout = {} \n    }};\n    \
                    ret = deviceInterface->DeviceRequest(&req, data);",
                    request_type, request, value, index, length, timeout_ms
                )
            }
            Instruction::Delay { milliseconds } => {
                format!("IOSleep({});", milliseconds)
            }
            Instruction::SetupInterrupt {
                irq_number,
                handler_name,
            } => {
                format!(
                    "IOInterruptEventSource *interruptSource = \n    \
                    IOInterruptEventSource::interruptEventSource(this, \n    \
                    OSMemberFunctionCast(IOInterruptEventAction, this, &{});\n    \
                    workLoop->addEventSource(interruptSource);",
                    handler_name
                )
            }
            Instruction::EnableInterrupt { irq_number } => {
                format!("interruptSource->enable();")
            }
            Instruction::DisableInterrupt { irq_number } => {
                format!("interruptSource->disable();")
            }
            Instruction::HandleError {
                error_code,
                handler,
            } => {
                format!(
                    "if (ret != kIOReturnSuccess) {{\n        \
                    IOLog(\"Error 0x{:x}: {}\");\n        \
                    {}();\n    }}",
                    error_code, handler, handler
                )
            }
            _ => "// Unsupported instruction".to_string(),
        }
    }
}

impl Default for MacOsBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for MacOsBackend {
    fn platform(&self) -> &str {
        "macos_driverkit"
    }

    fn generate(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
    ) -> Result<CodeGenerationOutput> {
        self.validate(instructions)?;

        let mut code = String::new();

        // Header
        code.push_str("#include <DriverKit/IOService.h>\n");
        code.push_str("#include <DriverKit/IOUSBHostDevice.h>\n");
        code.push_str("#include <DriverKit/OSAction.h>\n");
        code.push_str("#include <DriverKit/IOMemoryDescriptor.h>\n\n");

        // Class declaration
        code.push_str(&format!(
            "class {}_Driver : public IOService {{\n",
            device.device_name.replace(" ", "_")
        ));
        code.push_str("private:\n");
        code.push_str("    IOUSBHostDevice *device;\n");
        code.push_str("    IOUSBHostPipe *bulkPipe;\n");
        code.push_str("    IOInterruptEventSource *interruptSource;\n");
        code.push_str("    IOMemoryDescriptor *descriptor;\n\n");
        code.push_str("public:\n");
        code.push_str("    virtual bool init();\n");
        code.push_str("    virtual void free();\n");
        code.push_str("    virtual IOService *probe(IOService *provider, SInt32 *score);\n");
        code.push_str("    virtual bool start(IOService *provider);\n");
        code.push_str("    virtual void stop(IOService *provider);\n");
        code.push_str("};\n\n");

        // Implementation start
        code.push_str(&format!(
            "bool {}_Driver::start(IOService *provider) {{\n",
            device.device_name.replace(" ", "_")
        ));
        code.push_str("    if (!super::start(provider)) return false;\n\n");
        code.push_str("    device = OSDynamicCast(IOUSBHostDevice, provider);\n");
        code.push_str("    if (!device) return false;\n\n");

        // Convert instructions
        for instr in instructions {
            let line = self.instruction_to_code(instr);
            code.push_str("    ");
            code.push_str(&line);
            code.push('\n');
        }

        code.push_str("\n    return true;\n");
        code.push_str("}\n\n");

        code.push_str(&format!(
            "void {}_Driver::stop(IOService *provider) {{\n",
            device.device_name.replace(" ", "_")
        ));
        code.push_str("    if (interruptSource) {\n");
        code.push_str("        interruptSource->disable();\n");
        code.push_str("    }\n");
        code.push_str("    super::stop(provider);\n");
        code.push_str("}\n");

        let output = CodeGenerationOutput::new(
            "macos_driverkit".to_string(),
            code,
        )
        .with_build_instructions(
            "xcode build -project *.xcodeproj -scheme * -configuration Release".to_string(),
        );

        Ok(output)
    }

    fn validate(&self, instructions: &[Instruction]) -> Result<()> {
        for instr in instructions {
            match instr {
                Instruction::Comment { .. } => {
                    // Always valid
                }
                _ => {
                    // All other instructions are valid for macOS
                }
            }
        }
        Ok(())
    }

    fn convert_instruction(
        &self,
        instruction: &Instruction,
        device: &DeviceInterface,
    ) -> Result<ConvertedInstruction> {
        let code = self.instruction_to_code(instruction);
        Ok(ConvertedInstruction {
            instruction: instruction.clone(),
            platform: "macos_driverkit".to_string(),
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
                vec!["<DriverKit/IOUSBHostDevice.h>".to_string()]
            }
            Instruction::SetupInterrupt { .. }
            | Instruction::EnableInterrupt { .. }
            | Instruction::DisableInterrupt { .. } => {
                vec!["<DriverKit/IOInterruptEventSource.h>".to_string()]
            }
            _ => vec![],
        }
    }

    fn get_error_handling(&self, instruction: &Instruction) -> Option<String> {
        match instruction {
            Instruction::USBBulkWrite { .. } | Instruction::USBBulkRead { .. } => {
                Some("if (ret != kIOReturnSuccess) return ret;".to_string())
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
        manifest.push_str("  \"capabilities\": [\n");
        manifest.push_str("    \"com.apple.driverkit.usb.device\",\n");
        manifest.push_str("    \"com.apple.driverkit.kernel.iokit\"\n");
        manifest.push_str("  ],\n");
        manifest.push_str("  \"endpoints\": [\n");
        for endpoint in &device.endpoints {
            manifest.push_str(&format!(
                "    {{ \"number\": {}, \"direction\": \"{}\" }},\n",
                endpoint.endpoint_number,
                match endpoint.direction {
                    crate::device_interface::EndpointDirection::In => "in",
                    crate::device_interface::EndpointDirection::Out => "out",
                    crate::device_interface::EndpointDirection::Bidirectional => "inout",
                }
            ));
        }
        manifest.push_str("  ]\n");
        manifest.push_str("}\n");
        Ok(manifest)
    }
}
