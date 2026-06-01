#![doc = include_str!("../README.md")]

pub mod connection;
pub mod discovery;
pub mod streaming;
pub mod input;
pub mod file_sync;
pub mod device;
pub mod capability;
pub mod error;
pub mod telemetry;
pub mod security;

pub use connection::AndroidBridge;
pub use device::Device;
pub use error::{Error, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
