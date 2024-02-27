use config::{Config, File};
use serde::Deserialize;
//use std::error::Error;
use std::fmt;

use crate::error::Error;

const DEVELOPMENT: &str = "development";
const STAGING: &str = "staging";
const PRODUCTION: &str = "production";
const DEFAULT_ENVIRONMENT: &str = DEVELOPMENT;

const BASE_SETTINGS: &str = "base.yml";

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

pub enum Environment {
    Development,
    Staging,
    Production,
}

impl TryFrom<String> for Environment {
    type Error = Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        match input.to_lowercase().as_str() {
            DEVELOPMENT => Ok(Self::Development),
            STAGING => Ok(Self::Staging),
            PRODUCTION => Ok(Self::Production),

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
        };

        write!(f, "{output}")
    }
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let environment: Environment = std::env::var("ENVIRONMENT")
            .unwrap_or(DEFAULT_ENVIRONMENT.into())
            .try_into()?;

        let settings_dir = std::env::current_dir()
            .map_err(Error::Filesystem)?
            .join("settings");

        let settings = Config::builder()
            .add_source(File::from(settings_dir.join(BASE_SETTINGS)))
            .add_source(File::from(settings_dir.join(format!("{environment}.yml"))))
            .add_source(
                config::Environment::with_prefix("OVERRIDE")
                    .prefix_separator("_")
                    .separator("_"),
            )
            .build()?;

        settings.try_deserialize::<Self>().map_err(Into::into)
    }
}
