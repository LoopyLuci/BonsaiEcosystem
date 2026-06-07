//! Bonsai Buddy Backend - Tauri IPC & API Integration
//!
//! Production-ready backend for Bonsai Buddy Tauri app with:
//! - Tauri IPC bridge for command handling
//! - REST/WebSocket client for Omni-Bot
//! - Offline-first state management with LRU caching
//! - CRDT-based sync engine for conflict resolution
//! - Offline action queueing with persistent storage

pub mod api_client;
pub mod cache;
pub mod error;
pub mod handlers;
pub mod offline_queue;
pub mod state;
pub mod sync_engine;

pub use api_client::ApiClient;
pub use cache::CacheManager;
pub use error::{Error, Result};
pub use handlers::CommandHandlers;
pub use offline_queue::OfflineQueue;
pub use state::{AppState, SessionInfo};
pub use sync_engine::SyncEngine;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the backend system
pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    log::info!("Bonsai Buddy Backend v{} initialized", VERSION);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
