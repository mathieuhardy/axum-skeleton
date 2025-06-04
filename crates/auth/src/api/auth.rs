//! List of endpoints used for authentication process (login, logout, ...).

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tracing::instrument;
use validator::Validate;

use common_core::UseCase;
use common_state::AppState;
use common_web::extractor::FormOrJson;
use database::Db;

use crate::application::{Login, LoginStores, Logout};
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
#[instrument]
#[axum::debug_handler(state = AppState)]
pub(crate) async fn login(
    auth: Auth,
    db: Db,
    FormOrJson(credentials): FormOrJson<AuthCredentials>,
) -> ApiResult<impl IntoResponse> {
    credentials.validate()?;

    let db = db.into_shared();

    let stores = LoginStores {
        auth: SQLxAuthStore::new(&db),
    };

    Login::new(stores).handle((auth, credentials)).await
}

/// Logout handler.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub(crate) async fn logout(auth: Auth) -> ApiResult<impl IntoResponse> {
    Logout::new().handle(auth).await
}
