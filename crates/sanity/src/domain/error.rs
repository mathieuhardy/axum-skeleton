//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors returned in this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Error during the loading of the sanity configuration.
    #[error(transparent)]
    Configuration(#[from] config::ConfigError),

    /// Generic filesystem error.
    #[error(transparent)]
    Filesystem(#[from] utils::error::Error),
}
