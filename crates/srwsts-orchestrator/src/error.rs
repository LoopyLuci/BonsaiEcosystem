//! Error types for the SRWSTS orchestrator.

use thiserror::Error;
use uuid::Uuid;

/// Result type for orchestrator operations.
pub type Result<T> = std::result::Result<T, OrchestratorError>;

/// Comprehensive error type for orchestrator failures.
#[derive(Debug, Error, Clone)]
pub enum OrchestratorError {
    #[error("job {0} not found")]
    JobNotFound(Uuid),

    #[error("worker {0} not found")]
    WorkerNotFound(String),

    #[error("baseline {0} not found")]
    BaselineNotFound(String),

    #[error("no available workers")]
    NoAvailableWorkers,

    #[error("job {0} already scheduled")]
    JobAlreadyScheduled(Uuid),

    #[error("invalid job status transition: {from} -> {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("worker pool error: {0}")]
    WorkerPoolError(String),

    #[error("scheduler error: {0}")]
    SchedulerError(String),

    #[error("baseline manager error: {0}")]
    BaselineManagerError(String),

    #[error("result collection error: {0}")]
    ResultCollectionError(String),

    #[error("CAS access error: {0}")]
    CasError(String),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("worker communication timeout")]
    WorkerCommunicationTimeout,

    #[error("regression detected: metric '{metric}' degraded from {baseline} to {current}")]
    RegressionDetected {
        metric: String,
        baseline: f64,
        current: f64,
    },

    #[error("internal error: {0}")]
    InternalError(String),

    #[error("orchestrator has shutdown")]
    Shutdown,
}

impl OrchestratorError {
    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            OrchestratorError::WorkerCommunicationTimeout
                | OrchestratorError::NoAvailableWorkers
        )
    }

    /// Get a human-readable description suitable for logs.
    pub fn description(&self) -> &str {
        match self {
            OrchestratorError::JobNotFound(_) => "Job not found in registry",
            OrchestratorError::WorkerNotFound(_) => "Worker not found in pool",
            OrchestratorError::BaselineNotFound(_) => "Baseline not found in CAS",
            OrchestratorError::NoAvailableWorkers => "All workers are busy or unavailable",
            OrchestratorError::JobAlreadyScheduled(_) => "Job already scheduled",
            OrchestratorError::InvalidStatusTransition { .. } => "Invalid state transition",
            OrchestratorError::WorkerPoolError(_) => "Worker pool operation failed",
            OrchestratorError::SchedulerError(_) => "Job scheduler failed",
            OrchestratorError::BaselineManagerError(_) => "Baseline manager failed",
            OrchestratorError::ResultCollectionError(_) => "Result collection failed",
            OrchestratorError::CasError(_) => "Content-addressable storage access failed",
            OrchestratorError::SerializationError(_) => "Serialization/deserialization failed",
            OrchestratorError::WorkerCommunicationTimeout => "Worker communication timed out",
            OrchestratorError::RegressionDetected { .. } => "Performance regression detected",
            OrchestratorError::InternalError(_) => "Internal orchestrator error",
            OrchestratorError::Shutdown => "Orchestrator is shutting down",
        }
    }
}
