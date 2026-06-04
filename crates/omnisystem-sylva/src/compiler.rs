//! Sylva Compiler
//!
//! Compiles the Sylva AST to bytecode instructions that the VM can execute.

use crate::ast::Program;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Bytecode {
    // Stack operations
    Push(Value),
    Pop,
    Dup,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    Return,

    // Variables
    Load(String),
    Store(String),

    // Function calls
    Call(String, usize), // name, arity
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Unit,
}

/// Compile an AST into bytecode
pub fn compile_ast(program: &Program) -> Result<Vec<Bytecode>> {
    // TODO: Implement full bytecode compiler
    tracing::debug!("Compiling {} statements to bytecode", program.statements.len());

    Ok(vec![Bytecode::Return])
}
