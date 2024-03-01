//! This file contains all routes dedicated to Kubernetes.

use crate::prelude::*;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/liveness", get(liveness))
        .api_route("/readiness", get(readiness))
        .api_route("/startup", get(startup))
}

/// Route for Kubernetes's liveness probe.
///
/// # Returns
/// HTTP status code.
async fn liveness() -> impl IntoApiResponse {
    StatusCode::OK
}

/// Route for Kubernetes's readiness probe.
///
/// # Returns
/// HTTP status code.
async fn readiness() -> impl IntoApiResponse {
    StatusCode::OK
}

/// Route for Kubernetes's startup probe.
///
/// # Returns
/// HTTP status code.
async fn startup() -> impl IntoApiResponse {
    StatusCode::OK
}
