//! Databases connections initialization

use bb8_redis::RedisConnectionManager;
use sqlx::postgres::{PgPool, PgPoolOptions};

use common_core::RedisPool;

use crate::prelude::*;

/// Initialize the database connection and run migrations.
///
/// # Arguments
/// * `db_env_variable` - Environment variable used to get the URL of the SQL database.
/// * `redis_env_variable` - Environment variable used to get the URL of the Redis database.
///
/// # Returns
/// A result with the PostgresSQL pool and the Redis pool.
pub async fn initialize(
    db_env_variable: Option<&str>,
    redis_env_variable: Option<&str>,
) -> ApiResult<(PgPool, RedisPool)> {
    // PostgresSQL
    let db_url = std::env::var(db_env_variable.unwrap_or("DATABASE_URL")).map_err(Error::Env)?;

    let pg_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&pg_pool).await?;

    event!(Level::DEBUG, "PostgresSQL initialized");

    // Redis
    let db_url = std::env::var(redis_env_variable.unwrap_or("REDIS_URL")).map_err(Error::Env)?;

    let manager = RedisConnectionManager::new(db_url)?;
    let redis_pool = bb8::Pool::builder().build(manager).await?;

    event!(Level::DEBUG, "Redis initialized");

    Ok((pg_pool, redis_pool))
}
