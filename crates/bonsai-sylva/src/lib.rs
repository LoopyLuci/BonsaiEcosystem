#![allow(
    clippy::unnecessary_map_or,
    clippy::derivable_impls,
    clippy::self_assignment,
    clippy::result_large_err,
    clippy::get_first
)]

//! bonsai-sylva — Native Sylva language interpreter.
//!
//! Sylva is Bonsai's interactive scripting language (equivalent to Omnisystem's Sylva layer).
//! This crate implements:
//!   - Lexer (lexer.rs)
//!   - AST (ast.rs)
//!   - Parser (parser.rs)
//!   - Tree-walk interpreter / VM (vm.rs)
//!   - Standard library bindings (stdlib.rs)
//!   - Time-travel debugger (debugger.rs) — snapshot + rewind/replay
//!
//! The VM is designed to be embedded in `SylvaRuntime` (Tauri backend) to replace
//! the Lua VM. It exposes the same `bonsai.tool(name, args)` interface.

pub mod ast;
pub mod debugger;
pub mod lexer;
pub mod parser;
pub mod stdlib;
pub mod vm;

pub use debugger::{Debugger, RewindError, Snapshot};
pub use parser::ParseError;
pub use vm::{SylvaValue, SylvaVm, VmError, VmResult};
