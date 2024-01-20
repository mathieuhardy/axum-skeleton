//! This file contains all routes dedicated to Kubernetes.

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

use crate::state::State;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<State> {
    Router::new()
        .route("/liveness", get(liveness))
        .route("/readiness", get(readiness))
        .route("/startup", get(startup))
}

/// Route for Kubernetes's liveness probe.
///
/// # Returns
/// HTTP status code.
async fn liveness() -> StatusCode {
    StatusCode::OK
}

/// Route for Kubernetes's readiness probe.
///
/// # Returns
/// HTTP status code.
async fn readiness() -> StatusCode {
    StatusCode::OK
}

/// Route for Kubernetes's startup probe.
///
/// # Returns
/// HTTP status code.
async fn startup() -> StatusCode {
    StatusCode::OK
}
