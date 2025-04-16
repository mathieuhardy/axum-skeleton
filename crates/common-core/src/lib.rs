//! Common structure and utilities used by other crates.

#![forbid(unsafe_code)]

pub mod domain;

pub use domain::error::ApiError;
pub use domain::state::{AppState, RedisPool};
pub use domain::use_case::UseCase;
