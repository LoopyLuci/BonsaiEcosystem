# UDC API Reference

Complete API documentation for the Universal Driver Compiler (UDC).

## Table of Contents

- [Core Types](#core-types)
- [Backend Trait](#backend-trait)
- [Error Types](#error-types)
- [Helper Functions](#helper-functions)
- [Examples](#examples)

## Core Types

### `Instruction` Enum

Represents a single device operation.

```rust
pub enum Instruction {
    // Memory-Mapped I/O Operations
    MMIORead32 { addr: u64 },
    MMIOWrite32 { addr: u64, value: u32 },
    MMIORead64 { addr: u64 },
    MMIOWrite64 { addr: u64, value: u64 },
    
    // USB Bulk Transfers
    USBBulkWrite { 
        endpoint: u8,
        buffer_size: usize,
        timeout_ms: u32,
    },
    USBBulkRead { 
        endpoint: u8,
        buffer_size: usize,
        timeout_ms: u32,
    },
    
    // USB Control Transfers
    USBControlRead {
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        length: u16,
        timeout_ms: u32,
    },
    USBControlWrite {
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        length: u16,
        timeout_ms: u32,
    },
    
    // Control Flow
    Delay { milliseconds: u32 },
    ConditionalBranch { condition: String },
    Jump { label: String },
    Label { name: String },
    
    // Interrupt Handling
    SetupInterrupt { 
        irq_number: u32,
        handler_name: String,
    },
    EnableInterrupt { irq_number: u32 },
    DisableInterrupt { irq_number: u32 },
    
    // Memory Management
    Allocate { size: usize, name: String },
    Deallocate { name: String },
    
    // Error & Metadata
    HandleError { 
        error_code: u32,
        handler: String,
    },
    CapabilityGrant { 
        cap_type: String,
        target: String,
    },
    Comment { text: String },
}
```

#### Methods

```rust
impl Instruction {
    /// Get the category of this instruction
    pub fn category(&self) -> InstructionCategory;
    
    /// Check if this is a USB operation
    pub fn is_usb_operation(&self) -> bool;
    
    /// Check if this is an MMIO operation
    pub fn is_mmio_operation(&self) -> bool;
}

/// Instruction categories
pub enum InstructionCategory {
    Memory,
    Io,
    Control,
    Interrupt,
    Usb,
    Utility,
}
```

### `ConvertedInstruction` Type

Represents an instruction after platform-specific conversion.

```rust
pub struct ConvertedInstruction {
    pub instruction: Instruction,
    pub platform: String,
    pub generated_code: String,
    pub required_includes: Vec<String>,
    pub error_handling: Option<String>,
    pub comment: Option<String>,
}

impl ConvertedInstruction {
    pub fn new(
        instruction: Instruction,
        platform: String,
        generated_code: String,
    ) -> Self;
    
    pub fn with_includes(mut self, includes: Vec<String>) -> Self;
    pub fn with_error_handling(mut self, handling: String) -> Self;
    pub fn with_comment(mut self, comment: String) -> Self;
}
```

### `InstructionStream` Type

A complete sequence of instructions for device operation.

```rust
pub struct InstructionStream {
    pub instructions: Vec<Instruction>,
    pub metadata: StreamMetadata,
}

pub struct StreamMetadata {
    pub id: String,
    pub version: String,
    pub target_paradigm: String,  // e.g., "USB"
    pub created_at: String,        // RFC3339 timestamp
    pub description: String,
}

impl InstructionStream {
    pub fn new(
        instructions: Vec<Instruction>,
        target_paradigm: String,
        description: String,
    ) -> Self;
    
    /// Filter instructions by category
    pub fn filter_by_category(&self, category: InstructionCategory) -> Vec<Instruction>;
}
```

### `DeviceInterface` Type

Complete specification of a target USB device.

```rust
pub struct DeviceInterface {
    pub device_name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub endpoints: Vec<EndpointDescriptor>,
    pub mmio_regions: Vec<MmioRegion>,
    pub interrupts: Vec<InterruptSpec>,
    pub capabilities: DeviceCapabilities,
    pub platform_options: HashMap<String, String>,
    pub description: String,
    pub version: String,
}

impl DeviceInterface {
    pub fn new(device_name: String, vendor_id: u16, product_id: u16) -> Self;
    
    pub fn add_endpoint(mut self, endpoint: EndpointDescriptor) -> Self;
    pub fn add_mmio_region(mut self, region: MmioRegion) -> Self;
    pub fn add_interrupt(mut self, interrupt: InterruptSpec) -> Self;
    pub fn with_capabilities(mut self, caps: DeviceCapabilities) -> Self;
    pub fn with_description(mut self, desc: String) -> Self;
    
    pub fn get_bulk_endpoints(&self) -> Vec<&EndpointDescriptor>;
    pub fn get_interrupt_endpoints(&self) -> Vec<&EndpointDescriptor>;
}
```

### `EndpointDescriptor` Type

USB endpoint specification.

```rust
pub struct EndpointDescriptor {
    pub endpoint_number: u8,
    pub direction: EndpointDirection,
    pub transfer_type: TransferType,
    pub max_packet_size: u16,
    pub interval: u8,
}

pub enum EndpointDirection {
    In,
    Out,
    Bidirectional,
}

pub enum TransferType {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

impl EndpointDirection {
    pub fn to_linux_pipe(&self) -> &str;
    pub fn to_macos_direction(&self) -> u8;
}
```

### `MmioRegion` Type

Memory-mapped I/O region.

```rust
pub struct MmioRegion {
    pub name: String,
    pub base_address: u64,
    pub size: u64,
    pub access_type: AccessType,
}

pub enum AccessType {
    Read,
    Write,
    ReadWrite,
}
```

### `CodeGenerationOutput` Type

Complete output from backend code generation.

```rust
pub struct CodeGenerationOutput {
    pub platform: String,
    pub source_code: String,
    pub header_files: Vec<(String, String)>,      // (filename, content)
    pub configuration_files: Vec<(String, String)>, // (filename, content)
    pub build_artifacts: Vec<String>,
    pub compilation_instructions: String,
}

impl CodeGenerationOutput {
    pub fn new(platform: String, source_code: String) -> Self;
    
    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self;
    pub fn with_config_files(mut self, configs: Vec<(String, String)>) -> Self;
    pub fn with_build_instructions(mut self, instructions: String) -> Self;
}
```

## Backend Trait

### `Backend` Trait

All platform backends implement this trait.

```rust
pub trait Backend {
    /// Get the platform identifier (e.g., "linux_kernel")
    fn platform(&self) -> &str;
    
    /// Generate code from instructions
    fn generate(
        &self,
        instructions: &[Instruction],
        device: &DeviceInterface,
    ) -> Result<CodeGenerationOutput>;
    
    /// Validate all instructions for this platform
    fn validate(&self, instructions: &[Instruction]) -> Result<()>;
    
    /// Convert a single instruction to platform code
    fn convert_instruction(
        &self,
        instruction: &Instruction,
        device: &DeviceInterface,
    ) -> Result<ConvertedInstruction>;
    
    /// Get required includes/dependencies for an instruction
    fn get_includes_for_instruction(&self, instruction: &Instruction) -> Vec<String>;
    
    /// Get error handling code for an instruction
    fn get_error_handling(&self, instruction: &Instruction) -> Option<String>;
    
    /// Generate capability manifest (platform-specific)
    fn generate_capability_manifest(&self, device: &DeviceInterface) -> Result<String>;
    
    /// Generate build configuration (Makefile, CMakeLists.txt, etc.)
    fn generate_build_config(&self, device: &DeviceInterface) -> Result<String>;
}
```

### Backend Implementations

#### `MacOsBackend`

```rust
pub struct MacOsBackend;

impl MacOsBackend {
    pub fn new() -> Self;
}

impl Backend for MacOsBackend { /* ... */ }
impl Default for MacOsBackend { /* ... */ }
```

**Platform Identifier:** `"macos_driverkit"`

**Supported Instructions:**
- All MMIO operations (Read32, Write32, Read64, Write64)
- All USB operations (BulkRead, BulkWrite, ControlRead, ControlWrite)
- Delay, SetupInterrupt, EnableInterrupt, DisableInterrupt
- Comment, HandleError

#### `LinuxBackend`

```rust
pub struct LinuxBackend;

impl LinuxBackend {
    pub fn new() -> Self;
}

impl Backend for LinuxBackend { /* ... */ }
impl Default for LinuxBackend { /* ... */ }
```

**Platform Identifier:** `"linux_kernel"`

**Supported Instructions:**
- All MMIO operations (Read32, Write32, Read64, Write64)
- All USB operations (BulkRead, BulkWrite, ControlRead, ControlWrite)
- Delay, SetupInterrupt, EnableInterrupt, DisableInterrupt
- Comment, HandleError

#### `UsosBackend`

```rust
pub struct UsosBackend;

impl UsosBackend {
    pub fn new() -> Self;
}

impl Backend for UsosBackend { /* ... */ }
impl Default for UsosBackend { /* ... */ }
```

**Platform Identifier:** `"UOSC"`

**Supported Instructions:**
- All MMIO operations (Read32, Write32, Read64, Write64)
- All USB operations (BulkRead, BulkWrite, ControlRead, ControlWrite)
- Delay, SetupInterrupt, EnableInterrupt, DisableInterrupt
- Comment, HandleError

## Error Types

### `UdcError` Enum

All possible errors from UDC operations.

```rust
pub enum UdcError {
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Code generation failed: {0}")]
    CodeGenFailed(String),

    #[error("Device interface error: {0}")]
    DeviceInterfaceError(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, UdcError>;
```

## Helper Functions

### Creating Devices

```rust
// Basic USB device
let device = DeviceInterface::new(
    "MyDevice".to_string(),
    0x1234,  // vendor ID
    0x5678,  // product ID
);

// Device with endpoints
let device = DeviceInterface::new("MyDevice".to_string(), 0x1234, 0x5678)
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
    });

// Device with MMIO regions
let device = device.add_mmio_region(MmioRegion {
    name: "control".to_string(),
    base_address: 0x80000000,
    size: 0x1000,
    access_type: AccessType::ReadWrite,
});
```

### Creating Instructions

```rust
// Simple delay
let delay = Instruction::Delay { milliseconds: 100 };

// USB bulk write
let write = Instruction::USBBulkWrite {
    endpoint: 1,
    buffer_size: 256,
    timeout_ms: 5000,
};

// USB control read
let ctrl_read = Instruction::USBControlRead {
    request_type: 0xC0,
    request: 0x05,
    value: 0x0000,
    index: 0x0000,
    length: 2,
    timeout_ms: 5000,
};

// Comment
let comment = Instruction::Comment {
    text: "Initialize device".to_string(),
};
```

### Generating Code

```rust
// Create a backend
let backend = LinuxBackend::new();

// Generate code
let output = backend.generate(&instructions, &device)?;

// Access generated code
println!("{}", output.source_code);

// Access header files
for (filename, content) in &output.header_files {
    println!("{}: {}", filename, content);
}

// Access config files
for (filename, content) in &output.configuration_files {
    println!("{}: {}", filename, content);
}

// Get build instructions
println!("{}", output.compilation_instructions);
```

## Examples

### Example 1: Simple USB Device

```rust
use bonsai_udc::{Backend, LinuxBackend, DeviceInterface, 
                  EndpointDescriptor, EndpointDirection, TransferType, 
                  Instruction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define device
    let device = DeviceInterface::new(
        "SimpleUSB".to_string(),
        0xDEAD,
        0xBEEF,
    ).add_endpoint(EndpointDescriptor {
        endpoint_number: 1,
        direction: EndpointDirection::Out,
        transfer_type: TransferType::Bulk,
        max_packet_size: 512,
        interval: 0,
    });

    // Create instructions
    let instructions = vec![
        Instruction::USBBulkWrite {
            endpoint: 1,
            buffer_size: 64,
            timeout_ms: 5000,
        },
    ];

    // Generate for Linux
    let backend = LinuxBackend::new();
    let output = backend.generate(&instructions, &device)?;
    
    println!("{}", output.source_code);
    Ok(())
}
```

### Example 2: Complex Device with Multiple Operations

```rust
use bonsai_udc::{Backend, MacOsBackend, DeviceInterface, Instruction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = DeviceInterface::new("ComplexDevice".to_string(), 0x1234, 0x5678);
    
    let instructions = vec![
        Instruction::Comment {
            text: "Initialize hardware".to_string(),
        },
        Instruction::Delay { milliseconds: 100 },
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
            length: 0,
            timeout_ms: 5000,
        },
        Instruction::USBBulkWrite {
            endpoint: 1,
            buffer_size: 256,
            timeout_ms: 5000,
        },
    ];

    // Generate for macOS
    let backend = MacOsBackend::new();
    let output = backend.generate(&instructions, &device)?;
    
    println!("Generated {} bytes of code", output.source_code.len());
    println!("Compilation instructions:\n{}", output.compilation_instructions);
    
    Ok(())
}
```

### Example 3: Cross-Platform Generation

```rust
use bonsai_udc::{Backend, MacOsBackend, LinuxBackend, UsosBackend, 
                  DeviceInterface, Instruction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = DeviceInterface::new("MultiPlatform".to_string(), 0xABCD, 0xEF01);
    let instructions = vec![
        Instruction::USBBulkWrite {
            endpoint: 1,
            buffer_size: 512,
            timeout_ms: 5000,
        },
    ];

    let backends: Vec<Box<dyn Backend>> = vec![
        Box::new(MacOsBackend::new()),
        Box::new(LinuxBackend::new()),
        Box::new(UsosBackend::new()),
    ];

    for backend in backends {
        println!("Generating for {}", backend.platform());
        let output = backend.generate(&instructions, &device)?;
        println!("Generated {} bytes\n", output.source_code.len());
    }

    Ok(())
}
```

## Performance

### Typical Metrics

- **Code Generation**: 50-100ms per instruction stream
- **Validation**: <10ms per instruction stream
- **Output Size**: 2-5KB baseline + ~100 bytes per instruction
- **Memory Usage**: <10MB for typical operations

### Optimization Tips

1. **Batch Processing**: Generate for multiple devices in parallel
2. **Caching**: Cache validation results for identical device specs
3. **Streaming**: Process large instruction streams incrementally

## Thread Safety

- All types are `Send + Sync` where possible
- Backends are stateless and can be shared across threads
- No internal locks required for typical usage

## Compatibility

- **Rust Edition**: 2021
- **Minimum Rust Version**: 1.70.0
- **Platforms**: Linux, macOS, Windows

## See Also

- [UDC Implementation Guide](../../UDC_IMPLEMENTATION_COMPLETE.md)
- [README](./README.md)
- [Source Code](./src/)
- [Examples](./examples/)
- [Tests](./src/tests.rs)
