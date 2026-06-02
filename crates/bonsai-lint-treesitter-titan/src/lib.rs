/// Tree-sitter grammar bindings for Omnisystem Titan language.
///
/// Titan is Omnisystem's effect-tracking systems language with features like:
/// - Effect type system (Pure, IO, State, etc.)
/// - Resource management
/// - Linear types
/// - Effect polymorphism

pub mod language;

pub use language::LANGUAGE;

use tree_sitter::Language;

#[cfg(not(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "windows", target_arch = "x86_64"),
)))]
compile_error!("Unsupported target platform for bonsai-lint-treesitter-titan");

/// Get the Titan language grammar.
pub fn language() -> Language {
    LANGUAGE
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_titan_parser() {
        let mut parser = Parser::new();
        parser.set_language(LANGUAGE).expect("Error loading Titan grammar");

        let source = "pub fn pure_function() -> i32 { 42 }";
        let tree = parser.parse(source, None).expect("Error parsing");

        assert!(!tree.root_node().has_error());
    }
}
