//! Gather all methods that will be called by the server endpoints. The data processing will be
//! done here calling methods from the `database` crate.

#![forbid(unsafe_code)]

pub mod error;
pub mod prelude;
pub mod users;
