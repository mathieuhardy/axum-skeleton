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
    /// The user has not confirmed his email.
    #[error("User's confirmation link is expired")]
    ConfirmationLinkExpired,

    /// The user has not confirmed his email.
    #[error("User email is not found")]
    ConfirmationNotFound,

    /// The user has not confirmed his email.
    #[error("User email is not confirmed")]
    EmailNotConfirmed,

    /// Generic environment variable error.
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    /// Generic mailer variable error.
    #[error(transparent)]
    Mailer(#[from] mailer::Error),

    /// The user session is not found.
    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),

    /// The tower session is not found.
    #[error("Missing Tower session")]
    SessionNotFound,

    /// Generic SQLx error.
    #[error(transparent)]
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
            Self::ConfirmationLinkExpired => (StatusCode::FORBIDDEN, "CONFIRMATION_LINK_EXPIRED"),
            Self::ConfirmationNotFound => (StatusCode::NOT_FOUND, "CONFIRMATION_NOT_FOUND"),
            Self::EmailNotConfirmed => (StatusCode::UNAUTHORIZED, "EMAIL_NOT_CONFIRMED"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            Self::UserNotFound => (StatusCode::UNAUTHORIZED, "USER_NOT_FOUND"),
            Self::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE_ENTITY"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        (rc, ApiError::new(code, message)).into_response()
    }
}
