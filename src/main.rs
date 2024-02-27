//! Axum Skeleton is a proof-of-concept of a backend server using Axum. It can be used as a starter
//! kit. The idea is to provide out-of-the-box almost all functionnalities and configuration that
//! you can encounter when implementing a web server application.
//!
//! Here's the list of crates implemented and used in this project:
//!
//! - **[Database](../database/index.html)**: All database related utilities.
//! - **[Database derive macros](../database_derives/index.html)**: All derive macros used by the
//! `Database`'s crate.
//! - **[Sanity](../sanity/index.html)**: Files used to display a sanity dashboard of the project.
//! - **[Server](../server/index.html)**: Web server routes and configurations.
//! - **[Test utils](../test_utils/index.html)**: Unit tests utilities.
//! - **[Utils](../utils/index.html)**: Global project utilities.

use std::error::Error;

/// Entry point of the backend application. It loads environment variables,
/// initializes the logging system and starts the server.
///
/// # Returns
/// Result with generic error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Load `.env` file
    dotenv::dotenv()?;

    // Tracing configuration
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_level(true)
        .with_target(false)
        .compact()
        .try_init()?;

    // Start Web server
    server::start(None).await?;

    Ok(())
}
