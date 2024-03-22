//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use axum::http::StatusCode;
use thiserror::Error;

use actions::error::Error as ActionsError;
use database::error::Error as DatabaseError;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Actions error.
    #[error("{0}")]
    Actions(#[from] ActionsError),

    /// Generic Axum error.
    #[error("{0}")]
    Axum(#[source] std::io::Error),

    /// Error during the loading of the server configuration.
    #[error("{0}")]
    Configuration(#[from] config::ConfigError),

    /// Database read/write error.
    #[error("{0}")]
    Database(#[from] DatabaseError),

    /// Generic filesystem error.
    #[error("{0}")]
    Filesystem(#[from] utils::error::Error),

    /// Invalid environment configuration provided.
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    /// Generic sanity error.
    #[cfg(feature = "sanity")]
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

    /// Cannot authorize a user.
    #[error("Unauthorized")]
    Unauthorized,

    /// Unknown error (should be avoided).
    #[error("Unknown server error")]
    Unknown,

    /// Validation error.
    #[error("Unprocessable entity")]
    Validation(#[from] validator::ValidationErrors),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Actions(ActionsError::InvalidPassword) => StatusCode::FORBIDDEN.into_response(),
            Self::Database(DatabaseError::NotFound) => StatusCode::NOT_FOUND.into_response(),
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
