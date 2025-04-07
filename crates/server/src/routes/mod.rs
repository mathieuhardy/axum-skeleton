//! This file contains all routes of the application.

mod api;

use axum::Router;
use axum_login::login_required;

#[cfg(debug_assertions)]
#[cfg(feature = "sanity")]
use tracing::{event, Level};

use auth::AuthBackend;
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
pub fn build(config: &Config) -> ApiResult<Router<AppState>> {
    let mut router = Router::new();

    #[cfg(feature = "k8s")]
    {
        router = router.nest("/k8", k8s::router());
    }

    #[cfg(debug_assertions)]
    #[cfg(feature = "sanity")]
    if Environment::Development.equals(&config.environment) {
        router = router.nest("/sanity", sanity::router()?);

        event!(Level::INFO, "🩺 Sanity enabled");
    }

    router = router
        .nest("/api", api::router())
        .route_layer(login_required!(AuthBackend))
        .merge(auth::router());

    Ok(router)
}
