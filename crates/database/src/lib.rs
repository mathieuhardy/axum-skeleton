//! The `database`'s crate gathers of database related utilities:
//!
//! - models: structures that matches data returned from queries an matches the tables.
//! - scripts: SQL raw scripts used in the crate for queries.

#![forbid(unsafe_code)]
#![feature(box_into_inner)]

pub mod error;
pub mod models;
pub mod password;
pub mod traits;

pub(crate) mod prelude;
pub(crate) mod requests;

// Re-exports
pub use {sqlx, uuid};

// External crates
use sqlx::postgres::PgPoolOptions;

use crate::prelude::*;

/// Initialize the database connection and run migrations.
///
/// # Arguments
/// * `db_env_variable` - Environment variable used to get the URL of the database.
///
/// # Returns
/// A result with the PostgresSQL pool.
pub async fn initialize(db_env_variable: Option<&str>) -> Res<PgPool> {
    let db_url = std::env::var(db_env_variable.unwrap_or("DATABASE_URL")).map_err(Error::Env)?;

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
