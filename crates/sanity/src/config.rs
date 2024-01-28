//! This file contains all structures and functions used to handle the server
//! configuration. The configuration structure may be passed along all routes.

use serde::Deserialize;

use utils::filesystem::{relative_path, root_relative_path};

use crate::prelude::*;

/// Paths configuration structure.
#[derive(Debug, Deserialize)]
pub struct PathsConfig {
    /// Path where to find results to be loaded.
    pub inputs: String,

    /// Path of the HTML dashboard to serve.
    pub dashboard: String,
}

/// Global configuration structure.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub paths: PathsConfig,
}

impl Config {
    /// Creates a new configuration from environment variables and YAML
    /// configuration files.
    ///
    /// # Returns
    /// A result that contains an instance of Config.
    pub fn new() -> Res<Self> {
        let config_dir = relative_path("config/sanity")
            .or(root_relative_path("crates/sanity/config"))
            .map_err(Error::Filesystem)?;

        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("sanity.yml")))
            .build()?;

        config.try_deserialize::<Self>().map_err(Into::into)
    }
}
