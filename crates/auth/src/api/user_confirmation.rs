//! List of endpoints used for user email confirmation.

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tracing::instrument;
use uuid::Uuid;

use common_core::UseCase;
use common_state::AppState;

use crate::application::ConfirmEmail;
use crate::domain::auth::Auth;
use crate::infrastructure::SQLxAuthStore;
use crate::prelude::*;

/// Builds a router for the authorization endpoints.
///
/// # Returns
/// An Axum router.
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/confirm", post(confirm_email))
}

/// User email confirmation handler.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub(crate) async fn confirm_email(
    auth: Auth<SQLxAuthStore>,
    Query(token): Query<Uuid>,
) -> ApiResult<impl IntoResponse> {
    ConfirmEmail::new().handle((auth, token)).await
}
