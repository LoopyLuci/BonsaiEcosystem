//! Titan — Systems Programming Language
//!
//! Titan is the systems programming language of the Omnisystem, occupying the lowest level
//! of the four-language stack. It handles code where performance, memory layout, and safety
//! guarantees must all be simultaneously provable.
//!
//! **Key features:**
//! - No hidden allocations (explicit effect declarations)
//! - No undefined behavior (compile-time safety guarantees)
//! - No garbage collector (ownership & borrowing)
//! - All effects declared (IO, alloc, panic, telemetry)
//! - Compile to LLVM IR and native machine code
//! - Self-hosted compiler (bootstrapped from seed)

pub mod frontend;
pub mod parser;
pub mod ast;
pub mod typeck;
pub mod lower;

pub use frontend::TitanFrontend;

/// Register Titan in the language system
pub fn register_titan() {
    tracing::info!("Titan language support initialized");
}
