//! Linux kernel module backend code generator

use crate::dis::Instruction;
use crate::device_interface::DeviceInterface;
use crate::error::Result;
use super::{Backend, CodeGenerationOutput};
use crate::dis::ConvertedInstruction;

pub struct LinuxBackend;

impl LinuxBackend {
    pub fn new() -> Self {
        LinuxBackend
    }

    fn instruction_to_code(&self, instruction: &Instruction) -> String {
        match instruction {
            Instruction::MMIORead32 { addr } => {
                format!("uint32_t value = ioread32(0x{:x});", addr)
            }
            Instruction::MMIOWrite32 { addr, value } => {
                format!("iowrite32(0x{:x}, 0x{:x});", value, addr)
            }
            Instruction::MMIORead64 { addr } => {
                format!("uint64_t value = ioread64(0x{:x});", addr)
            }
            Instruction::MMIOWrite64 { addr, value } => {
                format!("iowrite64(0x{:x}, 0x{:x});", value, addr)
            }
            Instruction::USBBulkWrite {
                endpoint,
                buffer_size,
                timeout_ms,
            } => {
                format!(
                    "int pipe = usb_sndbulkpipe(dev, 0x{:02x});\n    \
                    int ret = usb_bulk_msg(dev, pipe, buffer, {}, &actual_length, {});",
                    endpoint, buffer_size, timeout_ms
                )
            }
            Instruction::USBBulkRead {
                endpoint,
                buffer_size,
                timeout_ms,
            } => {
                format!(
                    "int pipe = usb_rcvbulkpipe(dev, 0x{:02x});\n    \
                    int ret = usb_bulk_msg(dev, pipe, buffer, {}, &actual_length, {});",
                    endpoint, buffer_size, timeout_ms
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
                    "int ret = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),\n        \
                    0x{:02x}, 0x{:02x}, 0x{:04x}, 0x{:04x},\n        \
                    buffer, 0x{:04x}, {});",
                    request, request_type, value, index, length, timeout_ms
                )
            }
            Instruction::USBControlWrite {
                request_type,
                request,
                value,
                index,
                length,
                timeout_ms,
            } => {
                format!(
                    "int ret = usb_control_msg(dev, usb_sndctrlpipe(dev, 0),\n        \
                    0x{:02x}, 0x{:02x}, 0x{:04x}, 0x{:04x},\n        \
                    buffer, 0x{:04x}, {});",
                    request, request_type, value, index, length, timeout_ms
                )
            }
            Instruction::Delay { milliseconds } => {
                format!("msleep({});", milliseconds)
            }
            Instruction::SetupInterrupt {
                irq_number,
                handler_name,
            } => {
                format!(
                    "request_irq({}, {}, IRQF_SHARED, \"{}\", driver_data);",
                    irq_number, handler_name, handler_name
                )
            }
            Instruction::EnableInterrupt { irq_number } => {
                format!("enable_irq({});", irq_number)
            }
            Instruction::DisableInterrupt { irq_number } => {
                format!("disable_irq({});", irq_number)
            }
            Instruction::HandleError {
                error_code,
                handler,
            } => {
                format!(
                    "if (ret < 0) {{\n        \
                    printk(KERN_ERR \"Driver error 0x{:x}\");\n        \
                    {}();\n    }}",
                    error_code, handler
                )
            }
            _ => "// Unsupported instruction".to_string(),
        }
    }
}

impl Default for LinuxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for LinuxBackend {
    fn platform(&self) -> &str {
        "linux_kernel"
    }

    fn generate(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
    ) -> Result<CodeGenerationOutput> {
        self.validate(instructions)?;

        let mut code = String::new();

        // Headers
        code.push_str("#include <linux/kernel.h>\n");
        code.push_str("#include <linux/module.h>\n");
        code.push_str("#include <linux/usb.h>\n");
        code.push_str("#include <linux/io.h>\n");
        code.push_str("#include <linux/interrupt.h>\n");
        code.push_str("#include <linux/slab.h>\n\n");

        // Module metadata
        code.push_str("MODULE_LICENSE(\"GPL\");\n");
        code.push_str(&format!(
            "MODULE_DESCRIPTION(\"{} kernel driver\");\n",
            device.device_name
        ));
        code.push_str("MODULE_AUTHOR(\"UDC Generator\");\n\n");

        // Driver structure
        code.push_str(&format!(
            "static const struct usb_device_id {} = {{\n",
            device.device_name.replace(" ", "_").to_lowercase() + "_table"
        ));
        code.push_str(&format!(
            "    {{ USB_DEVICE(0x{:04x}, 0x{:04x}) }},\n",
            device.vendor_id, device.product_id
        ));
        code.push_str("    { /* Sentinel */ }\n");
        code.push_str("};\n");
        code.push_str("MODULE_DEVICE_TABLE(usb, device_table);\n\n");

        // Driver context
        code.push_str(&format!("struct {} {{\n", device.device_name.replace(" ", "_") + "_ctx"));
        code.push_str("    struct usb_device *dev;\n");
        code.push_str("    struct urb *urb;\n");
        code.push_str("    unsigned char *buffer;\n");
        code.push_str("};\n\n");

        // Probe function
        code.push_str("static int driver_probe(struct usb_interface *interface,\n");
        code.push_str("                        const struct usb_device_id *id) {\n");
        code.push_str("    struct usb_device *dev = interface_to_usbdev(interface);\n");
        code.push_str("    struct device_ctx *ctx;\n\n");

        code.push_str("    ctx = devm_kzalloc(&interface->dev, sizeof(*ctx), GFP_KERNEL);\n");
        code.push_str("    if (!ctx) return -ENOMEM;\n\n");

        code.push_str("    ctx->dev = dev;\n");
        code.push_str("    usb_set_intfdata(interface, ctx);\n\n");

        // Convert instructions
        for instr in instructions {
            let line = self.instruction_to_code(instr);
            code.push_str("    ");
            code.push_str(&line);
            code.push('\n');
        }

        code.push_str("\n    return 0;\n");
        code.push_str("}\n\n");

        // Disconnect function
        code.push_str("static void driver_disconnect(struct usb_interface *interface) {\n");
        code.push_str("    struct device_ctx *ctx = usb_get_intfdata(interface);\n");
        code.push_str("    if (!ctx) return;\n");
        code.push_str("    kfree(ctx->buffer);\n");
        code.push_str("}\n\n");

        // USB driver structure
        code.push_str("static struct usb_driver driver = {\n");
        code.push_str(&format!("    .name = \"{}\",\n", device.device_name));
        code.push_str("    .probe = driver_probe,\n");
        code.push_str("    .disconnect = driver_disconnect,\n");
        code.push_str("    .id_table = device_table,\n");
        code.push_str("};\n\n");

        // Module init/exit
        code.push_str("module_usb_driver(driver);\n");

        let makefile = "obj-m += driver.o\n\nall:\n\tmake -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules\n\nclean:\n\tmake -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean\n".to_string();

        let output = CodeGenerationOutput::new(
            "linux_kernel".to_string(),
            code,
        )
        .with_config_files(vec![("Makefile".to_string(), makefile)])
        .with_build_instructions("make -C /lib/modules/$(uname -r)/build M=$(pwd) modules".to_string());

        Ok(output)
    }

    fn validate(&self, instructions: &[Instruction]) -> Result<()> {
        for instr in instructions {
            match instr {
                Instruction::Comment { .. } => {
                    // Always valid
                }
                _ => {
                    // All other instructions are valid for Linux
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
            platform: "linux_kernel".to_string(),
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
                vec!["<linux/usb.h>".to_string()]
            }
            Instruction::SetupInterrupt { .. }
            | Instruction::EnableInterrupt { .. }
            | Instruction::DisableInterrupt { .. } => {
                vec!["<linux/interrupt.h>".to_string()]
            }
            Instruction::MMIORead32 { .. } | Instruction::MMIORead64 { .. }
            | Instruction::MMIOWrite32 { .. } | Instruction::MMIOWrite64 { .. } => {
                vec!["<linux/io.h>".to_string()]
            }
            _ => vec![],
        }
    }

    fn get_error_handling(&self, instruction: &Instruction) -> Option<String> {
        match instruction {
            Instruction::USBBulkWrite { .. } | Instruction::USBBulkRead { .. } => {
                Some("if (ret < 0) { printk(KERN_ERR \"USB transfer failed\"); return ret; }".to_string())
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
        manifest.push_str("  \"module_type\": \"kernel_module\",\n");
        manifest.push_str("  \"endpoints\": [\n");
        for endpoint in &device.endpoints {
            manifest.push_str(&format!(
                "    {{ \"number\": {}, \"type\": \"{}\" }},\n",
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
