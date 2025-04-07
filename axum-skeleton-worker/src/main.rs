//! Axum Skeleton job worker.
//!
//! The worker checks for pending jobs and process them.

use std::error::Error;

/// Entry point of the job worker.
///
/// # Returns
/// Result with generic error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    Ok(())
}
