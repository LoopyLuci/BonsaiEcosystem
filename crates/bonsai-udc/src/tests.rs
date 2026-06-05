//! Comprehensive test suite for UDC backend code generators

#[cfg(test)]
mod tests {
    use crate::backend::{Backend, MacOsBackend, LinuxBackend, UsosBackend};
    use crate::device_interface::*;
    use crate::dis::*;

    /// Helper to create a test device
    fn create_test_device() -> DeviceInterface {
        DeviceInterface::new(
            "TestUSBDevice".to_string(),
            0x1234,
            0x5678,
        )
        .add_endpoint(EndpointDescriptor {
            endpoint_number: 1,
            direction: EndpointDirection::Out,
            transfer_type: TransferType::Bulk,
            max_packet_size: 512,
            interval: 0,
        })
        .add_endpoint(EndpointDescriptor {
            endpoint_number: 2,
            direction: EndpointDirection::In,
            transfer_type: TransferType::Bulk,
            max_packet_size: 512,
            interval: 0,
        })
        .with_description("Test USB Device for DIS code generation".to_string())
    }

    /// Helper to create simple instruction stream
    fn create_test_instructions() -> Vec<Instruction> {
        vec![
            Instruction::Comment {
                text: "Initialize device".to_string(),
            },
            Instruction::Delay {
                milliseconds: 100,
            },
            Instruction::MMIOWrite32 {
                addr: 0x80000000,
                value: 0xDEADBEEF,
            },
            Instruction::MMIORead32 {
                addr: 0x80000004,
            },
            Instruction::USBBulkWrite {
                endpoint: 1,
                buffer_size: 256,
                timeout_ms: 5000,
            },
            Instruction::USBBulkRead {
                endpoint: 2,
                buffer_size: 256,
                timeout_ms: 5000,
            },
        ]
    }

    // ============================================================================
    // macOS DriverKit Backend Tests
    // ============================================================================

    #[test]
    fn test_macos_backend_platform_name() {
        let backend = MacOsBackend::new();
        assert_eq!(backend.platform(), "macos_driverkit");
    }

    #[test]
    fn test_macos_backend_generates_code() {
        let backend = MacOsBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.source_code.is_empty());
        assert_eq!(output.platform, "macos_driverkit");
    }

    #[test]
    fn test_macos_backend_includes_required_headers() {
        let backend = MacOsBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("DriverKit"));
    }

    #[test]
    fn test_macos_backend_mmio_operations() {
        let backend = MacOsBackend::new();
        let device = create_test_device();
        let instructions = vec![
            Instruction::MMIORead32 { addr: 0x1000 },
            Instruction::MMIOWrite32 {
                addr: 0x1004,
                value: 0xABCD,
            },
        ];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("0x1000"));
        assert!(output.source_code.contains("0xabcd"));
    }

    #[test]
    fn test_macos_backend_usb_bulk_operations() {
        let backend = MacOsBackend::new();
        let device = create_test_device();
        let instructions = vec![
            Instruction::USBBulkWrite {
                endpoint: 1,
                buffer_size: 512,
                timeout_ms: 5000,
            },
            Instruction::USBBulkRead {
                endpoint: 2,
                buffer_size: 512,
                timeout_ms: 5000,
            },
        ];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("USBBulkWrite"));
        assert!(output.source_code.contains("USBBulkRead"));
    }

    #[test]
    fn test_macos_backend_capability_manifest() {
        let backend = MacOsBackend::new();
        let device = create_test_device();

        let result = backend.generate_capability_manifest(&device);
        assert!(result.is_ok());

        let manifest = result.unwrap();
        assert!(manifest.contains("TestUSBDevice"));
        assert!(manifest.contains("0x1234"));
        assert!(manifest.contains("0x5678"));
    }

    // ============================================================================
    // Linux Kernel Backend Tests
    // ============================================================================

    #[test]
    fn test_linux_backend_platform_name() {
        let backend = LinuxBackend::new();
        assert_eq!(backend.platform(), "linux_kernel");
    }

    #[test]
    fn test_linux_backend_generates_code() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.source_code.is_empty());
        assert_eq!(output.platform, "linux_kernel");
    }

    #[test]
    fn test_linux_backend_includes_kernel_headers() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("#include <linux/"));
        assert!(output.source_code.contains("MODULE_LICENSE"));
        assert!(output.source_code.contains("MODULE_DEVICE_TABLE"));
    }

    #[test]
    fn test_linux_backend_usb_device_table() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = vec![Instruction::Comment {
            text: "test".to_string(),
        }];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("USB_DEVICE(0x1234, 0x5678)"));
        assert!(output.source_code.contains("usb_driver"));
    }

    #[test]
    fn test_linux_backend_probe_disconnect_functions() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("driver_probe"));
        assert!(output.source_code.contains("driver_disconnect"));
    }

    #[test]
    fn test_linux_backend_makefile_generation() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.configuration_files.is_empty());

        let makefile = output
            .configuration_files
            .iter()
            .find(|(name, _)| name == "Makefile");
        assert!(makefile.is_some());

        let (_, content) = makefile.unwrap();
        assert!(content.contains("obj-m"));
        assert!(content.contains("make -C"));
    }

    #[test]
    fn test_linux_backend_mmio_operations() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = vec![
            Instruction::MMIORead32 { addr: 0x2000 },
            Instruction::MMIOWrite32 {
                addr: 0x2004,
                value: 0x1234,
            },
        ];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("ioread32"));
        assert!(output.source_code.contains("iowrite32"));
    }

    #[test]
    fn test_linux_backend_usb_bulk_message() {
        let backend = LinuxBackend::new();
        let device = create_test_device();
        let instructions = vec![
            Instruction::USBBulkWrite {
                endpoint: 0x01,
                buffer_size: 256,
                timeout_ms: 3000,
            },
            Instruction::USBBulkRead {
                endpoint: 0x82,
                buffer_size: 256,
                timeout_ms: 3000,
            },
        ];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("usb_bulk_msg"));
        assert!(output.source_code.contains("usb_sndbulkpipe"));
        assert!(output.source_code.contains("usb_rcvbulkpipe"));
    }

    #[test]
    fn test_linux_backend_capability_manifest() {
        let backend = LinuxBackend::new();
        let device = create_test_device();

        let result = backend.generate_capability_manifest(&device);
        assert!(result.is_ok());

        let manifest = result.unwrap();
        assert!(manifest.contains("TestUSBDevice"));
        assert!(manifest.contains("kernel_module"));
    }

    // ============================================================================
    // UOSC Native Backend Tests
    // ============================================================================

    #[test]
    fn test_UOSC_backend_platform_name() {
        let backend = UsosBackend::new();
        assert_eq!(backend.platform(), "UOSC");
    }

    #[test]
    fn test_UOSC_backend_generates_code() {
        let backend = UsosBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.source_code.is_empty());
        assert_eq!(output.platform, "UOSC");
    }

    #[test]
    fn test_UOSC_backend_generates_titan_code() {
        let backend = UsosBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        // Should contain Titan/Rust syntax
        assert!(output.source_code.contains("impl Driver"));
        assert!(output.source_code.contains("async fn"));
    }

    #[test]
    fn test_UOSC_backend_device_metadata() {
        let backend = UsosBackend::new();
        let device = create_test_device();
        let instructions = vec![Instruction::Comment {
            text: "test".to_string(),
        }];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("0x1234"));
        assert!(output.source_code.contains("0x5678"));
        assert!(output.source_code.contains("TestUSBDevice"));
    }

    #[test]
    fn test_UOSC_backend_cargo_toml_generation() {
        let backend = UsosBackend::new();
        let device = create_test_device();
        let instructions = create_test_instructions();

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.configuration_files.is_empty());

        let cargo = output
            .configuration_files
            .iter()
            .find(|(name, _)| name == "Cargo.toml");
        assert!(cargo.is_some());

        let (_, content) = cargo.unwrap();
        assert!(content.contains("[package]"));
        assert!(content.contains("UOSC-test-usb-device-driver"));
    }

    #[test]
    fn test_UOSC_backend_mmio_operations() {
        let backend = UsosBackend::new();
        let device = create_test_device();
        let instructions = vec![
            Instruction::MMIORead32 { addr: 0x3000 },
            Instruction::MMIOWrite32 {
                addr: 0x3004,
                value: 0x5678,
            },
        ];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("ptr::read_volatile"));
        assert!(output.source_code.contains("ptr::write_volatile"));
    }

    #[test]
    fn test_UOSC_backend_async_usb_operations() {
        let backend = UsosBackend::new();
        let device = create_test_device();
        let instructions = vec![
            Instruction::USBBulkWrite {
                endpoint: 1,
                buffer_size: 512,
                timeout_ms: 5000,
            },
            Instruction::USBBulkRead {
                endpoint: 2,
                buffer_size: 512,
                timeout_ms: 5000,
            },
        ];

        let result = backend.generate(&instructions, &device);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.source_code.contains("bulk_write"));
        assert!(output.source_code.contains("bulk_read"));
        assert!(output.source_code.contains(".await"));
    }

    #[test]
    fn test_UOSC_backend_capability_manifest() {
        let backend = UsosBackend::new();
        let device = create_test_device();

        let result = backend.generate_capability_manifest(&device);
        assert!(result.is_ok());

        let manifest = result.unwrap();
        assert!(manifest.contains("TestUSBDevice"));
        assert!(manifest.contains("\"platform\": \"UOSC\""));
        assert!(manifest.contains("usb_device_access"));
    }

    // ============================================================================
    // Cross-Platform Backend Tests
    // ============================================================================

    #[test]
    fn test_all_backends_generate_for_same_device() {
        let device = create_test_device();
        let instructions = create_test_instructions();

        let macos_result = MacOsBackend::new().generate(&instructions, &device);
        let linux_result = LinuxBackend::new().generate(&instructions, &device);
        let UOSC_result = UsosBackend::new().generate(&instructions, &device);

        assert!(macos_result.is_ok());
        assert!(linux_result.is_ok());
        assert!(UOSC_result.is_ok());

        let macos_output = macos_result.unwrap();
        let linux_output = linux_result.unwrap();
        let UOSC_output = UOSC_result.unwrap();

        assert_eq!(macos_output.platform, "macos_driverkit");
        assert_eq!(linux_output.platform, "linux_kernel");
        assert_eq!(UOSC_output.platform, "UOSC");
    }

    #[test]
    fn test_instruction_stream_creation() {
        let instructions = create_test_instructions();
        let stream = InstructionStream::new(
            instructions.clone(),
            "USB".to_string(),
            "Test USB driver instruction stream".to_string(),
        );

        assert_eq!(stream.metadata.target_paradigm, "USB");
        assert_eq!(stream.instructions.len(), instructions.len());
    }

    #[test]
    fn test_instruction_categorization() {
        let mmio_read = Instruction::MMIORead32 { addr: 0x1000 };
        let usb_write = Instruction::USBBulkWrite {
            endpoint: 1,
            buffer_size: 256,
            timeout_ms: 5000,
        };
        let delay = Instruction::Delay { milliseconds: 100 };

        assert_eq!(mmio_read.category(), InstructionCategory::Io);
        assert_eq!(usb_write.category(), InstructionCategory::Usb);
        assert_eq!(delay.category(), InstructionCategory::Utility);
    }

    #[test]
    fn test_converted_instruction_building() {
        let instruction = Instruction::USBBulkWrite {
            endpoint: 1,
            buffer_size: 256,
            timeout_ms: 5000,
        };

        let converted = ConvertedInstruction::new(
            instruction,
            "linux_kernel".to_string(),
            "usb_bulk_msg(...)".to_string(),
        )
        .with_includes(vec!["<linux/usb.h>".to_string()])
        .with_error_handling("if (ret < 0) return ret;".to_string());

        assert_eq!(converted.platform, "linux_kernel");
        assert!(!converted.required_includes.is_empty());
        assert!(converted.error_handling.is_some());
    }

    #[test]
    fn test_device_interface_builder() {
        let device = DeviceInterface::new(
            "TestDevice".to_string(),
            0xABCD,
            0xEF01,
        )
        .add_endpoint(EndpointDescriptor {
            endpoint_number: 1,
            direction: EndpointDirection::Out,
            transfer_type: TransferType::Bulk,
            max_packet_size: 512,
            interval: 0,
        })
        .add_mmio_region(MmioRegion {
            name: "control".to_string(),
            base_address: 0x80000000,
            size: 0x1000,
            access_type: AccessType::ReadWrite,
        });

        assert_eq!(device.device_name, "TestDevice");
        assert_eq!(device.vendor_id, 0xABCD);
        assert_eq!(device.product_id, 0xEF01);
        assert_eq!(device.endpoints.len(), 1);
        assert_eq!(device.mmio_regions.len(), 1);
    }

    #[test]
    fn test_conversion_with_multiple_backends() {
        let backends: Vec<Box<dyn Backend>> = vec![
            Box::new(MacOsBackend::new()),
            Box::new(LinuxBackend::new()),
            Box::new(UsosBackend::new()),
        ];

        let device = create_test_device();
        let instruction = Instruction::MMIOWrite32 {
            addr: 0x4000,
            value: 0xDEAD,
        };

        for backend in backends {
            let result = backend.convert_instruction(&instruction, &device);
            assert!(result.is_ok());

            let converted = result.unwrap();
            assert_eq!(converted.instruction, instruction);
            assert!(!converted.generated_code.is_empty());
        }
    }

    #[test]
    fn test_error_handling_generation() {
        let macos = MacOsBackend::new();
        let linux = LinuxBackend::new();
        let UOSC = UsosBackend::new();

        let usb_write = Instruction::USBBulkWrite {
            endpoint: 1,
            buffer_size: 256,
            timeout_ms: 5000,
        };

        let macos_err = macos.get_error_handling(&usb_write);
        let linux_err = linux.get_error_handling(&usb_write);
        let UOSC_err = UOSC.get_error_handling(&usb_write);

        // All platforms should have error handling for USB operations
        assert!(macos_err.is_some());
        assert!(linux_err.is_some());
        assert!(UOSC_err.is_some());
    }

    #[test]
    fn test_include_generation() {
        let macos = MacOsBackend::new();
        let linux = LinuxBackend::new();
        let UOSC = UsosBackend::new();

        let mmio_op = Instruction::MMIORead32 { addr: 0x5000 };
        let usb_op = Instruction::USBBulkRead {
            endpoint: 2,
            buffer_size: 512,
            timeout_ms: 5000,
        };

        // MMIO operations need different includes per platform
        let macos_mmio = macos.get_includes_for_instruction(&mmio_op);
        let linux_mmio = linux.get_includes_for_instruction(&mmio_op);

        // USB operations also need different includes
        let macos_usb = macos.get_includes_for_instruction(&usb_op);
        let linux_usb = linux.get_includes_for_instruction(&usb_op);
        let UOSC_usb = UOSC.get_includes_for_instruction(&usb_op);

        assert!(!macos_mmio.is_empty() || !linux_mmio.is_empty());
        assert!(!macos_usb.is_empty());
        assert!(!linux_usb.is_empty());
        assert!(!UOSC_usb.is_empty());
    }

    #[test]
    fn test_complex_instruction_sequence() {
        let device = create_test_device();
        let instructions = vec![
            Instruction::Comment {
                text: "Initialize hardware".to_string(),
            },
            Instruction::Delay {
                milliseconds: 100,
            },
            Instruction::MMIOWrite32 {
                addr: 0xDEAD0000,
                value: 0x12345678,
            },
            Instruction::MMIORead32 {
                addr: 0xDEAD0004,
            },
            Instruction::USBControlWrite {
                request_type: 0x40,
                request: 0x01,
                value: 0x0000,
                index: 0x0000,
                length: 8,
                timeout_ms: 5000,
            },
            Instruction::USBBulkWrite {
                endpoint: 1,
                buffer_size: 64,
                timeout_ms: 5000,
            },
            Instruction::USBBulkRead {
                endpoint: 0x82,
                buffer_size: 64,
                timeout_ms: 5000,
            },
            Instruction::Delay {
                milliseconds: 50,
            },
        ];

        let backends: Vec<Box<dyn Backend>> = vec![
            Box::new(MacOsBackend::new()),
            Box::new(LinuxBackend::new()),
            Box::new(UsosBackend::new()),
        ];

        for backend in backends {
            let result = backend.generate(&instructions, &device);
            assert!(result.is_ok(), "Backend {} should generate code", backend.platform());

            let output = result.unwrap();
            assert!(!output.source_code.is_empty());
            assert_eq!(output.platform, backend.platform());
        }
    }
}
