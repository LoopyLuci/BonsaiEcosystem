//! Stub for HDE model management handlers - placeholder for future implementation
//! This module is referenced by lib.rs but not yet implemented

use axum::{http::StatusCode, Json};
use std::sync::Arc;

/// HDE state
#[derive(Clone, Debug)]
pub struct HdeState;

impl HdeState {
    pub fn new() -> Self {
        Self
    }
}

/// Placeholder handlers
pub async fn list_models() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({"error": "HDE not yet implemented"})),
    )
}

pub async fn promote_model() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({})))
}

pub async fn demote_model() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({})))
}

pub async fn get_shadow_reports() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({})))
}

pub async fn validate_shadow_model() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({})))
}
