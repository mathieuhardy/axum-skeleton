//! The `database`'s crate gathers of database related utilities such as:
//!
//! - Initialization of the connection pool to the database.
//! - Migrations.
//! - Extractors used to access database in endpoints.

#![forbid(unsafe_code)]

// Modules
mod domain;
mod extractor;
mod prelude;

// Exports
pub use domain::db::{initialize, Db, SharedDb};
pub use domain::error::Error;

// Re-exports
pub use common_state::RedisPool;
