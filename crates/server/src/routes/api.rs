//! This file contains all routes binding to our APIs.

use axum::Router;

use common_core::AppState;

/// Builds a router for the APIs.
///
/// # Returns
/// An Axum router.
pub fn router() -> Router<AppState> {
    // List all crates that provide APIs
    Router::new().nest("/users", user::router())
}
