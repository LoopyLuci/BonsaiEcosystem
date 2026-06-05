//! Device Instruction Stream (DIS) - Core instruction set for all platforms
//!
//! DIS defines 47 language paradigms and instruction types. This module focuses
//! on the USB/device control subset needed for driver code generation.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a single instruction in the Device Instruction Stream
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    /// Read 32-bit value from MMIO address
    MMIORead32 { addr: u64 },
    /// Write 32-bit value to MMIO address
    MMIOWrite32 { addr: u64, value: u32 },
    /// Read 64-bit value from MMIO address
    MMIORead64 { addr: u64 },
    /// Write 64-bit value to MMIO address
    MMIOWrite64 { addr: u64, value: u64 },

    /// USB Bulk Write
    USBBulkWrite {
        endpoint: u8,
        buffer_size: usize,
        timeout_ms: u32,
    },
    /// USB Bulk Read
    USBBulkRead {
        endpoint: u8,
        buffer_size: usize,
        timeout_ms: u32,
    },
    /// USB Control Transfer (device-to-host)
    USBControlRead {
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        length: u16,
        timeout_ms: u32,
    },
    /// USB Control Transfer (host-to-device)
    USBControlWrite {
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        length: u16,
        timeout_ms: u32,
    },

    /// Delay operation
    Delay { milliseconds: u32 },

    /// Conditional branch - execute following instructions if condition met
    ConditionalBranch { condition: String },
    /// Unconditional jump to label
    Jump { label: String },
    /// Label for jump targets
    Label { name: String },

    /// Interrupt request setup
    SetupInterrupt {
        irq_number: u32,
        handler_name: String,
    },
    /// Enable interrupt
    EnableInterrupt { irq_number: u32 },
    /// Disable interrupt
    DisableInterrupt { irq_number: u32 },

    /// Memory allocation
    Allocate { size: usize, name: String },
    /// Memory deallocation
    Deallocate { name: String },

    /// Error handling
    HandleError { error_code: u32, handler: String },

    /// Capability delegation (for UOSC)
    CapabilityGrant { cap_type: String, target: String },

    /// Metadata/documentation
    Comment { text: String },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::MMIORead32 { addr } => write!(f, "MMIORead32(0x{:x})", addr),
            Instruction::MMIOWrite32 { addr, value } => {
                write!(f, "MMIOWrite32(0x{:x}, 0x{:x})", addr, value)
            }
            Instruction::USBBulkWrite { endpoint, .. } => write!(f, "USBBulkWrite(ep={})", endpoint),
            Instruction::USBBulkRead { endpoint, .. } => write!(f, "USBBulkRead(ep={})", endpoint),
            Instruction::Delay { milliseconds } => write!(f, "Delay({}ms)", milliseconds),
            Instruction::Comment { text } => write!(f, "// {}", text),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Converted instruction after platform-specific transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertedInstruction {
    /// Original instruction
    pub instruction: Instruction,
    /// Platform-specific code (platform name)
    pub platform: String,
    /// Generated code snippet
    pub generated_code: String,
    /// Required imports/includes
    pub required_includes: Vec<String>,
    /// Required error handling
    pub error_handling: Option<String>,
    /// Comments
    pub comment: Option<String>,
}

impl ConvertedInstruction {
    pub fn new(
        instruction: Instruction,
        platform: String,
        generated_code: String,
    ) -> Self {
        Self {
            instruction,
            platform,
            generated_code,
            required_includes: Vec::new(),
            error_handling: None,
            comment: None,
        }
    }

    pub fn with_includes(mut self, includes: Vec<String>) -> Self {
        self.required_includes = includes;
        self
    }

    pub fn with_error_handling(mut self, handling: String) -> Self {
        self.error_handling = Some(handling);
        self
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

/// Complete instruction stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionStream {
    pub instructions: Vec<Instruction>,
    pub metadata: StreamMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub id: String,
    pub version: String,
    pub target_paradigm: String, // "USB" for initial launch
    pub created_at: String,
    pub description: String,
}

impl InstructionStream {
    pub fn new(
        instructions: Vec<Instruction>,
        target_paradigm: String,
        description: String,
    ) -> Self {
        Self {
            instructions,
            metadata: StreamMetadata {
                id: uuid::Uuid::new_v4().to_string(),
                version: "1.0.0".to_string(),
                target_paradigm,
                created_at: chrono::Utc::now().to_rfc3339(),
                description,
            },
        }
    }

    pub fn filter_by_category(
        &self,
        category: InstructionCategory,
    ) -> Vec<Instruction> {
        self.instructions
            .iter()
            .filter(|instr| instr.category() == category)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCategory {
    Memory,
    Io,
    Control,
    Interrupt,
    Usb,
    Utility,
}

impl Instruction {
    pub fn category(&self) -> InstructionCategory {
        match self {
            Instruction::MMIORead32 { .. } | Instruction::MMIOWrite32 { .. } |
            Instruction::MMIORead64 { .. } | Instruction::MMIOWrite64 { .. } => {
                InstructionCategory::Io
            }
            Instruction::USBBulkWrite { .. }
            | Instruction::USBBulkRead { .. }
            | Instruction::USBControlRead { .. }
            | Instruction::USBControlWrite { .. } => InstructionCategory::Usb,
            Instruction::Delay { .. } => InstructionCategory::Utility,
            Instruction::ConditionalBranch { .. }
            | Instruction::Jump { .. }
            | Instruction::Label { .. } => InstructionCategory::Control,
            Instruction::SetupInterrupt { .. }
            | Instruction::EnableInterrupt { .. }
            | Instruction::DisableInterrupt { .. } => InstructionCategory::Interrupt,
            Instruction::Allocate { .. } | Instruction::Deallocate { .. } => {
                InstructionCategory::Memory
            }
            Instruction::HandleError { .. }
            | Instruction::CapabilityGrant { .. }
            | Instruction::Comment { .. } => InstructionCategory::Utility,
        }
    }

    pub fn is_usb_operation(&self) -> bool {
        matches!(
            self,
            Instruction::USBBulkWrite { .. }
                | Instruction::USBBulkRead { .. }
                | Instruction::USBControlRead { .. }
                | Instruction::USBControlWrite { .. }
        )
    }

    pub fn is_mmio_operation(&self) -> bool {
        matches!(
            self,
            Instruction::MMIORead32 { .. }
                | Instruction::MMIOWrite32 { .. }
                | Instruction::MMIORead64 { .. }
                | Instruction::MMIOWrite64 { .. }
        )
    }
}
