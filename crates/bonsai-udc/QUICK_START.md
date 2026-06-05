# Universal Driver Compiler - Quick Start Guide

## 30-Second Overview

The UDC converts hardware device specifications into **compilable driver code** for Linux, macOS, and UOSC. Write once, deploy everywhere.

```bash
udc convert --input device.json --target linux-kernel --output ./output
```

## Installation

```bash
cd crates/bonsai-udc
cargo build --release --bin udc
export PATH="$PWD/target/release:$PATH"
```

## Basic Usage

### 1. Convert a Device (Linux Example)

Create a device spec (`brother_fax.json`):
```json
{
  "device": {
    "device_name": "Brother FAX 2840",
    "vendor_id": 4170,
    "product_id": 6418,
    "endpoints": [
      {
        "endpoint_number": 1,
        "direction": "Out",
        "transfer_type": "Bulk",
        "max_packet_size": 64
      }
    ],
    "capabilities": {
      "supports_dma": true,
      "supports_interrupts": true,
      "supports_power_management": true,
      "max_concurrent_transfers": 4,
      "alignment_requirement": 4
    }
  },
  "instructions": [
    {
      "type": "USBBulkWrite",
      "endpoint": 1,
      "buffer_size": 512,
      "timeout_ms": 5000
    }
  ]
}
```

Convert to Linux:
```bash
udc convert --input brother_fax.json --target linux-kernel --output ./linux_driver
```

**Output**: `./linux_driver/brother_fax.cpp` (150+ lines of compilable C code)

### 2. Convert to macOS

```bash
udc convert --input brother_fax.json --target macos-driverkit --output ./macos_driver
```

**Output**: `./macos_driver/brother_fax.cpp` (C++ DriverKit code)

### 3. Convert to UOSC

```bash
udc convert --input brother_fax.json --target UOSC --output ./UOSC_driver
```

**Output**: `./UOSC_driver/brother_fax.cpp` (Rust async/await code)

## CLI Commands

### Convert
Convert a device specification to driver code:
```bash
udc convert \
  --input device.json \        # Input DIS file
  --target linux-kernel \      # Target: linux-kernel, macos-driverkit, UOSC
  --output ./output            # Output directory
```

### Install
Register a converted driver:
```bash
udc install \
  --vendor 0x04f9 \           # Vendor ID (hex)
  --device 0x1917 \           # Device ID (hex)
  --target linux-kernel \     # Target platform
  --source ./output           # Source directory
```

### List
Show installed drivers:
```bash
udc list                       # All drivers
udc list --os linux-kernel    # Filter by platform
```

### Rollback
Revert to a previous version:
```bash
udc rollback \
  --vendor 0x04f9 \
  --device 0x1917 \
  --target linux-kernel \
  --version 1.0.0
```

### Help
Show command help:
```bash
udc help
```

## Programmatic Usage

### Rust Library

```rust
use bonsai_udc::{DriverConverter, DriverConversionContext};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create converter
    let converter = DriverConverter::with_default_engine();

    // Load device spec
    let context = DriverConversionContext::from_file(
        Path::new("device.json"),
        "linux_kernel"
    )?;

    // Convert
    let output = converter.convert_driver(&context)?;

    // Save to disk
    output.save_to_disk(Path::new("./output"))?;

    // Print summary
    println!("{}", output.summary());

    Ok(())
}
```

### Convert All Platforms

```rust
use bonsai_udc::DriverConverter;

let converter = DriverConverter::with_default_engine();
let context = DriverConversionContext::from_file(
    Path::new("device.json"),
    "linux_kernel"
)?;

let outputs = converter.convert_all_platforms(&context)?;
for output in outputs {
    println!("Platform: {}", output.conversion_result.platform);
    println!("Size: {} bytes", output.conversion_result.metrics.output_size_bytes);
}
```

## Output Files

After conversion, you'll get:

```
output/
├── device.cpp              # Main driver source code
├── device.h                # Header files (if needed)
├── Makefile                # Build configuration (Linux)
├── Cargo.toml              # Rust manifest (UOSC)
└── metadata.json           # Conversion metadata
```

### metadata.json
```json
{
  "device_name": "Brother FAX 2840",
  "vendor_id": "0x04f9",
  "device_id": "0x1917",
  "platform": "linux_kernel",
  "metrics": {
    "total_instructions": 5,
    "converted_instructions": 5,
    "rules_applied": 2,
    "generation_time_ms": 45,
    "output_size_bytes": 1247
  },
  "success": true,
  "timestamp": "2026-06-05T12:34:56Z"
}
```

## Device Specification (DIS)

### Device Section

```json
{
  "device": {
    "device_name": "String",          // Human-readable name
    "vendor_id": 0x1234,              // USB vendor ID
    "product_id": 0x5678,             // USB product ID
    "device_class": 255,              // USB class code
    "device_subclass": 0,             // USB subclass
    "device_protocol": 0,             // USB protocol
    "endpoints": [...],               // USB endpoints
    "mmio_regions": [...],            // Memory-mapped regions
    "interrupts": [...],              // IRQ handlers
    "capabilities": {...},            // Device features
    "description": "String",
    "version": "1.0.0"
  }
}
```

### Endpoints

```json
{
  "endpoint_number": 1,
  "direction": "In" | "Out" | "Bidirectional",
  "transfer_type": "Control" | "Isochronous" | "Bulk" | "Interrupt",
  "max_packet_size": 64,
  "interval": 0
}
```

### Instructions

Supported instruction types:

```json
{
  "type": "MMIORead32",
  "addr": "0xf0000000"
}

{
  "type": "MMIOWrite32",
  "addr": "0xf0000000",
  "value": 0x12345678
}

{
  "type": "USBBulkWrite",
  "endpoint": 1,
  "buffer_size": 512,
  "timeout_ms": 5000
}

{
  "type": "USBBulkRead",
  "endpoint": 129,
  "buffer_size": 4096,
  "timeout_ms": 5000
}

{
  "type": "USBControlWrite",
  "request_type": 64,
  "request": 64,
  "value": 0,
  "index": 0,
  "length": 0,
  "timeout_ms": 1000
}

{
  "type": "SetupInterrupt",
  "irq_number": 45,
  "handler_name": "irq_handler"
}

{
  "type": "Delay",
  "milliseconds": 100
}

{
  "type": "Comment",
  "text": "Initialize device"
}
```

## Example: Brother FAX 2840

The repository includes a complete working example:

```bash
cargo run --example full_conversion
```

This converts `examples/brother_fax_driver.json` to all three platforms.

## Generated Code Samples

### Linux Kernel (C)
```c
#include <linux/module.h>
#include <linux/usb.h>

static int driver_probe(struct usb_interface *interface,
                        const struct usb_device_id *id) {
    int pipe = usb_sndbulkpipe(dev, 0x01);
    int ret = usb_bulk_msg(dev, pipe, buffer, 512, &actual_length, 5000);
    if (ret < 0) {
        printk(KERN_ERR "USB transfer failed");
        return ret;
    }
    return 0;
}
```

### macOS DriverKit (C++)
```cpp
#include <DriverKit/IOService.h>
#include <DriverKit/IOUSBHostDevice.h>

bool BrotherFAX2840_Driver::start(IOService *provider) {
    IOReturn ret = bulkPipe->Send(descriptor, 5000, nullptr, nullptr);
    if (ret != kIOReturnSuccess) return false;
    return true;
}
```

### UOSC (Rust)
```rust
async fn initialize(&mut self) -> Result<(), DriverError> {
    let result = self.device.bulk_write(0x01, buffer.as_ref(), 512, 5000).await;
    result?;
    Ok(())
}
```

## Supported Platforms

| Platform | Output Language | Build Tool | Status |
|----------|-----------------|------------|--------|
| Linux | C | Makefile | ✅ Production |
| macOS | C++ | Xcode | ✅ Production |
| UOSC | Rust | Cargo | ✅ Production |

## Troubleshooting

### "No such file or directory" error
Make sure the input JSON file exists and the path is correct:
```bash
udc convert --input ./path/to/file.json --target linux-kernel --output ./output
```

### "Unsupported platform"
Check that the target is one of:
- `linux_kernel`
- `macos_driverkit`
- `UOSC`

### "Invalid hex ID"
Vendor and device IDs must be in hex format (with or without 0x prefix):
```bash
udc install --vendor 0x04f9 --device 0x1917 ...
# or
udc install --vendor 04f9 --device 1917 ...
```

## Tips & Tricks

### Batch Convert
```bash
for target in linux-kernel macos-driverkit UOSC; do
  udc convert --input device.json --target "$target" --output "./output_$target"
done
```

### Check Generated Files
```bash
ls -la output/
cat output/metadata.json | jq .metrics
```

### Register Multiple Versions
```bash
# Version 1.0.0
udc install --vendor 0x04f9 --device 0x1917 --target linux-kernel --source ./v1.0.0

# Version 1.1.0
udc install --vendor 0x04f9 --device 0x1917 --target linux-kernel --source ./v1.1.0

# List all
udc list

# Rollback if needed
udc rollback --vendor 0x04f9 --device 0x1917 --target linux-kernel --version 1.0.0
```

## Next Steps

1. Review your device specification JSON
2. Run the conversion for your target platform
3. Inspect the generated code
4. Build and test the driver
5. Register it in the driver registry
6. Deploy!

## Documentation

- **User Guide**: See `UDC_COMPLETE.md` for comprehensive documentation
- **Implementation Details**: See `IMPLEMENTATION_SUMMARY.md` for architecture
- **API Reference**: Check the source code in `src/` for detailed docs

## Support

For questions or issues:
1. Check the example in `examples/brother_fax_driver.json`
2. Review the user guide in `UDC_COMPLETE.md`
3. Run the integration test: `cargo test`
4. Check the source documentation: `cargo doc --open`

---

**Happy driver converting!** 🚀
