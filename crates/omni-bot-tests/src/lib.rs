//! Omni-Bot Integration Test Suite
//!
//! Comprehensive testing framework with 1000+ tests covering:
//! - API handlers and routes
//! - Service lifecycle management
//! - Environment operations
//! - Module management
//! - Asset generation and publishing
//! - Validation and testing workflows
//! - Capability-based security
//! - Offline operations and sync

pub mod helpers;
pub mod fixtures;

pub use helpers::{MockServer, TestClient, TestContext};
pub use fixtures::{TestDataBuilder, ServiceFixture};
