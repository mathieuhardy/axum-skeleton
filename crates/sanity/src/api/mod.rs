//! List of endpoints used to display the sanity dashboard.

use axum::http::Uri;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tower_http::services::{Redirect as HttpRedirect, ServeDir};

use common_core::AppState;
use utils::filesystem::{create_root_relative_path, relative_path, root_relative_path};

use crate::prelude::*;

/// Builds a router for the authorization crate.
///
/// # Returns
/// An Axum router.
pub fn router() -> ApiResult<Router<AppState>> {
    let config = crate::domain::config::Config::new()?;

    let inputs = match root_relative_path(&config.paths.inputs) {
        Ok(path) => path,
        Err(_) => create_root_relative_path(&config.paths.inputs)?,
    };

    let dashboard_dir = relative_path(&config.paths.dashboard)
        .or(root_relative_path("crates/sanity/data/dashboard"))
        .map_err(Error::Filesystem)?;

    let router = Router::new()
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

    Ok(router)
}

/// Redirects the call to a file.
///
/// # Returns
/// An Axum redirect object.
async fn redirect() -> Redirect {
    Redirect::permanent("crates.html")
}
