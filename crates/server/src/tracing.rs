use axum::extract::Request;
use axum::http::header::HeaderValue;
use axum::http::{header, HeaderName};
use std::sync::Arc;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::sensitive_headers::{
    SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer,
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use uuid::Uuid;

use crate::prelude::*;

#[derive(Clone, Default)]
pub struct Id;

pub fn tracing_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

pub fn sensitive_headers_layers() -> (
    SetSensitiveRequestHeadersLayer,
    SetSensitiveResponseHeadersLayer,
) {
    let headers: Arc<[_]> = Arc::new([
        header::AUTHORIZATION,
        header::PROXY_AUTHORIZATION,
        header::COOKIE,
        header::SET_COOKIE,
    ]);

    let request_layer = SetSensitiveRequestHeadersLayer::from_shared(Arc::clone(&headers));
    let response_layer = SetSensitiveResponseHeadersLayer::from_shared(headers);

    (request_layer, response_layer)
}

impl MakeRequestId for Id {
    fn make_request_id<T>(&mut self, _request: &Request<T>) -> Option<RequestId> {
        let uuid = Uuid::new_v4().into_bytes();

        match HeaderValue::from_bytes(&uuid) {
            Ok(header) => Some(RequestId::new(header)),
            Err(_) => None,
        }
    }
}

pub fn request_id_layers() -> (SetRequestIdLayer<Id>, PropagateRequestIdLayer) {
    let x_request_id = HeaderName::from_static("x-request-id");

    (
        SetRequestIdLayer::new(x_request_id.clone(), Id),
        PropagateRequestIdLayer::new(x_request_id),
    )
}
