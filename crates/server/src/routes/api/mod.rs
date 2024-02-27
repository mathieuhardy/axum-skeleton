//! This file contains all routes binding to our APIs.

mod v1;

use crate::prelude::*;

/// Builds a router for the APIs.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    Router::new().nest("/users", v1::users::build())
}
