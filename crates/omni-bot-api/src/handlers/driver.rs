//! Stub for driver conversion handlers - placeholder for future implementation
//! This module is referenced by lib.rs but not yet implemented

use axum::{http::StatusCode, Json};
use std::sync::Arc;

/// Driver conversion state
#[derive(Clone, Debug)]
pub struct DriverState;

impl DriverState {
    pub fn new() -> Self {
        Self
    }
}

/// Placeholder handlers
pub async fn convert_driver() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({"error": "Driver conversion not yet implemented"})),
    )
}

pub async fn get_conversion_result() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({})))
}

pub async fn install_driver() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({})))
}
