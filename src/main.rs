//! This file contains only the entry point called to run the backend
//! application.

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
