//! This file contains all structures and functions used to handle the server
//! configuration. The configuration structure may be passed along all routes.

use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::Error;

/// Name of the development environment.
const DEVELOPMENT: &str = "development";

/// Name of the pre-production environment.
const STAGING: &str = "staging";

/// Name of the production environment.
const PRODUCTION: &str = "production";

/// Name of the testing environment.
const TESTING: &str = "testing";

/// Name of the default environment.
const DEFAULT_ENVIRONMENT: &str = DEVELOPMENT;

/// File name of the base configuration that is always loaded.
const BASE_CONFIG: &str = "base.yml";

/// Structure that contains all settings of the application.
#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    /// Host name of the server.
    pub host: String,

    /// Port of the server.
    pub port: u16,
}

/// Structure that contains all CORS settings.
#[derive(Debug, Deserialize)]
pub struct CorsSettings {
    /// Allowed methods.
    pub methods: Vec<String>,

    /// Allowed headers.
    pub headers: Vec<String>,

    /// Allowed origins.
    pub allow_origins: Vec<String>,
}

/// Global configuration structure.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Application settings.
    pub application: ApplicationSettings,

    /// CORS settings.
    pub cors: CorsSettings,
}

/// Possible environment values.
pub enum Environment {
    /// Development (used in local or development platform).
    Development,

    /// Pre-production environment.
    Staging,

    /// Production environment.
    Production,

    /// Unit testing environment.
    Testing,
}

impl TryFrom<String> for Environment {
    type Error = Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        match input.to_lowercase().as_str() {
            DEVELOPMENT => Ok(Self::Development),
            STAGING => Ok(Self::Staging),
            PRODUCTION => Ok(Self::Production),
            TESTING => Ok(Self::Testing),

            unsupported => Err(Self::Error::InvalidEnvironment(unsupported.to_string())),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Environment::Development => DEVELOPMENT,
            Environment::Staging => STAGING,
            Environment::Production => PRODUCTION,
            Environment::Testing => TESTING,
        };

        write!(f, "{output}")
    }
}

impl Config {
    /// Creates a new configuration from environment variables and YAML
    /// configuration files.
    ///
    /// # Returns
    /// A result that contains an instance of Config.
    pub fn new() -> Result<Self, Error> {
        let environment: Environment = std::env::var("ENVIRONMENT")
            .unwrap_or(DEFAULT_ENVIRONMENT.into())
            .try_into()?;

        Self::from_env(&environment)
    }

    /// Creates a new configuration from a given environment.
    ///
    /// # Returns
    /// A result that contains an instance of Config.
    pub fn from_env(environment: &Environment) -> Result<Self, Error> {
        let config_dir = match std::env::var("CARGO_MANIFEST_DIR") {
            Ok(dir) => PathBuf::from_str(&dir).map_err(Error::Unexpected),
            Err(_) => std::env::current_dir().map_err(Error::Filesystem),
        }?;

        let exists = config_dir
            .join("config")
            .try_exists()
            .map_err(Error::Filesystem)?;

        let config_dir = if !exists {
            config_dir.join("crates/server") // TODO: not satisfying
        } else {
            config_dir
        }
        .join("config");

        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join(BASE_CONFIG)))
            .add_source(config::File::from(
                config_dir.join(format!("{environment}.yml")),
            ))
            .add_source(
                config::Environment::with_prefix("OVERRIDE")
                    .prefix_separator("_")
                    .separator("_"),
            )
            .build()?;

        config.try_deserialize::<Self>().map_err(Into::into)
    }
}
