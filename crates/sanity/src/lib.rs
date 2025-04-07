//! This file is the entry point for the sanity dashboard. It provides a
//! function to insert it into the router.

#![forbid(unsafe_code)]

mod api;
mod domain;
mod prelude;

pub use api::router;
pub use domain::error::Error;
