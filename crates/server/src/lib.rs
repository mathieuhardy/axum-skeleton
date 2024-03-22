//! This file is the entry point for the server application. It provides a
//! function to start it and some default handlers.

#![forbid(unsafe_code)]

pub mod config;

pub(crate) mod auth;
pub(crate) mod cors;
pub(crate) mod error;
pub(crate) mod prelude;
pub(crate) mod routes;
pub(crate) mod state;
pub(crate) mod timeout;
pub(crate) mod tracing;
pub(crate) mod types;

pub use axum;

use axum::response::IntoResponse;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeFile;

use crate::config::Config;
#[cfg(debug_assertions)]
#[cfg(feature = "sanity")]
use crate::config::Environment;
use crate::prelude::*;
use crate::tracing::tracing_layer;
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
    // Database configuration
    database::password::set_checks(utils::password::Checks {
        digit: config.password.pattern.digit,
        lowercase: config.password.pattern.lowercase,
        uppercase: config.password.pattern.uppercase,
        special: config.password.pattern.special,
        spaces: config.password.pattern.spaces,
        min_length: config.password.pattern.min_length,
        max_length: config.password.pattern.max_length,
    });

    // Create Postgresql pool connection
    let pool = database::initialize(db_env_variable).await?;
    event!(Level::INFO, "🗃  Database initialized");

    // CORS layer
    let cors = cors::build(config);

    event!(Level::INFO, "🔑 CORS configured");
    event!(Level::TRACE, "{:#?}", cors);

    // Timeout
    let timeout = timeout::timeout_layer(config);

    event!(Level::INFO, "⏰ Timeout configured");

    // Authentication layer
    let authentication = auth::authentication_layer(&pool);

    // Sensitive layers
    let (sensitive_request_layer, sensitive_response_layer) = tracing::sensitive_headers_layers();

    // Request ID layers
    let (request_id_layer, propagate_request_id_layer) = tracing::request_id_layers();

    // State shared between handlers
    let state = AppState::new(pool.clone());
    event!(Level::INFO, "📦 State configured");

    // Create router
    let mut router = Router::new()
        .fallback(handler_404)
        .nest("/", routes::build().await)
        .with_state(state);

    router = setup_favicon(router)?;

    router = router
        .layer(cors)
        .layer(timeout)
        .layer(CompressionLayer::new())
        .layer(authentication)
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
