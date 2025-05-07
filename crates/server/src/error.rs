//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

use common_core::ApiError;
use database::Error as DatabaseError;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Generic Axum error.
    #[error("{0}")]
    Axum(#[source] std::io::Error),

    /// Error during the loading of the server configuration.
    #[error(transparent)]
    Configuration(#[from] configuration::Error),

    /// Database read/write error.
    #[error(transparent)]
    Database(#[from] DatabaseError),

    /// Generic filesystem error.
    #[error(transparent)]
    Filesystem(#[from] utils::error::Error),

    /// Generic filesystem error.
    #[error("Forbidden")]
    Forbidden,

    /// Invalid environment configuration provided.
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    /// Generic sanity error.
    #[cfg(feature = "sanity")]
    #[error(transparent)]
    Sanity(#[from] sanity::Error),

    /// Generic socket error.
    #[error("{0}")]
    Socket(#[source] std::io::Error),

    /// Generic SQLx error.
    #[error(transparent)]
    SQLx(#[from] sqlx::Error),

    /// Unexpected error that should never happen.
    #[error("Unexpected server error")]
    Unexpected(#[source] std::convert::Infallible),

    /// Cannot authorize a user.
    #[error("Unauthorized")]
    Unauthorized,

    /// Unknown error (should be avoided).
    #[error("Unknown server error")]
    Unknown,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (rc, code) = match self {
            Self::Database(DatabaseError::NotFound) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        (rc, ApiError::new(code, message)).into_response()
    }
}
