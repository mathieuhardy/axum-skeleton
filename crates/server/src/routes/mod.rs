//! This file contains all routes of the application.

mod api;
#[cfg(feature = "k8s")]
mod k8s;
mod openapi;

use crate::prelude::*;

/// Builds a router for the entire application.
///
/// # Returns
/// An Axum router.
pub fn build() -> ApiRouter<AppState> {
    let router = ApiRouter::new();

    #[cfg(feature = "k8s")]
    let router = router.nest("/k8", k8s::build());

    router
        .nest("/openapi", openapi::build())
        .nest("/api", api::build())
}
