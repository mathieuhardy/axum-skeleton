//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// File or directory not found.
    #[error("Path not found: {0}")]
    PathNotFound(std::path::PathBuf),

    /// Generic filesystem error.
    #[error("{0}")]
    Filesystem(#[source] std::io::Error),

    /// Unexpected error that should never happen.
    #[error("Unexpected server error")]
    Unexpected(#[source] std::convert::Infallible),
}
