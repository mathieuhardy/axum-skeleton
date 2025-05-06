//! This crate provides security related entities and features to all other crates.

#![forbid(unsafe_code)]

pub mod password;

mod error;
mod prelude;

pub use crate::error::Error;
