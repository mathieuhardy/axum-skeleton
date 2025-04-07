//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
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

    /// Generic SQLx error.
    #[error("{0}")]
    SQLx(#[from] sqlx::Error),

    /// Generic utils error.
    #[error("{0}")]
    Utils(#[from] utils::error::Error),

    /// Validation error.
    #[error("Unprocessable entity")]
    Validation(#[from] validator::ValidationErrors),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Forbidden | Self::InvalidPassword => StatusCode::FORBIDDEN.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::Validation(_) | Self::MissingPassword => {
                StatusCode::UNPROCESSABLE_ENTITY.into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
