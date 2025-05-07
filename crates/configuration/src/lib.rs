//! This file contains all structures and functions used to handle the server
//! configuration. The configuration structure may be passed along all routes.

mod config;
mod error;

pub use config::{Config, Environment};
pub use error::Error;
