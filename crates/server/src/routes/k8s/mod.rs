use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

use crate::state::State;

pub fn build() -> Router<State> {
    Router::new()
        .route("/liveness", get(liveness))
        .route("/readiness", get(readiness))
        .route("/startup", get(startup))
}

async fn liveness() -> StatusCode {
    StatusCode::OK
}

async fn readiness() -> StatusCode {
    StatusCode::OK
}

async fn startup() -> StatusCode {
    StatusCode::OK
}
