//! Bonsai Resilience Patterns
//! Production-grade resilience implementations:
//! - Circuit breaker (prevent cascading failures)
//! - Retry logic (exponential backoff)
//! - Timeout enforcement
//! - Bulkhead isolation
//! - Backpressure handling

pub mod circuit_breaker;
pub mod retry;
pub mod timeout;
pub mod bulkhead;
pub mod backpressure;

pub use circuit_breaker::{CircuitBreaker, CircuitState};
pub use retry::{RetryPolicy, RetryStrategy};
pub use timeout::TimeoutPolicy;
pub use bulkhead::Bulkhead;
pub use backpressure::BackpressureController;

use serde::{Deserialize, Serialize};

/// Resilience configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceConfig {
    pub circuit_breaker_enabled: bool,
    pub retry_enabled: bool,
    pub timeout_enabled: bool,
    pub bulkhead_enabled: bool,
    pub backpressure_enabled: bool,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            circuit_breaker_enabled: true,
            retry_enabled: true,
            timeout_enabled: true,
            bulkhead_enabled: true,
            backpressure_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ResilienceConfig::default();
        assert!(config.circuit_breaker_enabled);
        assert!(config.retry_enabled);
    }
}
