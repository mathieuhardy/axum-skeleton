//! This file contains all State related structures and functions. The state is
//! an struct passed along routes that will be called in order to share common
//! objects (e.g. the database(s) handle(s)).

use sqlx::postgres::PgPool;

/// Type used to manipulate a Redis database.
pub type RedisPool = bb8::Pool<bb8_redis::RedisConnectionManager>;

/// State structure passed along routes.
#[derive(Clone, Debug)]
pub struct AppState {
    /// PostgreSQL database handle.
    pub db: PgPool,

    /// Redis database handle.
    pub redis: RedisPool,
}

impl AppState {
    /// Creates a new AppState instance with default values.
    ///
    /// # Returns
    /// New instance of AppState.
    pub fn new(db: PgPool, redis: RedisPool) -> Self {
        Self { db, redis }
    }
}
