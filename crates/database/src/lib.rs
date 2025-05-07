//! The `database`'s crate gathers of database related utilities such as:
//!
//! - Initialization of the connection pool to the database.
//! - Migrations.
//! - Extractors used to access database in endpoints.

#![forbid(unsafe_code)]

// Modules
pub mod extractor;

mod db;
mod error;
mod prelude;

// Exports
pub use db::initialize;
pub use error::Error;

// Re-exports
pub use common_state::RedisPool;
