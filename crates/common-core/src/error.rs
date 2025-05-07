//! Error entities shared accross the application.

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// A structure used to return an error to the frontend(s).
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// A unique string identifying the error and that can be used to fetch a message translated in
    /// the user's language.
    pub code: &'static str,

    /// A human-readable message describing the error in english.
    pub message: String,
}

impl ApiError {
    /// Creates a new `ApiError` instance.
    ///
    /// # Arguments
    /// * `code` - A unique string identifying the error.
    /// * `message` - A human-readable message describing the error in english.
    ///
    /// # Returns
    /// A new `ApiError` instance.
    pub fn new(code: &'static str, message: String) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
