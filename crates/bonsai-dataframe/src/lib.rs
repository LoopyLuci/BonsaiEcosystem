#![allow(clippy::redundant_closure)]

//! bonsai-dataframe — Polars-backed DataFrame API for Bonsai/Sylva.
//!
//! Wraps `polars` to provide:
//!   - `BonsaiFrame`: a newtype around `polars::DataFrame` with JSON-serialisable ops
//!   - `BonsaiLazyFrame`: lazy query API
//!   - `DfOp` / `DfResult`: error-bounded operation type
//!   - Serde-round-trip via JSON (for IPC with Tauri / Sylva VM)

pub mod error;
pub mod frame;
pub mod io;
pub mod lazy;
pub mod ops;

pub use error::{DfError, DfResult};
pub use frame::BonsaiFrame;
pub use lazy::BonsaiLazyFrame;
pub use ops::{AggExpr, FilterExpr, SortSpec};
