//! This file is the entry point for the sanity dashboard. It provides a
//! function to insert it into the router.

pub(crate) mod config;
pub mod error;
pub(crate) mod prelude;

use axum::Router;
use tower_http::services::ServeDir;

use crate::prelude::*;
use utils::filesystem::{relative_path, root_relative_path};

/// Initialize the sanity module and insert the needed routes in the provided router.
///
/// # Arguments:
/// * `router` - Router instance to be populated with new routes.
///
/// # Returns:
/// The new router instance or an error.
pub fn initialize(router: Router) -> Res<Router> {
    let config = crate::config::Config::new()?;

    // TODO: don't fail if directory doesn't exists
    let inputs = root_relative_path(&config.paths.inputs).map_err(Error::Filesystem)?;

    let dashboard_dir = relative_path(&config.paths.dashboard)
        .or(root_relative_path("crates/sanity/data/dashboard"))
        .map_err(Error::Filesystem)?;

    let router = router
        .nest_service("/sanity/data", ServeDir::new(inputs))
        .nest_service("/sanity", ServeDir::new(dashboard_dir));

    Ok(router)
}
