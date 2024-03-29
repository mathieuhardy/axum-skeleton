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
        .nest("/api", api::build())
        .route_layer(login_required!(Backend))
        .merge(auth::build())
}
