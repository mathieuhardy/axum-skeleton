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
    /// Generic filesystem error.
    #[error(transparent)]
    Auth(#[from] auth::Error),

    /// Generic filesystem error.
    #[error("Forbidden")]
    Forbidden,

    /// Invalid password
    #[error("Invalid password")]
    InvalidPassword,

    /// Missing password
    #[error("Missing password")]
    MissingPassword,

    /// Entry not found in database.
    #[error("NotFound")]
    NotFound,

    /// Security error.
    #[error(transparent)]
    Security(#[from] security::Error),

    /// Generic SQLx error.
    #[error(transparent)]
    SQLx(#[from] sqlx::Error),

    /// Validation error.
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (rc, code) = match self {
            Self::Forbidden | Self::InvalidPassword => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            Self::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            Self::Validation(_) | Self::MissingPassword => {
                (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE_ENTITY")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        (rc, ApiError::new(code, message)).into_response()
    }
}
