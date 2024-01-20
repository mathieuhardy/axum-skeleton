//! This file is the entry point for the server application. It provides a
//! function to start it and some default handlers.

mod config;
mod cors;
mod error;
mod routes;
mod state;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tokio::signal;
//use sqlx::postgres::PgPoolOptions;

//use database::models::users::User;

use crate::config::Config;
use crate::error::Error;
use crate::state::State;

/// Starts the server application.
///
/// # Returns
/// An empty Result.
pub async fn start() -> Result<(), Error> {
    let config = Config::new()?;

    log::info!("📄 Configuration loaded");
    log::trace!("{:#?}", config);

    //let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");

    //let dbpool = PgPoolOptions::new()
    //.max_connections(8)
    //.connect(&db_url)
    //.await?;

    //let user = sqlx::query_as::<_, User>(r#"select * from users u where u.email = $1"#)
    //.bind("mhardy2008@gmail.com")
    //.fetch_one(&dbpool)
    //.await
    //.unwrap();

    //log::debug!("{:#?}", user);

    // Start TCP listener
    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(Error::Socket)?;

    // Prepare application
    let cors = cors::build(&config);

    log::info!("🔒 CORS configured");
    log::trace!("{:#?}", cors);

    let state = State::new();
    log::info!("📦 State configured");

    let app = Router::new()
        .nest("/", routes::build().await)
        .with_state(state)
        .layer(cors);

    let app = app.fallback(handler_404);

    // Start server
    log::info!(
        "🚀 Listening on {}",
        listener.local_addr().map_err(Error::Socket)?
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
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
