//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Error during the loading of the sanity configuration.
    #[error("{0}")]
    Configuration(#[from] config::ConfigError),

    /// Generic filesystem error.
    #[error("{0}")]
    Filesystem(#[source] utils::error::Error),
}
