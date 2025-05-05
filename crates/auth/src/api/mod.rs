//! List of endpoints provided by this crate.

mod auth;

use axum::Router;

use common_core::AppState;

/// Builds a router for the authorization crate.
///
/// # Returns
/// An Axum router.
pub fn router() -> Router<AppState> {
    Router::new().merge(auth::router())
}
