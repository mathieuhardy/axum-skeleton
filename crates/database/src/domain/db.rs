//! Database generic types

use bb8_redis::RedisConnectionManager;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::TryStreamExt;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::{ConnectOptions, Database, Describe, Either, Execute, Executor, Postgres, Transaction};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::log::LevelFilter;

use common_state::RedisPool;

use crate::prelude::*;

// Alias for a shared database handle wrapped in an `Arc<Mutex<_>>`.
pub type SharedDb = Arc<Mutex<Db>>;

/// PostgreSQL database handle.
#[derive(Clone, Debug)]
pub struct Db {
    /// Database pool.
    pool: PgPool,

    /// Optional transaction. If set then this will be used instead of the pool.
    tx: Option<Arc<Mutex<Transaction<'static, Postgres>>>>,
}

impl Db {
    /// Creates a new Db instance.
    ///
    /// # Arguments
    /// * `pool` - A `PgPool` instance representing the connection pool to the PostgreSQL database.
    ///
    /// # Returns
    /// A new instance of `Db` initialized with the provided pool and no active transaction.
    pub fn new(pool: PgPool) -> Self {
        Self { pool, tx: None }
    }

    /// Converts the `Db` instance into a shared instance wrapped in an `Arc<Mutex<_>>`.
    ///
    /// # Returns
    /// An `Arc<Mutex<Db>>` that can be shared across threads.
    pub fn into_shared(self) -> SharedDb {
        Arc::new(Mutex::new(self))
    }

    /// Starts a new transaction.
    ///
    /// # Returns
    /// An `ApiResult` indicating success or failure.
    pub async fn start_transaction(&mut self) -> ApiResult<()> {
        let tx = self.pool.begin().await?;

        self.tx = Some(Arc::new(Mutex::new(tx)));

        Ok(())
    }

    /// Commits the current transaction, if any.
    ///
    /// # Returns
    /// An `ApiResult` indicating success or failure.
    pub async fn commit_transaction(&mut self) -> ApiResult<()> {
        if let Some(tx) = self.tx.take() {
            if let Some(tx) = Arc::into_inner(tx) {
                let tx = tx.into_inner();
                tx.commit().await?;
            }
        }

        Ok(())
    }
}

impl<'c> Executor<'c> for Db {
    type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            Either<<Self::Database as Database>::QueryResult, <Self::Database as Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        E: 'q + Execute<'q, Self::Database>,
    {
        if let Some(tx) = self.tx {
            Box::pin(async_stream::try_stream! {
                let mut guard = tx.lock().await;
                let mut stream = guard.fetch_many(query);
                while let Some(item) = stream.try_next().await? {
                    yield item;
                }
            })
        } else {
            self.pool.fetch_many(query)
        }
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as Database>::Row>, sqlx::Error>>
    where
        E: 'q + Execute<'q, Self::Database>,
    {
        if let Some(tx) = self.tx {
            Box::pin(async move {
                let mut tx = tx.lock().await;
                tx.fetch_optional(query).await
            })
        } else {
            self.pool.fetch_optional(query)
        }
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as Database>::Statement<'q>, sqlx::Error>> {
        if let Some(tx) = self.tx {
            Box::pin(async move {
                let mut tx = tx.lock().await;
                tx.prepare_with(sql, parameters).await
            })
        } else {
            self.pool.prepare_with(sql, parameters)
        }
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<Describe<Self::Database>, sqlx::Error>> {
        if let Some(tx) = self.tx {
            Box::pin(async move {
                let mut tx = tx.lock().await;
                tx.describe(sql).await
            })
        } else {
            self.pool.describe(sql)
        }
    }
}

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

    let options = PgConnectOptions::from_str(&db_url)?
        .log_statements(LevelFilter::Off)
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));

    let pg_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect_with(options)
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
