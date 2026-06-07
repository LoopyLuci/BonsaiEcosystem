//! Omni-Bot REST/WebSocket API Layer
//! Comprehensive REST API for service management, configuration, and monitoring
//!
//! Phase 1 API Coverage - Service Management (8 core endpoints):
//!
//! Services - 8 endpoints:
//! - GET /services - List all services with status summary
//! - POST /services/{name}/start - Start a service with optional config
//! - POST /services/{name}/stop - Stop a service gracefully or forcefully
//! - GET /services/{name}/status - Get detailed service status and metrics
//! - POST /services/{name}/restart - Restart service with pid tracking
//! - POST /services/{name}/configure - Apply or merge service configuration
//! - POST /services/{name}/snapshot - Create service state snapshot
//! - GET /services/{name}/logs - Retrieve service logs with filtering
//!
//! Plus health check, metrics endpoints, and capability-based authentication.

pub mod error;
pub mod models;
pub mod middleware;
pub mod routes;
pub mod handlers;

pub use error::{ApiError, ApiResult};
pub use models::*;
pub use routes::create_router;
pub use handlers::{
    validation, driver, hde,
    ValidationState, DriverState, HdeState,
};

/// Initialize API system
pub fn init() {
    log::info!("Omni-Bot API v{} initialized", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
        assert!(true);
    }
}
