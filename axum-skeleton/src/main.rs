//! Axum Skeleton is a proof-of-concept of a backend server using Axum. It can be used as a starter
//! kit. The idea is to provide out-of-the-box almost all functionnalities and configuration that
//! you can encounter when implementing a web server application.

use std::error::Error;

#[cfg(feature = "jemalloc")]
use jemallocator::Jemalloc;

/// Global variable used by the jemalloc allocator.
#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

/// Entry point of the backend application. It loads environment variables,
/// initializes the logging system and starts the server.
///
/// # Returns
/// Result with generic error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_level(true)
        .with_target(false)
        .compact()
        .try_init()?;

    server::start(None).await?;

    Ok(())
}
