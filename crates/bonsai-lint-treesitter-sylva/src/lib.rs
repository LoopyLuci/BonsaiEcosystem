/// Tree-sitter grammar bindings for Omnisystem Sylva language.
///
/// Sylva is Omnisystem's scripting language with:
/// - Dynamic typing with gradual typing support
/// - First-class functions and closures
/// - Data-driven programming
/// - Integration with Bonsai runtime

pub mod language;
pub use language::LANGUAGE;

use tree_sitter::Language;

pub fn language() -> Language {
    LANGUAGE
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_sylva_parser() {
        let mut parser = Parser::new();
        parser.set_language(LANGUAGE).expect("Error loading Sylva grammar");

        let source = "let x = 42; print(x)";
        let tree = parser.parse(source, None).expect("Error parsing");

        assert!(!tree.root_node().has_error());
    }
}
