//! This file contains all authentication related structures and functions.
//! It also provides the Axum layer needed to enable the authentication in the
//! server.

use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use sqlx::postgres::PgPool;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use auth::AuthBackend;

use crate::config::Config;

/// Gets the Axum layer used to enable authentication in the HTTP server.
///
/// # Arguments
/// * `db` - Database connection.
///
/// # Returns
/// The authentication layer.
pub fn authentication_layer(
    config: &Config,
    db: &PgPool,
) -> AuthManagerLayer<AuthBackend, MemoryStore> {
    // Session storage backend
    // TODO: use reddis to store the values
    let session_store = MemoryStore::default();

    // Session layer
    let session_layer = SessionManagerLayer::new(session_store).with_expiry(Expiry::OnInactivity(
        time::Duration::hours(config.sessions.timeout_in_hours.into()),
    ));

    // Authentication backend
    let backend = AuthBackend::new(db);

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
