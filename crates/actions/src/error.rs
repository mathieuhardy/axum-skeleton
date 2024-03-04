//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

use database::error::Error as DatabaseError;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Database read/write error.
    #[error("{0}")]
    Database(#[from] DatabaseError),

    /// Utils error.
    #[error("{0}")]
    Utils(#[from] utils::error::Error),

    /// Password provided is invalid.
    #[error("Provided password is invalid")]
    InvalidPassword,
}
