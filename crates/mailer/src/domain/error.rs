//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

use common_core::ApiError;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors used in this crate.
#[derive(Debug, Error)]
pub enum Error {}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (rc, code) = (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR");

        (rc, ApiError::new(code, message)).into_response()
    }
}
