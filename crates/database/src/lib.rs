//! The `database`'s crate gathers of database related utilities:
//!
//! - models: structures that matches data returned from queries an matches the tables.
//! - scripts: SQL raw scripts used in the crate for queries.

#![feature(box_into_inner)]
#![feature(async_fn_in_trait)]
#![feature(result_option_inspect)]

pub mod error;
pub mod models;
pub mod traits;

// External crates
pub use {sqlx, uuid};

pub(crate) mod prelude;
pub(crate) mod requests;
