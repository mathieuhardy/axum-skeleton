//! This file contains everything needed for unit testing (i.e. creating a
//! server instance, etc).

#![forbid(unsafe_code)]
#![feature(async_closure)]

pub use axum::http::StatusCode;

use axum::body::Body;
use axum::extract::Request;
use axum::http::header::{HeaderName, HeaderValue, CONTENT_TYPE, COOKIE, SET_COOKIE};
use axum::http::response::Response;
use axum::http::Method;
use axum::Router;
use http_body_util::BodyExt;
use mime::Mime;
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::panic::{catch_unwind, UnwindSafe};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::util::ServiceExt;
use tracing::subscriber::DefaultGuard;

#[cfg(feature = "derives")]
pub use test_utils_derives::*;

use server::config::{Config, Environment};
use server::{app, axum};
use utils::filesystem::root_relative_path;

/// Structure used by the tests to make requests to the test server.
#[derive(Debug)]
pub struct TestClient {
    /// Database connection if needed for tests.
    pub db: PgPool,

    /// Router application to be tested.
    app: Router,

    /// Store cookie or not.
    pub cookie_store: bool,

    /// Cookie header value to be sent.
    cookie: Option<HeaderValue>,

    /// Guard to be kept during the tests for tracing.
    _subscriber_guard: DefaultGuard,
}

/// Structure used to build a HTTP request for the tests.
pub struct TestRequestBuilder<'a> {
    /// Application to be used.
    client: &'a mut TestClient,

    /// HTTP method.
    method: Method,

    /// URL to be requested.
    url: String,

    /// Body to be sent alongside the request.
    body: Body,

    /// Content type of the request.
    content_type: Option<Mime>,
}

impl TestRequestBuilder<'_> {
    /// Creates a new request builder with the JSON data provided.
    ///
    /// # Arguments
    /// * `data` - JSON data to be used.
    ///
    /// # Returns
    /// A new request builder (for chaining).
    pub fn json<T: Serialize>(self, data: &T) -> Self {
        Self {
            body: Body::from(serde_json::to_vec(data).unwrap()),
            content_type: Some(mime::APPLICATION_JSON),
            ..self
        }
    }

    /// Creates a new request builder with the form data provided.
    ///
    /// # Arguments
    /// * `data` - Form data to be used.
    ///
    /// # Returns
    pub fn form<K, V>(self, data: &[(K, V)]) -> Self
    where
        K: ToString + Display + Serialize,
        V: ToString + Display + Serialize,
    {
        let body: String = data
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join("&");

        Self {
            body: Body::from(body),
            content_type: Some(mime::APPLICATION_WWW_FORM_URLENCODED),
            ..self
        }
    }

    /// Sets the `cookie_store` value.
    ///
    /// # Arguments
    /// * `value` - Value to be set.
    ///
    ///
    /// # Returns
    /// A test response object.
    pub fn cookie_store(self, value: bool) -> Self {
        self.client.cookie_store = value;
        self
    }

    /// Sends the request represented by ths builder.
    /// The function consumes the self object.
    ///
    /// # Returns
    /// A test response object.
    pub async fn send(self) -> TestResponse {
        let mut builder = Request::builder()
            .method(self.method.clone())
            .uri(self.url.clone());

        if let Some(content_type) = &self.content_type {
            builder = builder.header(CONTENT_TYPE, content_type.as_ref());
        }

        if self.client.cookie_store {
            if let Some(cookie) = &self.client.cookie {
                builder = builder.header(COOKIE, cookie.clone());
            }
        }

        let response = self
            .client
            .app
            .clone()
            .oneshot(builder.body(self.body).unwrap())
            .await
            .unwrap();

        if self.client.cookie_store {
            if let Some(cookie) = response.headers().get::<HeaderName>(SET_COOKIE) {
                self.client.cookie = Some(cookie.clone());
            }
        }

        TestResponse {
            rc: response.status(),
            response,
        }
    }
}

/// Structure returned by a request for tests.
pub struct TestResponse {
    /// Status code.
    rc: StatusCode,

    /// Raw response.
    response: Response<Body>,
}

impl TestResponse {
    /// Gets the status code of the request.
    ///
    /// # Returns
    /// The status code of the execution of the request.
    pub fn status(&self) -> StatusCode {
        self.rc
    }

    /// Converts the body into JSON data. Consumes the self object.
    ///
    /// # Returns
    /// An object whose type is defined by the caller.
    pub async fn json<T>(self) -> T
    where
        T: for<'a> Deserialize<'a>,
    {
        let body = self
            .response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();

        let data: T = serde_json::from_slice(&body).unwrap();

        data
    }
}

impl TestClient {
    /// Creates a request builder with parameters.
    ///
    /// # Arguments
    /// * `method` - HTTP method used for the request.
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// The request builder.
    fn make_builder<T>(&mut self, method: Method, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        TestRequestBuilder {
            client: self,
            method,
            url: url.to_string(),
            body: Body::default(),
            content_type: None,
        }
    }

    /// Prepares a DELETE request.
    ///
    /// # Arguments
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// A request builder that can be enriched before sending.
    pub fn delete<T>(&mut self, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        self.make_builder(Method::DELETE, url)
    }

    /// Prepares a GET request.
    ///
    /// # Arguments
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// A request builder that can be enriched before sending.
    pub fn get<T>(&mut self, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        self.make_builder(Method::GET, url)
    }

    /// Prepares a POST request.
    ///
    /// # Arguments
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// A request builder that can be enriched before sending.
    pub fn post<T>(&mut self, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        self.make_builder(Method::POST, url)
    }

    /// Prepares a HEAD request.
    ///
    /// # Arguments
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// A request builder that can be enriched before sending.
    pub fn head<T>(&mut self, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        self.make_builder(Method::HEAD, url)
    }

    /// Prepares a PATCH request.
    ///
    /// # Arguments
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// A request builder that can be enriched before sending.
    pub fn patch<T>(&mut self, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        self.make_builder(Method::PATCH, url)
    }

    /// Prepares a PUT request.
    ///
    /// # Arguments
    /// * `url` - URL of the destination.
    ///
    /// # Returns
    /// A request builder that can be enriched before sending.
    pub fn put<T>(&mut self, url: T) -> TestRequestBuilder
    where
        T: ToString + Display,
    {
        self.make_builder(Method::PUT, url)
    }
}

/// Initialize a test server and returns a client used to test it.
///
/// # Returns
/// Result of TestClient.
pub async fn init_server() -> Result<TestClient, Box<dyn Error>> {
    dotenvy::dotenv()?;

    // Tracing
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_level(true)
        .with_target(false)
        .with_test_writer()
        .compact()
        .finish();

    let subscriber_guard = tracing::subscriber::set_default(subscriber);

    // Initialize database and server
    let db_env_variable = "DATABASE_URL_TEST";
    let config: Config = Environment::Testing.try_into()?;

    let db = initialize_database(db_env_variable).await?;
    let app = app(&config, Some(db_env_variable), None).await.unwrap();

    // Run custom test script to populate the database with fake data
    reset_fake_data(&db).await?;

    Ok(TestClient {
        db,
        app,
        cookie_store: false,
        cookie: None,
        _subscriber_guard: subscriber_guard,
    })
}

/// Initialize the database use in the application.
///
/// # Arguments
/// * `db_env_variable` - Name of the environment variable to use to access database.
///
/// # Returns
/// Postgres pool or an error.
async fn initialize_database(db_env_variable: &str) -> Result<PgPool, Box<dyn Error>> {
    let db_url = std::env::var(db_env_variable)?;

    let db = PgPoolOptions::new().connect(&db_url).await?;

    if !sqlx::Postgres::database_exists(&db_url).await? {
        sqlx::Postgres::create_database(&db_url).await?;
    }

    Ok(db)
}

/// Sets/resets fake data in database
///
/// # Arguments
/// * `db` - Database handle.
///
/// # Returns
/// Empty result.
pub async fn reset_fake_data(db: &PgPool) -> Result<(), Box<dyn Error>> {
    let test_script = root_relative_path("data/tests/populate.sql")?;

    let sql = std::fs::read_to_string(test_script)?;

    sqlx::query(&sql).execute(db).await?;

    Ok(())
}

/// Runs a test calling a setup function before the test and a teardown function
/// at the end.
///
/// # Arguments
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
    let body_result = catch_unwind(std::panic::AssertUnwindSafe(async || body(body_data).await));

    let teardown_result = catch_unwind(std::panic::AssertUnwindSafe(async || {
        teardown(teardown_data).await
    }));

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
