//! List of endpoints used for authentication process (login, logout, ...).

#[cfg(test)]
mod tests;

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tracing::instrument;
use validator::Validate;

use common_core::{AppState, UseCase};
use common_web::extractor::FormOrJson;

use crate::application::{Login, Logout};
use crate::domain::auth_user::AuthCredentials;
use crate::infrastructure::{SQLxAuthRepository, SQLxAuthSession};
use crate::prelude::*;

/// Builds a router for the authorization crate.
///
/// # Returns
/// An Axum router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

/// Login handler.
///
/// # Arguments
/// * `auth_session`: Authentication session.
/// * `credentials`: User's credentials to authenticate.
///
/// # Returns
/// One of these HTTP codes:
///   - 200: OK,
///   - 401: UNAUTHORIZED,
///   - 500: INTERNAL_SERVER_ERROR.
#[instrument]
#[axum::debug_handler]
pub async fn login(
    auth_session: SQLxAuthSession,
    FormOrJson(credentials): FormOrJson<AuthCredentials>,
) -> ApiResult<impl IntoResponse> {
    credentials.validate()?;

    Login::<SQLxAuthRepository>::new()
        .handle((auth_session, credentials))
        .await
}

/// Logout handler.
//
/// # Arguments
/// * `auth_session`: Current user session.
///
/// # Returns
/// One of these HTTP codes:
///   - 200: OK,
///   - 500: INTERNAL_SERVER_ERROR.
#[instrument]
#[axum::debug_handler]
pub async fn logout(auth_session: SQLxAuthSession) -> impl IntoResponse {
    Logout::<SQLxAuthRepository>::new()
        .handle(auth_session)
        .await
}
