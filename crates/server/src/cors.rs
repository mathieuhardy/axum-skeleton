use axum::http::header::{HeaderName, HeaderValue};
use axum::http::Method;
use std::str::FromStr;
use tower_http::cors::CorsLayer;

use crate::config::Config;

pub fn create(config: &Config) -> CorsLayer {
    let methods: Vec<_> = config
        .cors
        .methods
        .iter()
        .filter_map(|method| {
            Method::from_str(method)
                .map_err(|e| log::warn!("{}", e))
                .ok()
        })
        .collect();

    let headers: Vec<_> = config
        .cors
        .headers
        .iter()
        .filter_map(|header| {
            HeaderName::from_str(header)
                .map_err(|e| log::warn!("{}", e))
                .ok()
        })
        .collect();

    let allow_origins: Vec<_> = config
        .cors
        .allow_origins
        .iter()
        .filter_map(|origin| {
            HeaderValue::from_str(origin)
                .map_err(|e| log::warn!("{}", e))
                .ok()
        })
        .collect();

    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(methods)
        .allow_headers(headers)
        .allow_origin(allow_origins)
}
