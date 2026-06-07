//! SRWSTS Fault Injection
//!
//! Provides fault injection capabilities for stress testing, including:
//! - CPU stress simulation
//! - Memory pressure simulation
//! - Disk I/O stress
//! - Network packet loss
//! - Process termination
//! - Kernel signal injection
//!
//! Supports both deterministic (scheduled) and random fault injection.

use srwsts_core::{FaultDefinition, FaultType, SrwstsError, SrwstsResult};
use std::collections::HashMap;

/// Fault injector implementation
pub struct SimpleFaultInjector {
    active_faults: HashMap<String, FaultDefinition>,
}

impl SimpleFaultInjector {
    /// Create a new fault injector
    pub fn new() -> Self {
        Self {
            active_faults: HashMap::new(),
        }
    }

    /// Inject a CPU stress fault
    pub fn inject_cpu_stress(
        &mut self,
        fault_id: &str,
        num_cores: u32,
    ) -> SrwstsResult<()> {
        if num_cores == 0 || num_cores > 256 {
            return Err(SrwstsError::InvalidFaultParameters {
                reason: format!("invalid number of cores: {}", num_cores),
            });
        }

        let fault = FaultDefinition::new(
            fault_id,
            FaultType::CpuStress,
            0,
            60,
        ).with_parameter("cpu_count", serde_json::json!(num_cores));

        self.active_faults.insert(fault_id.to_string(), fault);
        Ok(())
    }

    /// Inject a memory exhaustion fault
    pub fn inject_memory_exhaustion(
        &mut self,
        fault_id: &str,
        memory_percent: f64,
    ) -> SrwstsResult<()> {
        if memory_percent < 0.0 || memory_percent > 100.0 {
            return Err(SrwstsError::InvalidFaultParameters {
                reason: format!("invalid memory percent: {}", memory_percent),
            });
        }

        let fault = FaultDefinition::new(
            fault_id,
            FaultType::MemoryExhaustion,
            0,
            60,
        ).with_parameter("memory_percent", serde_json::json!(memory_percent));

        self.active_faults.insert(fault_id.to_string(), fault);
        Ok(())
    }

    /// Recover from a fault
    pub fn recover_fault(&mut self, fault_id: &str) -> SrwstsResult<()> {
        if !self.active_faults.contains_key(fault_id) {
            return Err(SrwstsError::FaultRecoveryFailed {
                reason: format!("fault {} not found", fault_id),
            });
        }

        self.active_faults.remove(fault_id);
        Ok(())
    }

    /// Check if a fault is still active
    pub fn is_active(&self, fault_id: &str) -> bool {
        self.active_faults.contains_key(fault_id)
    }

    /// Get all active faults
    pub fn active_faults(&self) -> Vec<&FaultDefinition> {
        self.active_faults.values().collect()
    }

    /// Clear all active faults
    pub fn clear_all(&mut self) {
        self.active_faults.clear();
    }
}

impl Default for SimpleFaultInjector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_injector() {
        let injector = SimpleFaultInjector::new();
        assert_eq!(injector.active_faults().len(), 0);
    }

    #[test]
    fn test_inject_cpu_stress() {
        let mut injector = SimpleFaultInjector::new();
        let result = injector.inject_cpu_stress("f1", 4);
        assert!(result.is_ok());
        assert!(injector.is_active("f1"));
    }

    #[test]
    fn test_inject_invalid_cpu_count() {
        let mut injector = SimpleFaultInjector::new();
        let result = injector.inject_cpu_stress("f1", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_inject_memory_exhaustion() {
        let mut injector = SimpleFaultInjector::new();
        let result = injector.inject_memory_exhaustion("f2", 80.0);
        assert!(result.is_ok());
        assert!(injector.is_active("f2"));
    }

    #[test]
    fn test_recover_fault() {
        let mut injector = SimpleFaultInjector::new();
        let _ = injector.inject_cpu_stress("f1", 4);
        assert!(injector.is_active("f1"));

        let result = injector.recover_fault("f1");
        assert!(result.is_ok());
        assert!(!injector.is_active("f1"));
    }

    #[test]
    fn test_clear_all_faults() {
        let mut injector = SimpleFaultInjector::new();
        let _ = injector.inject_cpu_stress("f1", 4);
        let _ = injector.inject_memory_exhaustion("f2", 50.0);
        assert_eq!(injector.active_faults().len(), 2);

        injector.clear_all();
        assert_eq!(injector.active_faults().len(), 0);
    }
}
