//! Timeouts configurations for the whole server.

use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

use crate::config::Config;

/// Gets the layer used to configure the routes timeout.
///
/// # Arguments
/// * `config`- Server configuration.
///
/// # Returns
/// The timeout layer.
pub fn timeout_layer(config: &Config) -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(config.application.timeout))
}
