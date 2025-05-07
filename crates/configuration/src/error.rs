//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

use common_core::ApiError;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Error during the loading of the server configuration.
    #[error(transparent)]
    Configuration(#[from] config::ConfigError),

    /// Generic filesystem error.
    #[error(transparent)]
    Filesystem(#[from] utils::error::Error),

    /// Invalid environment configuration provided.
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (rc, code) = match self {
            // Self::Database(DatabaseError::NotFound) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            // Self::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            Self::InvalidEnvironment(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INVALID_ENVIRONMENT")
            }

            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        (rc, ApiError::new(code, message)).into_response()
    }
}
