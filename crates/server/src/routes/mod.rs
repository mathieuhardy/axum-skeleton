//! This file contains all routes of the application.

mod api;
#[cfg(feature = "k8s")]
mod k8s;

use axum::response::Html;

use crate::prelude::*;

/// Builds a router for the entire application.
///
/// # Returns
/// An Axum router.
pub async fn build() -> Router<AppState> {
    let router = Router::new();

    #[cfg(feature = "k8s")]
    let router = router.nest("/k8", k8s::build());

    router
        .route("/", get(hello().await))
        .nest("/api", api::build())
}

/// Demo handler.
#[axum::debug_handler]
async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
