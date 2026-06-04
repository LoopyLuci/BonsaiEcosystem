//! Aether — Distributed Actor Language with Integrated Database
//!
//! Aether is an actor-native language that integrates AriaDB, enabling:
//! - Type-safe schema definitions that are also Aether types
//! - Reactive queries (LiveSet<T>, Live<T>) that push updates to actors
//! - Persistent actor state stored in AriaDB
//! - Capability-based security and row-level policies
//! - Effect-tracked database operations (DbRead, DbWrite)
//! - Safe schema evolution via migrations

pub mod frontend;
pub mod database;
pub mod actor;
pub mod migrations;

pub use frontend::AetherFrontend;
pub use database::{Schema, Query, LiveSet, Live, DbCapability, PersistenceManager};
pub use actor::{Actor, ActorRef, ActorRuntime};
pub use migrations::{Migration, MigrationEngine};

pub fn register_aether() {
    tracing::info!("Aether actor language with integrated database support initialized");
}
