pub mod detect;
pub mod allocator;
pub mod types;
pub mod cpu;

pub use detect::detect_hardware;
pub use allocator::Allocator;
pub use types::*;

use std::sync::OnceLock;
use log::{info, warn};

/// Global hardware profile, detected once at startup.
static HARDWARE_PROFILE: OnceLock<HardwareProfile> = OnceLock::new();

/// Initialize the BUEB backend. Must be called once at application startup.
pub fn initialize() -> anyhow::Result<()> {
    info!("🔧 BUEB: Detecting hardware...");
    let profile = detect_hardware()?;

    info!("✅ BUEB: Detected system:");
    info!("   CPU: {} ({} logical cores @ {}MHz)",
        profile.cpu.model, profile.cpu.logical_cores, profile.cpu.frequency_mhz);
    info!("   RAM: {} GB", profile.memory.total_bytes / 1_000_000_000);

    if profile.gpus.is_empty() {
        warn!("⚠️  BUEB: No GPUs detected. All workloads will use CPU.");
        warn!("   Consider using quantized models (Q4_K_M) for better performance.");
    } else {
        for (i, gpu) in profile.gpus.iter().enumerate() {
            info!("   GPU {}: {} ({} VRAM, {})",
                i, gpu.name, gpu.vram_bytes / 1_000_000_000, gpu.backend);
        }
    }

    HARDWARE_PROFILE.set(profile).map_err(|_| anyhow::anyhow!("BUEB already initialized"))?;
    Ok(())
}

/// Get the current hardware profile. Panics if BUEB not initialized.
pub fn profile() -> &'static HardwareProfile {
    HARDWARE_PROFILE.get()
        .expect("BUEB not initialized. Call bonsai_backend::initialize() first.")
}

/// Allocate the best device(s) for a given task.
pub fn allocate(task: &TaskRequirements) -> DeviceAllocation {
    let profile = profile();
    allocator::allocate(profile, task)
}

/// Check if GPU acceleration is available.
pub fn has_gpu() -> bool {
    profile().gpus.len() > 0
}

/// Get the number of available GPUs.
pub fn gpu_count() -> usize {
    profile().gpus.len()
}

/// Get available CPU cores.
pub fn cpu_cores() -> u32 {
    profile().cpu.logical_cores
}

/// Get total available RAM in bytes.
pub fn total_memory() -> u64 {
    profile().memory.total_bytes
}
