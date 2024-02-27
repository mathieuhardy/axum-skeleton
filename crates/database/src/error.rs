//! This file contains all possible errors handled in this crate. If also
//! provides the conversions from other error types.

use thiserror::Error;

/// Helper for return types inside this crate.
pub type Res<T> = Result<T, Error>;

/// Enumerates the possible errors returned by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// No record found in database.
    #[error("No record found in database")]
    NotFound,

    /// Generic SQLx error.
    #[error("{0}")]
    SQLx(#[source] sqlx::Error),
}
