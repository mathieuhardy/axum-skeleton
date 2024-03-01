use aide::openapi::{Info, OpenApi};
use axum::Extension;

use crate::prelude::*;

pub fn setup(router: ApiRouter) -> IntoMakeService<Router> {
    let mut api = OpenApi {
        info: Info {
            title: "Axum Skeleton OpenApi".to_string(),
            description: Some("OpenAPI documentation for this project".to_string()),
            version: "".to_string(), // TODO
            ..Info::default()
        },
        ..OpenApi::default()
    };

    router
        .finish_api(&mut api)
        .layer(Extension(api))
        .into_make_service()
}
