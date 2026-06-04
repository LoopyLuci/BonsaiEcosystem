//! Sylva Parser
//!
//! Parses Sylva source code into an Abstract Syntax Tree (AST)
//! using a Pest-based PEG grammar.

use crate::ast::*;
use anyhow::Result;

/// Parse Sylva source code into a Program AST
pub fn parse_sylva(source: &str) -> Result<Program> {
    // TODO: Implement full Sylva parser with Pest
    // For now, return an empty program
    tracing::debug!("Parsing Sylva code ({} bytes)", source.len());

    Ok(Program {
        statements: Vec::new(),
    })
}
