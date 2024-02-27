//! This file is the entry point for the sanity dashboard. It provides a
//! function to insert it into the router.

pub mod error;
pub mod prelude;

use axum::Router;
use tower_http::services::ServeDir;

use utils::filesystem::relative_path;

use crate::prelude::*;

/// Initialize the sanity module and insert the needed routes in the provided router.
///
/// # Arguments:
/// * `router` - Router instance to be populated with new routes.
///
/// # Returns:
/// The new router instance or an error.
pub fn initialize(router: Router) -> Res<Router> {
    // TODO: Get name of folder through configuration file
    let dashboard_path = "data/dashboard";

    let sanity_dir = relative_path(dashboard_path)
        .or(relative_path(&format!("crates/sanity/{dashboard_path}"))) // TODO: not satisfying
        .map_err(Error::Filesystem)?;

    Ok(router.nest_service("/sanity", ServeDir::new(sanity_dir)))
}
