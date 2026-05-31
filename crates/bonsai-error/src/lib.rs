#![allow(clippy::bool_comparison)]

use std::fmt;

// ── Error kinds ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Internal,
    InvalidInput,
    NotFound,
    Timeout,
    PermissionDenied,
    Network,
    Io,
    Parse,
    Crypto,
    State,
    ResourceExhausted,
    NotSupported,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Internal => "internal error",
            Self::InvalidInput => "invalid input",
            Self::NotFound => "not found",
            Self::Timeout => "timeout",
            Self::PermissionDenied => "permission denied",
            Self::Network => "network error",
            Self::Io => "I/O error",
            Self::Parse => "parse error",
            Self::Crypto => "cryptographic error",
            Self::State => "invalid state",
            Self::ResourceExhausted => "resource exhausted",
            Self::NotSupported => "not supported",
        };
        write!(f, "{s}")
    }
}

// ── BonsaiError ───────────────────────────────────────────────────────────────

pub struct BonsaiError {
    pub kind: ErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    context: Vec<String>,
    recovery_hint: Option<String>,
}

impl BonsaiError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
            context: Vec::new(),
            recovery_hint: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound, message)
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::InvalidInput, message)
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Io, message)
    }

    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context.push(ctx.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.recovery_hint = Some(hint.into());
        self
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn recovery_hint(&self) -> Option<&str> {
        self.recovery_hint.as_deref()
    }

    pub fn is_transient(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::Timeout | ErrorKind::Network | ErrorKind::ResourceExhausted
        )
    }
}

impl fmt::Debug for BonsaiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BonsaiError({:?}: {})", self.kind, self.message)?;
        for ctx in &self.context {
            write!(f, "\n  context: {ctx}")?;
        }
        if let Some(h) = &self.recovery_hint {
            write!(f, "\n  hint: {h}")?;
        }
        Ok(())
    }
}

impl fmt::Display for BonsaiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)?;
        if !self.context.is_empty() {
            write!(f, " ({})", self.context.join(", "))?;
        }
        Ok(())
    }
}

impl std::error::Error for BonsaiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// ── From conversions ──────────────────────────────────────────────────────────

impl From<std::io::Error> for BonsaiError {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::Io, e.to_string()).with_source(e)
    }
}

impl From<String> for BonsaiError {
    fn from(s: String) -> Self {
        Self::internal(s)
    }
}

impl From<&str> for BonsaiError {
    fn from(s: &str) -> Self {
        Self::internal(s)
    }
}

// ── Result alias ──────────────────────────────────────────────────────────────

pub type BonsaiResult<T> = Result<T, BonsaiError>;

// ── Context extension trait ───────────────────────────────────────────────────

pub trait ResultExt<T> {
    fn context(self, ctx: impl Into<String>) -> BonsaiResult<T>;
    fn kind(self, kind: ErrorKind) -> BonsaiResult<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for Result<T, E> {
    fn context(self, ctx: impl Into<String>) -> BonsaiResult<T> {
        self.map_err(|e| {
            let msg = e.to_string();
            BonsaiError::internal(msg).with_source(e).with_context(ctx)
        })
    }

    fn kind(self, kind: ErrorKind) -> BonsaiResult<T> {
        self.map_err(|e| {
            let msg = e.to_string();
            BonsaiError::new(kind, msg).with_source(e)
        })
    }
}

// ── Macro ─────────────────────────────────────────────────────────────────────

#[macro_export]
macro_rules! bail {
    ($kind:expr, $($arg:tt)*) => {
        return Err($crate::BonsaiError::new($kind, format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $($arg:tt)*) => {
        if !$cond {
            return Err($crate::BonsaiError::new($kind, format!($($arg)*)));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let e = BonsaiError::not_found("module 'foo'")
            .with_context("loading kdb")
            .with_hint("run 'bonsai import' first");
        assert!(e.to_string().contains("not found"));
        assert!(e.is_transient() == false);
    }

    #[test]
    fn from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let be: BonsaiError = io_err.into();
        assert_eq!(be.kind, ErrorKind::Io);
    }

    #[test]
    fn transient_kinds() {
        assert!(BonsaiError::new(ErrorKind::Timeout, "").is_transient());
        assert!(BonsaiError::new(ErrorKind::Network, "").is_transient());
        assert!(!BonsaiError::new(ErrorKind::Internal, "").is_transient());
    }
}
