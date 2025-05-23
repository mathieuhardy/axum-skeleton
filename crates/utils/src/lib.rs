//! This file lists the utilities modules provided by this crate.

#![forbid(unsafe_code)]

pub mod error;

#[cfg(feature = "fs")]
pub mod filesystem;

#[cfg(feature = "hashing")]
pub mod hashing;

pub use error::Error;
