mod error;
mod settings;

use axum::response::Html;
use axum::routing::get;

use axum::Router;
use sqlx::postgres::PgPoolOptions;

use database::models::users::User;

use crate::error::Error;
use crate::settings::Settings;

pub async fn start() -> Result<(), Error> {
    let _settings = Settings::new()?;

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");

    let dbpool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await?;

    let user = sqlx::query_as::<_, User>(r#"select * from users u where u.email = $1"#)
        .bind("mhardy2008@gmail.com")
        .fetch_one(&dbpool)
        .await
        .unwrap();

    log::debug!("{:#?}", user);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    log::info!("🚀 Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
