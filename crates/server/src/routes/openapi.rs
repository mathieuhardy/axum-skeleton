use aide::openapi::OpenApi;
use axum::Extension;

use crate::prelude::*;

/// Builds a router for the OpenAPI.
///
/// # Returns
/// An Axum router.
pub fn build() -> ApiRouter<AppState> {
    ApiRouter::new().route("/", get(serve_api))
}

// TODO: document
// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
