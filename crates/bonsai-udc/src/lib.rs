//! Universal Driver Compiler (UDC) - Generate valid, compilable driver code
//! for macOS DriverKit, Linux kernel modules, and UOSC native drivers.
//!
//! The UDC system takes Device Instruction Stream (DIS) instructions and converts
//! them into production-ready driver code for three distinct platforms.
//!
//! # Quick Start
//!
//! ```ignore
//! use bonsai_udc::{DriverConverter, DriverConversionContext, DeviceInterface};
//!
//! let converter = DriverConverter::with_default_engine();
//! let context = DriverConversionContext::from_file("driver.json", "linux_kernel")?;
//! let output = converter.convert_driver(&context)?;
//! ```

pub mod dis;
pub mod device_interface;
pub mod error;
pub mod backend;
pub mod rules;
pub mod registry;
pub mod engine;
pub mod cli;
pub mod integrator;

pub use dis::{Instruction, ConvertedInstruction, InstructionStream};
pub use device_interface::DeviceInterface;
pub use error::{UdcError, Result};
pub use backend::{Backend, MacOsBackend, LinuxBackend, UsosBackend};
pub use rules::RuleDatabase;
pub use registry::DriverRegistry;
pub use engine::{ConversionEngine, ConversionResult};
pub use cli::{Cli, CliArgs};
pub use integrator::{DriverConverter, DriverConversionContext, DriverConversionOutput};

#[cfg(test)]
mod tests;
