//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use std::env::VarError;
use thiserror::Error;

use database::error::Error as DatabaseError;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Generic Axum error.
    #[error("{0}")]
    Axum(#[source] std::io::Error),

    /// Error during the loading of the server configuration.
    #[error("{0}")]
    Configuration(#[from] config::ConfigError),

    /// Database read/write error.
    #[error("{0}")]
    Database(#[from] DatabaseError),

    /// Generic environment variable error.
    #[error("{0}")]
    Env(#[source] VarError),

    /// Generic filesystem error.
    #[error("{0}")]
    Filesystem(#[source] utils::error::Error),

    /// Invalid environment configuration provided.
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    /// Generic sanity error.
    #[error("{0}")]
    Sanity(#[source] sanity::error::Error),

    /// Generic socket error.
    #[error("{0}")]
    Socket(#[source] std::io::Error),

    /// Generic SQLx error.
    #[error("{0}")]
    SQLx(#[from] sqlx::Error),

    /// Unexpected error that should never happen.
    #[error("Unexpected server error")]
    Unexpected(#[source] std::convert::Infallible),

    /// Unknown error (should be avoided).
    #[error("Unknown server error")]
    Unknown,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(DatabaseError::NotFound) => StatusCode::NOT_FOUND.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
