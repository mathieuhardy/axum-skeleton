//! This file is the entry point for the server application. It provides a
//! function to start it and some default handlers.

pub mod config;
pub mod cors;
pub mod error;
pub mod prelude;
pub mod routes;
pub mod state;
pub mod types;

pub use axum;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tokio::signal;

use crate::config::{Config, Environment};
use crate::prelude::*;

/// Starts the server application.
///
/// # Returns
/// An empty Result.
pub async fn start(config: Option<crate::config::Config>) -> Res<()> {
    let config = match config {
        Some(config) => config,
        None => Config::new()?,
    };

    log::info!("📄 Configuration loaded");
    log::trace!("{:#?}", config);

    // Prepare application
    let app = app(&config).await?;

    // Create TCP listener
    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(&address).await.map_err(Error::Socket)?;

    // Start server
    log::info!(
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
pub async fn app(config: &Config) -> Res<Router> {
    // Create CORS
    let cors = cors::build(config);

    log::info!("🔒 CORS configured");
    log::trace!("{:#?}", cors);

    // Create Postgresql pool connection
    // TODO: enable dedicated database
    //#[cfg(not(test))]
    let db_url = std::env::var("DATABASE_URL").map_err(Error::Env)?;

    //#[cfg(test)]
    //let db_url = std::env::var("DATABASE_URL_TEST").map_err(Error::Env)?;

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await?;

    log::info!("🗃  Database initialized");

    let state = AppState::new(pool.clone());
    log::info!("📦 State configured");

    let mut router = Router::new()
        .fallback(handler_404)
        .nest("/", routes::build().await)
        .with_state(state)
        .layer(cors);

    #[cfg(debug_assertions)]
    #[cfg(feature = "sanity")]
    if Environment::Development.equals(&config.environment) {
        router = sanity::initialize(router).map_err(Error::Sanity)?;

        log::info!("🩺 Sanity installed");
    }

    Ok(router)
}

/// Default handler for NotFound errors.
///
/// # Returns
/// Anything that can be converted to a Response.
async fn handler_404() -> impl IntoResponse {
    log::warn!("Unhandled route");

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
    log::info!("👋 Bye bye");
}
