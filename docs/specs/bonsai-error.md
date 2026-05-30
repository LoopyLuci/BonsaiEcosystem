# Crate Spec: bonsai-error

## Replaces
`anyhow`, `thiserror`

## Used By
All 30 workspace crates that currently import `anyhow` or `thiserror`.

## API Surface

```rust
use std::fmt;

/// Structured error with a kind discriminant, context chain, and source.
/// Drop-in replacement for anyhow::Error + thiserror::Error combined.
pub struct BonsaiError {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    context: Vec<String>,
}

/// Top-level error categories for the BonsAI workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Io,
    Parse,
    Crypto,
    Network,
    Database,
    Training,
    NotFound,
    PermissionDenied,
    InvalidInput,
    Timeout,
    Internal,
    Other(String),
}

/// Result alias — use throughout all BonsAI crates.
pub type Result<T, E = BonsaiError> = std::result::Result<T, E>;

impl BonsaiError {
    pub fn new(kind: ErrorKind, msg: impl Into<String>) -> Self;
    pub fn other(msg: impl Into<String>) -> Self;
    pub fn kind(&self) -> &ErrorKind;
    pub fn context(self, msg: impl Into<String>) -> Self;
    pub fn with_source(self, src: impl std::error::Error + Send + Sync + 'static) -> Self;
    pub fn is_not_found(&self) -> bool;
    pub fn is_io(&self) -> bool;
}

/// Macro equivalent of anyhow::bail!
#[macro_export]
macro_rules! bail {
    ($kind:expr, $($arg:tt)*) => {
        return Err($crate::BonsaiError::new($kind, format!($($arg)*)))
    };
}

/// Macro equivalent of anyhow::ensure!
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $($arg:tt)*) => {
        if !($cond) { $crate::bail!($kind, $($arg)*) }
    };
}

/// Convert any std::error::Error into a BonsaiError.
impl<E: std::error::Error + Send + Sync + 'static> From<E> for BonsaiError { ... }

impl fmt::Display for BonsaiError { ... }
impl fmt::Debug for BonsaiError { ... }
impl std::error::Error for BonsaiError { ... }
```

## Invariants
- `BonsaiError` is `Send + Sync + 'static` — safe to send across threads
- Context messages are stored in order from outermost to innermost
- `Display` output: `"kind: message\n  context1\n  context2\n  caused by: source"`
- Converting from `std::io::Error` → `BonsaiError` sets `kind = ErrorKind::Io`
- Converting from `std::num::ParseIntError` → sets `kind = ErrorKind::Parse`

## Performance Target
- Error creation: < 1µs (no heap allocations except the message string)
- `Display` formatting: < 10µs

## External Dependencies Allowed
- none (pure std)

## Test Vectors
```rust
// bail! macro
fn might_fail(x: i32) -> Result<i32> {
    ensure!(x > 0, ErrorKind::InvalidInput, "x must be positive, got {}", x);
    Ok(x * 2)
}
assert!(might_fail(-1).is_err());
assert_eq!(might_fail(5).unwrap(), 10);

// context chain
let err = BonsaiError::new(ErrorKind::Io, "file not found")
    .context("reading config")
    .context("starting daemon");
let msg = err.to_string();
assert!(msg.contains("reading config"));
assert!(msg.contains("starting daemon"));

// From<io::Error>
let io_err: BonsaiError = std::io::Error::new(std::io::ErrorKind::NotFound, "test").into();
assert_eq!(io_err.kind(), &ErrorKind::Io);
assert!(io_err.is_io());
```

## Migration Notes
Replace all call sites:
- `use anyhow::{Result, bail, ensure, Context}` → `use bonsai_error::{Result, bail, ensure, BonsaiError, ErrorKind}`
- `anyhow::anyhow!("msg")` → `BonsaiError::other("msg")`
- `.context("msg")?` → `.map_err(|e| e.context("msg"))?`
- `#[derive(thiserror::Error)]` → Remove derive, implement `From<YourError> for BonsaiError`
- `Result<T>` with implicit `anyhow::Error` → `Result<T>` with `bonsai_error::BonsaiError`

## Notes
This crate is the first to build because every other Phase 1 crate depends on it.
Keep it small — no proc macros, no derive, pure struct and trait impls.
The `From<E>` blanket impl may conflict with stdlib — use a specific wrapper instead if needed.
