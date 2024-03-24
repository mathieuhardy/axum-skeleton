//! This file contains all routes of the application.

mod api;
mod auth;
#[cfg(feature = "k8s")]
mod k8s;

use axum_login::login_required;

use crate::prelude::*;

/// Builds a router for the entire application.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    let router = Router::new();

    #[cfg(feature = "k8s")]
    let router = router.nest("/k8", k8s::build());

    router
        .route("/protected", get(protected))
        .route_layer(login_required!(Backend))
        .merge(auth::build())
        .nest("/api", api::build())
}

/// Protected handler.
#[axum::debug_handler]
async fn protected() -> impl IntoResponse {
    axum::response::Html("<h1>Hello, World!</h1>")
}
