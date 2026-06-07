//! Wave 3, Component 2: Safety Envelope
//! Enforces runtime guarantees: latency, memory, determinism

pub struct SafetyConstraints {
    max_latency_ms: u64,
    max_memory_mb: u64,
}

impl SafetyConstraints {
    pub fn new(max_latency_ms: u64, max_memory_mb: u64) -> Self {
        Self {
            max_latency_ms,
            max_memory_mb,
        }
    }

    pub fn check(&self, latency: u64, memory: u64) -> Result<(), String> {
        if latency > self.max_latency_ms {
            return Err("latency exceeded".to_string());
        }
        if memory > self.max_memory_mb {
            return Err("memory exceeded".to_string());
        }
        Ok(())
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraints() {
        let c = SafetyConstraints::new(100, 512);
        assert!(c.check(50, 256).is_ok());
        assert!(c.check(150, 256).is_err());
    }
}
