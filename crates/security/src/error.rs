//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type ApiResult<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Hashing error using Argon2
    #[error(transparent)]
    Utils(#[from] utils::Error),
}
