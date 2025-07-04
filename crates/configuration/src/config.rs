//! This file contains all structures and functions used to handle the server
//! configuration. The configuration structure may be passed along all routes.

use serde::Deserialize;
use std::fmt;

use utils::filesystem::{relative_path, root_relative_path};

use crate::error::*;

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
#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationSettings {
    /// Host name of the server.
    pub host: String,

    /// Port of the server.
    pub port: u16,

    /// Timeout value for routes (in seconds).
    pub timeout: u64,
}

/// Structure that contains all CORS settings.
#[derive(Clone, Debug, Deserialize)]
pub struct CorsSettings {
    /// Allowed methods.
    pub methods: Vec<String>,

    /// Allowed headers.
    pub headers: Vec<String>,

    /// Allowed origins.
    pub allow_origins: Vec<String>,
}

/// Structure that contains all passwords settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PasswordSettings {
    /// Allowed methods.
    pub pattern: PasswordPatternSettings,
}

/// Structure that contains all sessions settings.
#[derive(Clone, Debug, Deserialize)]
pub struct SessionsSettings {
    /// Timeout for the user session.
    pub timeout_in_hours: u32,
}

/// Structure that contains all sessions settings.
#[derive(Clone, Debug, Deserialize)]
pub struct AuthSettings {
    /// Timeout for the user's email confirmation.
    pub email_confirmation_timeout_hours: u32,
}

/// Structure that contains all passwords's pattern settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PasswordPatternSettings {
    /// Must contains at least one digit.
    pub digit: bool,

    /// Must contains at least one lowercase letter.
    pub lowercase: bool,

    /// Must contains at least one uppercase letter.
    pub uppercase: bool,

    /// Must contains at least one special character.
    pub special: bool,

    /// Can contains some spaces.
    pub spaces: bool,

    /// Minimum length of the password.
    pub min_length: u32,

    /// Maximum length of the password (should never be set).
    pub max_length: Option<u32>,
}

/// Global configuration structure.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// Application settings.
    pub application: ApplicationSettings,

    /// CORS settings.
    pub cors: CorsSettings,

    /// Environment value.
    pub environment: String,

    /// Passwords configuration.
    pub password: PasswordSettings,

    /// Sessions configuration.
    pub sessions: SessionsSettings,

    /// Authentication configuration.
    pub auth: AuthSettings,
}

/// Possible environment values.
#[derive(Debug, Deserialize)]
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

impl TryFrom<Environment> for Config {
    type Error = Error;

    fn try_from(environment: Environment) -> Result<Self, Self::Error> {
        let config_dir = relative_path("config/server")
            .or(root_relative_path("crates/configuration/data"))
            .map_err(Error::Filesystem)?;

        let config = config::Config::builder()
            .set_default("environment", &environment)?
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

impl From<&Environment> for config::ValueKind {
    fn from(value: &Environment) -> Self {
        Self::String(value.to_string())
    }
}

impl Environment {
    /// Checks if the environment value equals a provided one.
    ///
    /// # Arguments
    /// * `value` - Value to check.
    ///
    /// # Returns
    /// `true` if self equals the given value, `false` otherwise.
    pub fn equals(&self, value: &str) -> bool {
        self.to_string().as_str() == value
    }
}

impl Config {
    /// Creates a new configuration from environment variables and YAML
    /// configuration files.
    ///
    /// # Returns
    /// A result that contains an instance of Config.
    pub fn new() -> ApiResult<Self> {
        let environment: Environment = std::env::var("ENVIRONMENT")
            .unwrap_or(DEFAULT_ENVIRONMENT.into())
            .try_into()?;

        environment.try_into()
    }
}
