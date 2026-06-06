/// Tree-sitter grammar bindings for Omnisystem Aether language.
///
/// Aether is Omnisystem's actor-based concurrency language with:
/// - Actor definition and supervision
/// - Message passing and protocol verification
/// - Location transparency
/// - Distributed object model

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
    fn test_aether_parser() {
        let mut parser = Parser::new();
        parser.set_language(LANGUAGE).expect("Error loading Aether grammar");

        let source = "actor Counter { }";
        let tree = parser.parse(source, None).expect("Error parsing");

        assert!(!tree.root_node().has_error());
    }
}
