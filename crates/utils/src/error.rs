//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Hashing error using Argon2
    #[error("Cannot hash string: {0}")]
    Hashing(String),

    /// File or directory not found.
    #[error("Path not found: {0:?}")]
    PathNotFound(std::path::PathBuf),

    /// Generic filesystem error.
    #[error(transparent)]
    Filesystem(#[from] std::io::Error),

    /// Generic tokio task joining error.
    #[cfg(feature = "hashing")]
    #[error(transparent)]
    TaskJoin(#[from] tokio::task::JoinError),

    /// Unexpected error that should never happen.
    #[error("Unexpected server error")]
    Unexpected(#[source] std::convert::Infallible),
}
