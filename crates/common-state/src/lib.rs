//! Common structure and utilities used by other crates regarding hexagonal architecture.

#![forbid(unsafe_code)]

mod state;

pub use state::{AppState, RedisPool};
