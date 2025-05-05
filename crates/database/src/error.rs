//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors used in this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Generic environment variable error.
    #[error("{0}")]
    Env(#[source] std::env::VarError),

    /// SQLx migration error.
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// No record found in database.
    #[error("No record found in database")]
    NotFound,

    /// Generic Redis error.
    #[error(transparent)]
    Redis(#[from] bb8_redis::redis::RedisError),

    /// Generic SQLx error.
    #[error(transparent)]
    SQLx(#[from] sqlx::Error),
}
