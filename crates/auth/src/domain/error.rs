//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

use common_core::ApiError;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors used in this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Internal server error.
    #[error("Internal server error")]
    Internal,

    /// Generic SQLx error.
    #[error("{0}")]
    SQLx(#[from] sqlx::Error),

    /// Cannot authorize a user.
    #[error("Unauthorized")]
    Unauthorized,

    /// The user is not found in database.
    #[error("User not found")]
    UserNotFound,

    /// Validation error.
    #[error("Unprocessable entity")]
    Validation(#[from] validator::ValidationErrors),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (rc, code) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            Self::UserNotFound => (StatusCode::UNAUTHORIZED, "USER_NOT_FOUND"),
            Self::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE_ENTITY"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        (rc, ApiError::new(code, message)).into_response()
    }
}
