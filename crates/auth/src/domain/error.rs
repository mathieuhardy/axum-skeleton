//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

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
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
