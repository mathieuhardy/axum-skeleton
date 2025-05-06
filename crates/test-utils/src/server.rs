//! Utilities used to fire a server when testing endpoints of the application.

use axum::body::Body;
use axum::extract::Request;
use axum::http::header::{HeaderName, HeaderValue, CONTENT_TYPE, COOKIE, SET_COOKIE};
use axum::http::response::Response;
use axum::http::{Method, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use mime::Mime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::error::Error;
use std::fmt::Display;
use tower::util::ServiceExt;
use tracing::subscriber::DefaultGuard;

use auth::AuthCredentials;
use security::password::Password;
use server::app;
use server::config::{Config, Environment};

use crate::database::initialize_database;

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

    /// Login user `AuthUser`.
    ///
    /// # Arguments
    /// * `user` - User to be logged as.
    pub async fn login(&mut self, email: &str, password: &Password) {
        self.post("/login")
            .cookie_store(true)
            .json(&AuthCredentials {
                email: email.to_string(),
                password: password.to_owned(),
            })
            .send()
            .await;
    }
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

    Ok(TestClient {
        db,
        app,
        cookie_store: false,
        cookie: None,
        _subscriber_guard: subscriber_guard,
    })
}
