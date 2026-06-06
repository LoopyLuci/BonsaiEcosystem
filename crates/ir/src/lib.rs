//! Bonsai IR — intermediate representation and effect system.

pub mod codegen;
pub mod effects;
pub mod ops;
pub mod parser;

pub use ops::{
    ArrayOpKind, BinOpKind, DataFrameOpKind, DeviceTarget, EffectHandler, EffectType, IrFunction,
    IrLit, IrModule, IrOp, IrParam, IrPattern, IrProof, IrType, IrTypeDef, IrTypeDefKind, Modality,
    UnOpKind,
};
pub use parser::{parse, parse_expr, ParseError, ParseResult};
