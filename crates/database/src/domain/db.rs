//! Database generic types

use sqlx::postgres::PgPool;

/// PostgreSQL database handle.
#[derive(Clone, Debug)]
pub struct Db(pub PgPool);
