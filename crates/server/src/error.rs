//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Generic Axum error.
    #[error("{0}")]
    Axum(#[source] std::io::Error),

    /// Error during the loading of the server configuration.
    #[error("{0}")]
    Configuration(#[from] config::ConfigError),

    /// Generic filesystem error.
    #[error("{0}")]
    Filesystem(#[source] std::io::Error),

    /// Invalid environment configuration provided.
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    /// Generic socket error.
    #[error("{0}")]
    Socket(#[source] std::io::Error),

    /// Generic Sqlx error.
    #[error("TODO")]
    Sqlx(#[from] sqlx::Error),

    /// Unexpected error that should never happen.
    #[error("Unexpected server error")]
    Unexpected(#[source] std::convert::Infallible),

    /// Unknown error (should be avoided).
    #[error("Unknown server error")]
    Unknown,
}
