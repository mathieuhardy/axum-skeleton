//! This file contains all routes binding to our APIs.

mod v1;

use crate::prelude::*;

/// Builds a router for the APIs.
///
/// # Returns
/// An Axum router.
pub fn build() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/users", v1::users::build())
}
