use std::error::Error;

#[tokio::main]
// TODO: Custom error
async fn main() -> Result<(), Box<dyn Error>> {
    // Load `.env` file
    dotenv::dotenv().ok();

    // Start Web server
    server::start().await?;

    Ok(())
}
