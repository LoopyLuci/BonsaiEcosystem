#![allow(clippy::doc_overindented_list_items, clippy::unnecessary_cast)]

//! bonsai-array — APL/J-inspired rank-polymorphic array evaluator.
//!
//! Implements a pure-Rust array language kernel covering the core APL/J
//! primitives.  No external array library dependency — rank-polymorphic
//! operations are implemented directly on `NdArray<f64>`.
//!
//! Supported primitives (APL names in parens):
//!   - Scalar arithmetic: + - × ÷ | ⌈ ⌊ * ○ ! ?
//!   - Structural: ⍴ (shape/reshape), ⍋ (grade-up), ⍒ (grade-down),
//!                 ⌽ (reverse), ⍉ (transpose), ↑ (take), ↓ (drop),
//!                 , (ravel), ⍪ (table), ≡ (depth), ≢ (tally)
//!   - Boolean: ∧ ∨ ⍲ ⍱ < ≤ = ≥ > ≠
//!   - Reductions: +/ -/ ×/ ⌈/ ⌊/
//!   - Scans: +\ ×\
//!   - Inner product: +.×
//!   - Outer product: ∘.f

pub mod array;
pub mod error;
pub mod eval;
pub mod ops;

pub use array::NdArray;
pub use error::ArrayError;
pub use eval::{AplEval, EvalResult};
