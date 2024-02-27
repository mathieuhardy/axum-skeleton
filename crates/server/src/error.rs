use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown server error")]
    Unknown,

    #[error("TODO")]
    Sqlx(#[from] sqlx::Error),

    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    #[error("{0}")]
    Filesystem(#[source] std::io::Error),

    #[error("{0}")]
    Configuration(#[from] config::ConfigError),
}
