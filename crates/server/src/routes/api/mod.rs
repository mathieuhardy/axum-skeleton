//! This file contains all routes binding to our APIs.

mod v1;

use axum::Router;

use crate::state::State;

/// Builds a router for the APIs.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<State> {
    Router::new().nest("/users", v1::users::build())
}
