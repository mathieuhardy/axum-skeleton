//! This file contains only the entry point called to run the backend
//! application.

use std::error::Error;

/// Entry point of the backend application. It loads environment variables,
/// initializes the logging system and starts the server.
///
/// # Returns
/// Result with generic error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load `.env` file
    dotenv::dotenv()?;

    // Initialize logging system
    env_logger::try_init()?;

    // Start Web server
    server::start().await?;

    Ok(())
}
