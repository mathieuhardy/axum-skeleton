use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Configuration(#[from] config::ConfigError),

    #[error("{0}")]
    Filesystem(#[source] std::io::Error),

    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    #[error("{0}")]
    Socket(#[source] std::io::Error),

    #[error("TODO")]
    Sqlx(#[from] sqlx::Error),

    #[error("Unknown server error")]
    Unknown,
}
