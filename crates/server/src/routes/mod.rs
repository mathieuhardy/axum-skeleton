//! This file contains all routes of the application.

#[cfg(feature = "k8s")]
mod k8s;
mod users;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::state::State;

/// Builds a router for the entire application.
///
/// # Returns
/// An Axum router.
pub async fn build() -> Router<State> {
    let router = Router::new()
        .route("/", get(hello().await))
        .nest("/users", users::build());

    #[cfg(feature = "k8s")]
    let router = router.nest("/k8", k8s::build());

    router
}

/// Demo handler.
#[axum::debug_handler]
async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
