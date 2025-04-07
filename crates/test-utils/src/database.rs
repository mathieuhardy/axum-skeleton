//! Utilities used to initialiaze a connection to the database for testing purpose.

use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;

/// Initialize the database use in the application.
///
/// # Arguments
/// * `db_env_variable` - Name of the environment variable to use to access database.
///
/// # Returns
/// Postgres pool or an error.
pub async fn setup_test_database() -> Result<PgPool, Box<dyn Error>> {
    dotenvy::dotenv()?;

    initialize_database("DATABASE_URL_TEST").await
}

/// Initialize the database use in the application.
///
/// # Arguments
/// * `db_env_variable` - Name of the environment variable to use to access database.
///
/// # Returns
/// Postgres pool or an error.
pub async fn initialize_database(db_env_variable: &str) -> Result<PgPool, Box<dyn Error>> {
    let db_url = std::env::var(db_env_variable)?;

    let db = PgPoolOptions::new().connect(&db_url).await?;

    if !sqlx::Postgres::database_exists(&db_url).await? {
        sqlx::Postgres::create_database(&db_url).await?;
    }

    Ok(db)
}
