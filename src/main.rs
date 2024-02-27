use std::error::Error;

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
