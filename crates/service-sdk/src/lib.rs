//! Phase 4: Service SDK
//! Provides the Snapshotable trait and utilities for service developers

use std::error::Error;

/// Core trait all background services must implement
pub trait Snapshotable: Send + Sync {
    /// Called before kernel snapshot
    /// Service must:
    /// 1. Flush buffered writes
    /// 2. Close transient connections
    /// 3. Return opaque state for on_resume
    fn on_pause(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Called after kernel restore
    /// Service must reconstruct state from blob
    fn on_resume(&mut self, state: &[u8]) -> Result<(), Box<dyn Error>>;

    /// Health check (optional)
    fn health_check(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

/// Service configuration helper
#[derive(Clone, Debug)]
pub struct ServiceConfig {
    pub name: String,
    pub version: String,
    pub memory_mb: u32,
    pub cpu_cores: f32,
}

impl ServiceConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            memory_mb: 256,
            cpu_cores: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestService {
        state: String,
    }

    impl Snapshotable for TestService {
        fn on_pause(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
            Ok(self.state.as_bytes().to_vec())
        }

        fn on_resume(&mut self, state: &[u8]) -> Result<(), Box<dyn Error>> {
            self.state = String::from_utf8(state.to_vec())?;
            Ok(())
        }
    }

    #[test]
    fn test_snapshotable() {
        let mut service = TestService {
            state: "test_state".to_string(),
        };
        let state = service.on_pause().unwrap();
        service.state = String::new();
        service.on_resume(&state).unwrap();
        assert_eq!(service.state, "test_state");
    }
}
