//! Sylva — Interactive Scripting Language
//!
//! Sylva is a gradually-typed, interactive scripting language designed for
//! exploratory programming, data science, and rapid prototyping within the Bonsai Ecosystem.
//!
//! **Key features:**
//! - Gradual typing (optional type annotations)
//! - Interactive REPL with time-travel debugging
//! - Zero-overhead calls to Titan functions
//! - First-class actors (Aether integration)
//! - Bytecode interpreter with JIT compilation

pub mod frontend;
pub mod parser;
pub mod ast;
pub mod compiler;
pub mod vm;

pub use frontend::SylvaFrontend;

/// Register this language frontend with the registry
/// Call this during application initialization
pub fn register_sylva() {
    // TODO: Hook into a registration system when needed
    // For now, this is a placeholder for runtime registration
}
