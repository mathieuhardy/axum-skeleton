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

pub(crate) mod prelude;
pub(crate) mod requests;

// Re-exports
pub use {sqlx, uuid};

// External crates
use sqlx::postgres::PgPoolOptions;

use prelude::*;

pub async fn initialize(db_env_variable: Option<&str>) -> Res<PgPool> {
    let db_url = std::env::var(db_env_variable.unwrap_or("DATABASE_URL")).map_err(Error::Env)?;

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
