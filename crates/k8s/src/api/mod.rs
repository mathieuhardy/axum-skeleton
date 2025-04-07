//! APIs used by kubernetes to check the status of our application.

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

use common_core::AppState;

/// Builds an Axum router.
///
/// # Returns
/// An Axum router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/liveness", get(k8s_liveness))
        .route("/readiness", get(k8s_readiness))
        .route("/startup", get(k8s_startup))
}

/// Route for Kubernetes's liveness probe.
///
/// # Returns
/// HTTP status code.
async fn k8s_liveness() -> StatusCode {
    StatusCode::OK
}

/// Route for Kubernetes's readiness probe.
///
/// # Returns
/// HTTP status code.
async fn k8s_readiness() -> StatusCode {
    StatusCode::OK
}

/// Route for Kubernetes's startup probe.
///
/// # Returns
/// HTTP status code.
async fn k8s_startup() -> StatusCode {
    StatusCode::OK
}
