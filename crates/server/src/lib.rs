mod config;
mod cors;
mod error;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
//use sqlx::postgres::PgPoolOptions;

//use database::models::users::User;

use crate::config::Config;
use crate::error::Error;

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

    // Start server
    let cors = cors::create(&config);

    log::info!("🔒 CORS configured");
    log::trace!("{:#?}", cors);

    let app = Router::new().route("/", get(handler)).layer(cors);

    log::info!(
        "🚀 Listening on {}",
        listener.local_addr().map_err(Error::Socket)?
    );

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
