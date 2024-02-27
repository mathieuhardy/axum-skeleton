//! This file is the entry point for the server application. It provides a
//! function to start it and some default handlers.

pub mod config;
pub(crate) mod cors;
pub(crate) mod error;
pub(crate) mod prelude;
pub(crate) mod routes;
pub(crate) mod state;
pub(crate) mod timeout;
pub(crate) mod tracing;
pub(crate) mod types;

pub use axum;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tokio::signal;

use crate::config::Config;
#[cfg(debug_assertions)]
#[cfg(feature = "sanity")]
use crate::config::Environment;
use crate::prelude::*;
use crate::tracing::tracing_layer;

/// Starts the server application.
///
/// # Returns
/// An empty Result.
pub async fn start(config: Option<crate::config::Config>) -> Res<()> {
    // Load configuration
    let config = match config {
        Some(config) => config,
        None => Config::new()?,
    };

    event!(Level::INFO, "📄 Configuration loaded");
    event!(Level::TRACE, "{:#?}", config);

    // Prepare application
    let app = app(&config, None).await?;

    // Create TCP listener
    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(&address).await.map_err(Error::Socket)?;

    // Start server
    event!(
        Level::INFO,
        "🚀 Listening on {}",
        listener.local_addr().map_err(Error::Socket)?
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(Error::Axum)?;

    Ok(())
}

/// Creates an Axum application that can be served.
///
/// # Arguments
/// * `config` - Configuration object.
///
/// # Returns
/// An Axum router instance.
pub async fn app(config: &Config, db_env_variable: Option<&str>) -> Res<Router> {
    // CORS layer
    let cors = cors::build(config);

    event!(Level::INFO, "🔒 CORS configured");
    event!(Level::TRACE, "{:#?}", cors);

    // Timeout
    let timeout = timeout::timeout_layer(config);

    event!(Level::INFO, "🕑 Timeout configured");

    // Sensitive layers
    let (sensitive_request_layer, sensitive_response_layer) = tracing::sensitive_headers_layers();

    // Request ID layers
    let (request_id_layer, propagate_request_id_layer) = tracing::request_id_layers();

    // Create Postgresql pool connection
    let db_url = std::env::var(db_env_variable.unwrap_or("DATABASE_URL")).map_err(Error::Env)?;

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await?;

    event!(Level::INFO, "🗃  Database initialized");

    let state = AppState::new(pool.clone());
    event!(Level::INFO, "📦 State configured");

    let mut router = Router::new()
        .fallback(handler_404)
        .nest("/", routes::build().await)
        .with_state(state);

    router = router
        .layer(cors)
        .layer(timeout)
        .layer(request_id_layer)
        .layer(sensitive_request_layer)
        .layer(tracing_layer())
        .layer(propagate_request_id_layer)
        .layer(sensitive_response_layer);

    #[cfg(debug_assertions)]
    #[cfg(feature = "sanity")]
    if Environment::Development.equals(&config.environment) {
        router = sanity::initialize(router).map_err(Error::Sanity)?;

        event!(Level::INFO, "🩺 Sanity installed");
    }

    Ok(router)
}

/// Default handler for NotFound errors.
///
/// # Returns
/// Anything that can be converted to a Response.
async fn handler_404() -> impl IntoResponse {
    event!(Level::WARN, "Unhandled route");

    StatusCode::NOT_FOUND
}

/// Default handler for signals (CTRL-C, terminate, etc).
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install Terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { bye() },
        _ = terminate => { bye() },
    }
}

/// Function called at the stopping of the server.
fn bye() {
    event!(Level::INFO, "👋 Bye bye");
}
