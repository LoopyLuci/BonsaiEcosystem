/// Tree-sitter grammar bindings for Omnisystem Axiom language.
///
/// Axiom is Omnisystem's formal verification language with:
/// - Dependent types and refinement types
/// - Proof tactics and proof state management
/// - Integration with SMT solvers
/// - Verifiable code annotations

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
    fn test_axiom_parser() {
        let mut parser = Parser::new();
        parser.set_language(LANGUAGE).expect("Error loading Axiom grammar");

        let source = "theorem add_comm : ∀ x y, x + y = y + x { }";
        let tree = parser.parse(source, None).expect("Error parsing");

        assert!(!tree.root_node().has_error());
    }
}
