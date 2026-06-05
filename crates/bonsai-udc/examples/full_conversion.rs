//! Complete driver conversion example
//!
//! This example demonstrates the full pipeline:
//! 1. Load a device specification from JSON
//! 2. Create a conversion context
//! 3. Convert the driver for all platforms
//! 4. Save the output to disk

use bonsai_udc::{DriverConverter, DriverConversionContext};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Universal Driver Compiler - Full Conversion Example ===\n");

    // Load the example Brother FAX driver spec
    let json_path = Path::new("examples/brother_fax_driver.json");

    if !json_path.exists() {
        eprintln!("Error: {} not found", json_path.display());
        eprintln!("Run this example from the bonsai-udc crate directory");
        return Err("File not found".into());
    }

    println!("Loading driver specification from: {}", json_path.display());

    // Create converter with default rules
    let converter = DriverConverter::with_default_engine();

    // Load driver spec for Linux first
    println!("\n--- Converting for Linux Kernel ---");
    let context = DriverConversionContext::from_file(json_path, "linux_kernel")?;
    let output_dir = Path::new("target/linux_driver");

    match converter.convert_and_save(&context, output_dir) {
        Ok(output) => {
            println!("{}", output.summary());
            println!("Files saved to: {}", output_dir.display());
        }
        Err(e) => eprintln!("Conversion failed: {}", e),
    }

    // Convert for macOS DriverKit
    println!("\n--- Converting for macOS DriverKit ---");
    let context = DriverConversionContext::from_file(json_path, "macos_driverkit")?;
    let output_dir = Path::new("target/macos_driver");

    match converter.convert_and_save(&context, output_dir) {
        Ok(output) => {
            println!("{}", output.summary());
            println!("Files saved to: {}", output_dir.display());
        }
        Err(e) => eprintln!("Conversion failed: {}", e),
    }

    // Convert for UOSC
    println!("\n--- Converting for UOSC ---");
    let context = DriverConversionContext::from_file(json_path, "UOSC")?;
    let output_dir = Path::new("target/UOSC_driver");

    match converter.convert_and_save(&context, output_dir) {
        Ok(output) => {
            println!("{}", output.summary());
            println!("Files saved to: {}", output_dir.display());
        }
        Err(e) => eprintln!("Conversion failed: {}", e),
    }

    println!("\n=== Conversion Complete ===");
    println!("Check the target/ directory for generated driver code");

    Ok(())
}
