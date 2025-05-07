//! Common structure and utilities used by other crates.

#![forbid(unsafe_code)]

mod error;
mod use_case;

pub use error::ApiError;
pub use use_case::UseCase;
