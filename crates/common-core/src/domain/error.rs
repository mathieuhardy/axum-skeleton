//! Error entities shared accross the application.

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &'static str, message: String) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
