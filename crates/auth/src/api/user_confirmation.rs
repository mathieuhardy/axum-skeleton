//! List of endpoints used for user email confirmation.

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use common_core::UseCase;
use common_state::AppState;
use mailer::FakeMailer;

use crate::application::{
    ConfirmEmail, ConfirmEmailStores, SendEmailConfirmation, SendEmailConfirmationStores,
};
use crate::domain::auth::Auth;
use crate::infrastructure::SQLxAuthStore;
use crate::prelude::*;

/// Builds a router for the authorization endpoints.
///
/// # Returns
/// An Axum router.
pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/confirm", post(confirm_email))
        .route("/send_confirmation", post(send_email_confirmation))
}

/// Parameters for the email confirmation endpoint.
#[derive(Debug, Deserialize)]
struct ConfirmEmailParams {
    /// Token (ID) of the confirmation in database.
    token: Uuid,
}

/// User email confirmation handler.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub(crate) async fn confirm_email(
    auth: Auth<SQLxAuthStore>,
    Query(params): Query<ConfirmEmailParams>,
) -> ApiResult<impl IntoResponse> {
    let stores = ConfirmEmailStores {
        auth: Arc::new(auth.store),
    };

    ConfirmEmail::new(stores).handle(params.token).await
}

/// User email confirmation re-send handler.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub(crate) async fn send_email_confirmation(
    auth: Auth<SQLxAuthStore>,
    State(state): State<AppState>,
) -> ApiResult<impl IntoResponse> {
    let user = auth.try_user()?;

    let stores = SendEmailConfirmationStores {
        mailer: Arc::new(FakeMailer::new()),
        auth: Arc::new(auth.store),
    };

    SendEmailConfirmation::new(state.config, stores)
        .handle(user)
        .await
}
