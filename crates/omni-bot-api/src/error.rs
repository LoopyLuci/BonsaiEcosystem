//! API error types and handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// API error types
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Environment not found: {0}")]
    EnvironmentNotFound(String),

    #[error("Service already exists: {0}")]
    ServiceAlreadyExists(String),

    #[error("Environment already exists: {0}")]
    EnvironmentAlreadyExists(String),

    #[error("Module already installed: {0}")]
    ModuleAlreadyInstalled(String),

    #[error("Invalid resource allocation: {0}")]
    InvalidResourceAllocation(String),

    #[error("Operation not allowed: {0}")]
    OperationNotAllowed(String),

    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),

    #[error("Snapshot not found: {0}")]
    SnapshotNotFound(String),

    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Asset not found: {0}")]
    AssetNotFound(String),

    #[error("Asset already exists: {0}")]
    AssetAlreadyExists(String),

    #[error("Asset generation failed: {0}")]
    AssetGenerationFailed(String),

    #[error("Invalid asset type: {0}")]
    InvalidAssetType(String),

    #[error("Workflow not found: {0}")]
    WorkflowNotFound(String),

    #[error("Workflow already exists: {0}")]
    WorkflowAlreadyExists(String),

    #[error("Invalid workflow DAG: {0}")]
    InvalidWorkflowDAG(String),

    #[error("Workflow execution failed: {0}")]
    WorkflowExecutionFailed(String),

    #[error("Workflow step failed: {0}")]
    WorkflowStepFailed(String),

    #[error("Workflow rollback failed: {0}")]
    WorkflowRollbackFailed(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("UMS publishing failed: {0}")]
    UMSPublishingFailed(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InvalidRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::ServiceNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::ModuleNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::EnvironmentNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::ServiceAlreadyExists(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::EnvironmentAlreadyExists(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::ModuleAlreadyInstalled(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::InvalidResourceAllocation(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::OperationNotAllowed(ref msg) => (StatusCode::FORBIDDEN, msg.clone()),
            ApiError::SignatureVerificationFailed(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApiError::SnapshotNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::MigrationFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::ExecutionFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::Timeout(ref msg) => (StatusCode::REQUEST_TIMEOUT, msg.clone()),
            ApiError::Internal(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::Serialization(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            ApiError::Io(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            ApiError::AssetNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::AssetAlreadyExists(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::AssetGenerationFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::InvalidAssetType(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::WorkflowNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::WorkflowAlreadyExists(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::InvalidWorkflowDAG(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::WorkflowExecutionFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::WorkflowStepFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::WorkflowRollbackFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::InvalidParameter(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::UMSPublishingFailed(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
            "error_type": format!("{}", self),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let err = ApiError::EnvironmentNotFound("test".to_string());
        assert_eq!(err.to_string(), "Environment not found: test");
    }

    #[test]
    fn test_api_error_response() {
        let err = ApiError::InvalidRequest("bad input".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
