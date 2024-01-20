mod k8s;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::state::State;

pub async fn build() -> Router<State> {
    Router::new()
        .route("/", get(hello().await))
        .nest("/k8", k8s::build())
}

#[axum::debug_handler]
async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
