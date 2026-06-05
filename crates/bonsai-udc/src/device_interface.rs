//! Device Interface - Metadata and capabilities for target devices

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete device interface specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInterface {
    pub device_name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,

    /// USB endpoints
    pub endpoints: Vec<EndpointDescriptor>,

    /// MMIO memory regions
    pub mmio_regions: Vec<MmioRegion>,

    /// Interrupts used
    pub interrupts: Vec<InterruptSpec>,

    /// Device capabilities
    pub capabilities: DeviceCapabilities,

    /// Platform-specific options
    pub platform_options: HashMap<String, String>,

    /// Metadata
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointDescriptor {
    pub endpoint_number: u8,
    pub direction: EndpointDirection,
    pub transfer_type: TransferType,
    pub max_packet_size: u16,
    pub interval: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndpointDirection {
    In,
    Out,
    Bidirectional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferType {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

impl EndpointDirection {
    pub fn to_linux_pipe(&self) -> &str {
        match self {
            EndpointDirection::In => "usb_rcvbulkpipe",
            EndpointDirection::Out => "usb_sndbulkpipe",
            EndpointDirection::Bidirectional => "usb_sndbulkpipe", // default
        }
    }

    pub fn to_macos_direction(&self) -> u8 {
        match self {
            EndpointDirection::In => 0x80, // kUSBIn
            EndpointDirection::Out => 0x00, // kUSBOut
            EndpointDirection::Bidirectional => 0x00,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MmioRegion {
    pub name: String,
    pub base_address: u64,
    pub size: u64,
    pub access_type: AccessType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptSpec {
    pub irq_number: u32,
    pub interrupt_type: InterruptType,
    pub handler_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptType {
    Edge,
    Level,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub supports_dma: bool,
    pub supports_interrupts: bool,
    pub supports_power_management: bool,
    pub max_concurrent_transfers: u32,
    pub alignment_requirement: u32,
}

impl DeviceInterface {
    pub fn new(
        device_name: String,
        vendor_id: u16,
        product_id: u16,
    ) -> Self {
        Self {
            device_name,
            vendor_id,
            product_id,
            device_class: 0,
            device_subclass: 0,
            device_protocol: 0,
            endpoints: Vec::new(),
            mmio_regions: Vec::new(),
            interrupts: Vec::new(),
            capabilities: DeviceCapabilities {
                supports_dma: false,
                supports_interrupts: false,
                supports_power_management: false,
                max_concurrent_transfers: 1,
                alignment_requirement: 4,
            },
            platform_options: HashMap::new(),
            description: String::new(),
            version: "1.0.0".to_string(),
        }
    }

    pub fn add_endpoint(mut self, endpoint: EndpointDescriptor) -> Self {
        self.endpoints.push(endpoint);
        self
    }

    pub fn add_mmio_region(mut self, region: MmioRegion) -> Self {
        self.mmio_regions.push(region);
        self
    }

    pub fn add_interrupt(mut self, interrupt: InterruptSpec) -> Self {
        self.interrupts.push(interrupt);
        self
    }

    pub fn with_capabilities(mut self, capabilities: DeviceCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn get_bulk_endpoints(&self) -> Vec<&EndpointDescriptor> {
        self.endpoints
            .iter()
            .filter(|ep| ep.transfer_type == TransferType::Bulk)
            .collect()
    }

    pub fn get_interrupt_endpoints(&self) -> Vec<&EndpointDescriptor> {
        self.endpoints
            .iter()
            .filter(|ep| ep.transfer_type == TransferType::Interrupt)
            .collect()
    }
}
