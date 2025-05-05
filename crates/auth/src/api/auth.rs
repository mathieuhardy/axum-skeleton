//! List of endpoints used for authentication process (login, logout, ...).

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tracing::instrument;
use validator::Validate;

use common_core::{AppState, UseCase};
use common_web::extractor::FormOrJson;

use crate::application::{Login, Logout};
use crate::domain::auth::{Auth, AuthCredentials};
use crate::infrastructure::SQLxAuthStore;
use crate::prelude::*;

/// Builds a router for the authorization endpoints.
///
/// # Returns
/// An Axum router.
pub(crate) fn router() -> Router<AppState> {
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
#[axum::debug_handler(state = AppState)]
pub(crate) async fn login(
    auth: Auth<SQLxAuthStore>,
    FormOrJson(credentials): FormOrJson<AuthCredentials>,
) -> ApiResult<impl IntoResponse> {
    credentials.validate()?;

    Login::new().handle((auth, credentials)).await
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
#[axum::debug_handler(state = AppState)]
pub(crate) async fn logout(auth: Auth<SQLxAuthStore>) -> impl IntoResponse {
    Logout::new().handle(auth).await
}
