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

    Ok(TestClient {
        client: reqwest::Client::new(),
        address,
    })
}
