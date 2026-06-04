//! LAIR — Language-Agnostic Intermediate Representation
//!
//! LAIR is the common compilation target for all languages in the Bonsai Ecosystem.
//! Every language frontend (Titan, Sylva, Python, Go, SQL, etc.) compiles to LAIR,
//! which is then optimized by BACE and lowered to machine code or bytecode by the backend.
//!
//! **Key properties of LAIR:**
//! - **Typed SSA form:** Every value has a type, and variables are assigned exactly once
//! - **Effect annotations:** IO, allocation, panic, and other effects are explicit
//! - **Hot-reload metadata:** Functions can be recompiled and relinked without stopping execution
//! - **Language-agnostic:** A single LAIR representation unifies all languages

use serde::{Deserialize, Serialize};

/// A LAIR module — the output of a language frontend
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LairModule {
    /// Name of the module
    pub name: String,

    /// Functions defined in this module
    pub functions: Vec<LairFunction>,

    /// Type definitions
    pub types: Vec<LairTypeDefinition>,

    /// Global constants
    pub constants: Vec<LairConstant>,

    /// Module metadata (imports, exports, etc.)
    pub metadata: ModuleMetadata,
}

/// A function in LAIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LairFunction {
    pub name: String,
    pub params: Vec<(String, LairValueType)>,
    pub return_type: LairValueType,
    pub body: LairValue,
    pub effects: Vec<Effect>,
    pub hot_reload_enabled: bool,
}

/// A value/expression in LAIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LairValue {
    Literal(LairLiteral),
    Var(String),
    BinOp {
        left: Box<LairValue>,
        op: String,
        right: Box<LairValue>,
    },
    Call {
        func: Box<LairValue>,
        args: Vec<LairValue>,
    },
    If {
        cond: Box<LairValue>,
        then_val: Box<LairValue>,
        else_val: Option<Box<LairValue>>,
    },
    Block(Vec<LairValue>),
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LairLiteral {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Unit,
}

/// Value type in LAIR
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LairValueType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Str,
    Unit,
    Custom(String),
    Array(Box<LairValueType>),
    Tuple(Vec<LairValueType>),
    Function(Vec<LairValueType>, Box<LairValueType>),
}

/// Effects that a function may perform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Effect {
    Io,
    Alloc,
    Network,
    Panic,
    Unsafe,
    Telemetry,
    Device(String),
}

/// A type definition in the module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LairTypeDefinition {
    pub name: String,
    pub kind: LairTypeKind,
}

/// Kind of type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LairTypeKind {
    Struct(Vec<(String, LairValueType)>),
    Enum(Vec<(String, Option<LairValueType>)>),
    Alias(LairValueType),
}

/// A global constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LairConstant {
    pub name: String,
    pub typ: LairValueType,
    pub value: LairValue,
}

/// Module metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleMetadata {
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub source_language: Option<String>,
}
