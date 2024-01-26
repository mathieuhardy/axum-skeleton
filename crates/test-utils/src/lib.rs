//! This file contains everything needed for unit testing (i.e. creating a
//! server instance, etc).

#![feature(async_closure)]

pub use reqwest::{Client, RequestBuilder, StatusCode};

use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::net::SocketAddr;
use std::panic::{catch_unwind, UnwindSafe};
use std::string::ToString;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[cfg(feature = "derives")]
pub use test_utils_derives::*;

use server::config::{Config, Environment};
use server::{app, axum};
use utils::filesystem::root_relative_path;

/// Structure used by the tests to make requests to the test server.
#[derive(Debug)]
pub struct TestClient {
    /// Client used to send requests.
    pub client: Client,

    /// Base address prefixed to every URL passed.
    pub address: SocketAddr,

    /// Database connection if needed for tests.
    pub db: PgPool,
}

impl TestClient {
    /// Builds an URL from base address and relative URL provided.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// The string of the full URL.
    fn build_url<T: ToString + Display>(&self, url: T) -> String {
        format!("http://{}{url}", self.address)
    }

    /// Sends a DELETE request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder that can be enriched before sending.
    pub fn delete<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.delete(self.build_url(url))
    }

    /// Sends a GET request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder that can be enriched before sending.
    pub fn get<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.get(self.build_url(url))
    }

    /// Sends a HEAD request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder that can be enriched before sending.
    pub fn head<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.head(self.build_url(url))
    }

    /// Sends a PATCH request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder that can be enriched before sending.
    pub fn patch<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.patch(self.build_url(url))
    }

    /// Sends a POST request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder that can be enriched before sending.
    pub fn post<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.post(self.build_url(url))
    }

    /// Sends a PUT request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder that can be enriched before sending.
    pub fn put<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.put(self.build_url(url))
    }
}

/// Initialize a test server and returns a client used to test it.
///
/// # Returns:
/// Result of TestClient.
pub async fn init_server() -> Result<TestClient, Box<dyn Error>> {
    dotenv::dotenv()?;

    let config: Config = Environment::Testing.try_into()?;

    // Configure server
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await?;

    let address = listener.local_addr()?;

    // TODO: get logs from server
    tokio::spawn(async move {
        let app = app(&config).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // Configure connection to the database
    let db = initialize_database().await?;

    Ok(TestClient {
        client: reqwest::Client::new(),
        address,
        db,
    })
}

async fn initialize_database() -> Result<PgPool, Box<dyn Error>> {
    let db_url = std::env::var("DATABASE_URL_TEST")?;

    if sqlx::Postgres::database_exists(&db_url).await? {
        sqlx::Postgres::drop_database(&db_url).await?;
    }

    if !sqlx::Postgres::database_exists(&db_url).await? {
        sqlx::Postgres::create_database(&db_url).await?;
    }

    // Migrations
    let migrations_dir = root_relative_path("migrations")?;

    let db = PgPoolOptions::new().connect(&db_url).await?;

    sqlx::migrate::Migrator::new(migrations_dir)
        .await?
        .run(&db)
        .await?;

    // Run custom test script to populate
    let test_script = root_relative_path("data/tests/populate.sql")?;

    let sql = std::fs::read_to_string(test_script)?;

    sqlx::query(&sql).execute(&db).await?;

    Ok(db)
}

/// Runs a test calling a setup function before the test and a teardown function
/// at the end.
///
/// # Arguments:
/// * `setup` - Setup function called before the body.
/// * `body` - Body function of the test.
/// * `teardown` - Teardown function called after the body.
pub async fn run_test<Setup, Body, Teardown, Data, SetupReturn, BodyReturn, TeardownReturn>(
    setup: Setup,
    body: Body,
    teardown: Teardown,
) where
    Setup: FnOnce() -> SetupReturn + UnwindSafe,
    SetupReturn: Future<Output = Data>,
    Body: FnOnce(Arc<Mutex<Data>>) -> BodyReturn + UnwindSafe,
    BodyReturn: Future<Output = ()>,
    Teardown: FnOnce(Arc<Mutex<Data>>) -> TeardownReturn + UnwindSafe,
    TeardownReturn: Future<Output = ()>,
{
    // Call setup and check result
    let setup_result = catch_unwind(async || setup().await);
    assert!(setup_result.is_ok());

    // Prepare data for the next calls
    let body_data = Arc::new(Mutex::new(setup_result.unwrap().await));
    let teardown_data = body_data.clone();

    // Call body and teardown without checking errors (we want to be sure the teardown is always
    // called)
    let body_result = catch_unwind(async || body(body_data).await);
    let teardown_result = catch_unwind(async || teardown(teardown_data).await);

    // Checks final results in order
    assert!(body_result.is_ok());
    body_result.unwrap().await;

    assert!(teardown_result.is_ok());
    teardown_result.unwrap().await;
}

/// Dummy function use as placeholder when calling the `run_test` function.
pub async fn no_setup() {}

/// Dummy function use as placeholder when calling the `run_test` function.
pub async fn no_body<T>(_: T) {}

/// Dummy function use as placeholder when calling the `run_test` function.
pub async fn no_teardown<T>(_: T) {}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_failure() {
        panic!();
    }

    async fn body_failure<T>(_: T) {
        panic!();
    }

    async fn teardown_failure<T>(_: T) {
        panic!();
    }

    mod nominal {
        use super::*;

        #[tokio::test]
        async fn setup() {
            run_test(no_setup, no_body, no_teardown).await
        }
    }

    mod failures {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn setup() {
            run_test(setup_failure, no_body, no_teardown).await
        }

        #[tokio::test]
        #[should_panic]
        async fn body() {
            run_test(no_setup, body_failure, no_teardown).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn teardown() {
            run_test(no_setup, no_body, teardown_failure).await;
        }
    }
}
