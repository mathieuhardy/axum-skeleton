//! This file contains everything needed for unit testing (i.e. creating a
//! server instance, etc).

pub use reqwest::{Client, RequestBuilder, StatusCode};

use std::error::Error;
use std::fmt::Display;
use std::net::SocketAddr;
use std::string::ToString;
use tokio::net::TcpListener;

use server::config::{Config, Environment};
use server::{app, axum};

/// Structure used by the tests to make requests to the test server.
pub struct TestClient {
    /// Client used to send requests.
    pub client: Client,

    /// Base address prefixed to every URL passed.
    pub address: SocketAddr,
}

impl TestClient {
    /// Sends a DELETE request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder htat can be enriched before sending.
    pub fn delete<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.delete(format!("http://{}{url}", self.address))
    }

    /// Sends a GET request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder htat can be enriched before sending.
    pub fn get<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.get(format!("http://{}{url}", self.address))
    }

    /// Sends a HEAD request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder htat can be enriched before sending.
    pub fn head<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.head(format!("http://{}{url}", self.address))
    }

    /// Sends a PATCH request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder htat can be enriched before sending.
    pub fn patch<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.patch(format!("http://{}{url}", self.address))
    }

    /// Sends a POST request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder htat can be enriched before sending.
    pub fn post<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.post(format!("http://{}{url}", self.address))
    }

    /// Sends a PUT request to the test server.
    ///
    /// # Arguments:
    /// * `url` - Relative URL of the destination.
    ///
    /// # Returns:
    /// A request builder htat can be enriched before sending.
    pub fn put<T: ToString + Display>(&self, url: T) -> RequestBuilder {
        self.client.put(format!("http://{}{url}", self.address))
    }
}

/// Initialize a test server and returns a client used to test it.
///
/// # Returns:
/// Result of TestClient.
pub async fn init_server() -> Result<TestClient, Box<dyn Error>> {
    let config: Config = Environment::Testing.try_into()?;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await?;

    let address = listener.local_addr()?;

    // TODO: get logs from server
    tokio::spawn(async move {
        let app = app(&config).await;
        axum::serve(listener, app).await.unwrap();
    });

    Ok(TestClient {
        client: reqwest::Client::new(),
        address,
    })
}
