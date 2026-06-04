//! Aether — Distributed Actor Language
//! Phase 5: Actor model with supervision, location transparency, message passing

pub mod frontend;
pub use frontend::AetherFrontend;

pub fn register_aether() {
    tracing::info!("Aether actor language support initialized");
}
