//! This file contains all routes of the application.

mod api;

use axum::Router;

#[cfg(debug_assertions)]
#[cfg(feature = "sanity")]
use tracing::{event, Level};

use auth::require_authentication;
use common_core::AppState;

use crate::config::Config;
use crate::error::ApiResult;

#[cfg(debug_assertions)]
#[cfg(feature = "sanity")]
use crate::config::Environment;

/// Builds a router for the entire application.
///
/// # Returns
/// An Axum router.
#[allow(unused_variables)]
pub fn build(config: &Config, state: AppState) -> ApiResult<Router<AppState>> {
    let mut router = Router::new();

    router = router
        // All APIs of this application
        .nest("/api", api::router())
        // Before this layer, all endpoints needs to be called by an authenticated user.
        // After this layer, authentication is not required (login for example).
        .route_layer(require_authentication!(state))
        // Special endpoints for authentication
        .merge(auth::router());

    #[cfg(feature = "k8s")]
    {
        // Special endpoints for Kubernetes
        router = router.nest("/k8", k8s::router());
    }

    #[cfg(debug_assertions)]
    #[cfg(feature = "sanity")]
    if Environment::Development.equals(&config.environment) {
        // Special endpoints for sanity dashboard
        router = router.nest("/sanity", sanity::router()?);

        event!(Level::INFO, "🩺 Sanity enabled");
    }

    Ok(router)
}
