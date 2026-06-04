//! Error types for the AI fallback framework

use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    #[error("AI domain timed out")]
    AiTimeout,

    #[error("AI confidence below threshold")]
    ConfidenceTooLow,

    #[error("AI output inconsistent")]
    Inconsistent,

    #[error("AI domain unhealthy")]
    AiUnhealthy,

    #[error("Heuristic layer failed")]
    HeuristicFailed,

    #[error("Deterministic core failed")]
    CoreFailed,

    #[error("Invalid configuration")]
    InvalidConfig,

    #[error("Resource limit exceeded")]
    ResourceLimited,

    #[error("Unknown error")]
    Unknown,
}

impl Error {
    pub fn is_ai_error(&self) -> bool {
        matches!(
            self,
            Error::AiTimeout | Error::ConfidenceTooLow | Error::Inconsistent | Error::AiUnhealthy
        )
    }

    pub fn should_fallback(&self) -> bool {
        self.is_ai_error() || matches!(self, Error::HeuristicFailed)
    }
}
