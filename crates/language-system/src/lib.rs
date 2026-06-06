//! Bonsai Language Frontend System — Unified polyglot language integration
//!
//! This crate provides the core trait and registry system for integrating
//! programming languages into the Bonsai Ecosystem. Every language—whether
//! an Omnisystem language (Titan, Sylva, Aether, Axiom) or an existing
//! language (Python, Go, SQL)—implements the `LanguageFrontend` trait.

pub mod frontend;
pub mod registry;
pub mod lsp;
pub mod errors;

pub use frontend::{LanguageFrontend, LspHandler, LairInterpreter, Diagnostic, DiagnosticLevel, CompletionItem, CompletionKind};
pub use registry::{LanguageRegistry, LanguageRegistration};
pub use errors::{FrontendError, Result};

// Re-export for convenience
pub use core_ir::LairModule;
use std::path::Path;

/// Global language registry
pub static LANGUAGE_REGISTRY: once_cell::sync::Lazy<LanguageRegistry> =
    once_cell::sync::Lazy::new(LanguageRegistry::new);

/// Get all registered languages
pub fn list_languages() -> Vec<String> {
    LANGUAGE_REGISTRY.list_all()
}

/// Get a language frontend by name
pub fn get_frontend(name: &str) -> Option<Box<dyn LanguageFrontend>> {
    LANGUAGE_REGISTRY.get(name)
}

/// Detect language from file extension
pub fn detect_language(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    LANGUAGE_REGISTRY.get_by_extension(ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_registry() {
        let registry = LanguageRegistry::new();
        assert_eq!(registry.list_all().len(), 0);
    }
}
