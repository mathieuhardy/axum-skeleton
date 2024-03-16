//! This file is the entry point for the sanity dashboard. It provides a
//! function to insert it into the router.

#![forbid(unsafe_code)]

pub(crate) mod config;
pub mod error;
pub(crate) mod prelude;

use axum::http::Uri;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tower_http::services::{Redirect as HttpRedirect, ServeDir};

use crate::prelude::*;
use utils::filesystem::{create_root_relative_path, relative_path, root_relative_path};

/// Initialize the sanity module and insert the needed routes in the provided router.
///
/// # Arguments
/// * `router` - Router instance to be populated with new routes.
///
/// # Returns
/// The new router instance or an error.
pub fn initialize(router: Router) -> Res<Router> {
    let config = crate::config::Config::new()?;

    let inputs = match root_relative_path(&config.paths.inputs) {
        Ok(path) => path,
        Err(_) => create_root_relative_path(&config.paths.inputs)?,
    };

    let dashboard_dir = relative_path(&config.paths.dashboard)
        .or(root_relative_path("crates/sanity/data/dashboard"))
        .map_err(Error::Filesystem)?;

    let sanity_router = Router::new()
        // Redirect index.html to crates.html
        .route("/index.html", get(redirect))
        // Serve all files under /sanity (other html files)
        .nest_service(
            "/",
            ServeDir::new(dashboard_dir).fallback(HttpRedirect::<String>::permanent(
                "/sanity/crates.html".parse::<Uri>().unwrap(),
            )),
        )
        // Serve all resources files (css, js, ...)
        .nest_service("/data", ServeDir::new(inputs))
        // In case a not found is fired, redirect to crates.html
        .fallback(redirect);

    let router = router.nest("/sanity", sanity_router);

    Ok(router)
}

/// Redirects the call to a file.
///
/// # Returns
/// An Axum redirect object.
async fn redirect() -> Redirect {
    Redirect::permanent("crates.html")
}
