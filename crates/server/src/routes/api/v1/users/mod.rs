//! This file contains all routes dedicated to the users management.

//use crate::types::form_or_json::*;

use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;

use database::models::users::UserRequest;

use crate::prelude::*;
use crate::state::State;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<State> {
    Router::new()
        .route("/me", get(me))
        .route("/", post(post_user))
}

/// Handler used to get information about the currently logged user.
#[axum::debug_handler]
async fn me() -> Html<&'static str> {
    Html("Hello you")
}

/// Handler used to create a new user.
async fn post_user(FormOrJson(user): FormOrJson<UserRequest>) {
    dbg!(user);
}
