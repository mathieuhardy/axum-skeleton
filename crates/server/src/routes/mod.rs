#[cfg(feature = "k8s")]
mod k8s;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::state::State;

pub async fn build() -> Router<State> {
    let router = Router::new().route("/", get(hello().await));

    #[cfg(feature = "k8s")]
    let router = router.nest("/k8", k8s::build());

    router
}

#[axum::debug_handler]
async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
