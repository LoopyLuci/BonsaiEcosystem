#![allow(
    clippy::bind_instead_of_map,
    clippy::needless_return,
    clippy::map_identity,
    clippy::suspicious_open_options,
    clippy::items_after_test_module
)]

// Re-export modules for integration testing
pub mod admin_api;
pub mod buddy_client;
pub mod config;
pub mod dedup;
pub mod formatter;
pub mod health;
pub mod metrics;
pub mod mgmt_client;
pub mod platforms;
pub mod router;
pub mod sanitizer;
pub mod scheduler;
pub mod session;
pub mod swarm_client;

pub mod port_manager;
pub mod rule_engine;
