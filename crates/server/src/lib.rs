//! This file is the entry point for the server application. It provides a
//! function to start it and some default handlers.

#![forbid(unsafe_code)]

pub mod config;
pub mod layers;

pub(crate) mod error;
pub(crate) mod extractors;
pub(crate) mod prelude;
pub(crate) mod routes;
pub(crate) mod state;

#[cfg(test)]
mod tests;

pub use axum;

use tokio::net::TcpListener;
use tokio::signal;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeFile;

use crate::config::Config;
#[cfg(debug_assertions)]
#[cfg(feature = "sanity")]
use crate::config::Environment;
use crate::layers::*;
use crate::prelude::*;
use utils::filesystem::{relative_path, root_relative_path};

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
    let app = app(&config, None, None).await?;

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
/// * `db_env_variable` - Environment variable used to get the URL of the SQL database.
/// * `redis_env_variable` - Environment variable used to get the URL of the Redis database.
///
/// # Returns
/// An Axum router instance.
pub async fn app(
    config: &Config,
    db_env_variable: Option<&str>,
    redis_env_variable: Option<&str>,
) -> Res<Router> {
    // Database configuration
    actions::validators::password::set_checks(utils::password::Checks {
        digit: config.password.pattern.digit,
        lowercase: config.password.pattern.lowercase,
        uppercase: config.password.pattern.uppercase,
        special: config.password.pattern.special,
        spaces: config.password.pattern.spaces,
        min_length: config.password.pattern.min_length,
        max_length: config.password.pattern.max_length,
    });

    // Create Postgresql pool connection
    let (pg_pool, redis_pool) = database::initialize(db_env_variable, redis_env_variable).await?;
    event!(Level::INFO, "🗃  Database initialized");

    // CORS layer
    let cors = layers::cors::build(config);

    event!(Level::INFO, "🔑 CORS configured");
    event!(Level::TRACE, "{:#?}", cors);

    // Timeout
    let timeout = layers::timeout::timeout_layer(config);

    event!(Level::INFO, "⏰ Timeout configured");

    // Compression
    let compression_layer = CompressionLayer::new();

    event!(Level::INFO, "🔻 Compression enabled");

    // Authentication layer
    let authentication = auth::authentication_layer(config, &pg_pool);

    event!(Level::INFO, "👤 Authentication enabled");

    // Sensitive layers
    let (sensitive_request_layer, sensitive_response_layer) = tracing::sensitive_headers_layers();

    // Request ID layers
    let (request_id_layer, propagate_request_id_layer) = tracing::request_id_layers();

    // Tracing
    let tracing_layer = layers::tracing::tracing_layer();

    // State shared between handlers
    let state = AppState::new(pg_pool, redis_pool);

    event!(Level::INFO, "📦 State configured");

    // Create router
    let mut router = Router::new()
        .fallback(handler_404)
        .nest("/", routes::build())
        .with_state(state);

    router = setup_favicon(router)?;

    router = router
        .layer(cors)
        .layer(timeout)
        .layer(compression_layer)
        .layer(authentication)
        .layer(request_id_layer)
        .layer(sensitive_request_layer)
        .layer(tracing_layer)
        .layer(propagate_request_id_layer)
        .layer(sensitive_response_layer);

    #[cfg(debug_assertions)]
    #[cfg(feature = "sanity")]
    if Environment::Development.equals(&config.environment) {
        router = sanity::initialize(router).map_err(Error::Sanity)?;

        event!(Level::INFO, "🩺 Sanity enabled");
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

/// Configures the favicon.ico handler.
///
/// # Arguments
/// * `router` - Router to be populated with new route(s).
///
/// # Returns
/// New router handle.
fn setup_favicon(router: Router) -> Res<Router> {
    let icon_path = relative_path("data/images/favicon.ico")
        .or(root_relative_path("crates/server/data/images/favicon.ico"))?;

    event!(Level::INFO, "🟣 Favicon setup");

    Ok(router.nest_service("/favicon.ico", ServeFile::new(icon_path)))
}
