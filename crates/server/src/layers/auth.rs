//! This file contains all authentication related structures and functions.
//! It also provides the Axum layer needed to enable the authentication in the
//! server.

use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::config::Config;

/// Gets the Axum layer used to enable authentication in the HTTP server.
///
/// # Arguments
/// * `config` - Application configuration.
///
/// # Returns
/// The authentication layer.
pub fn authentication_session_layer(config: &Config) -> SessionManagerLayer<MemoryStore> {
    // Session storage backend
    // TODO: use reddis to store the values
    let session_store = MemoryStore::default();

    // Session layer
    SessionManagerLayer::new(session_store).with_expiry(Expiry::OnInactivity(
        time::Duration::hours(config.sessions.timeout_in_hours.into()),
    ))
}
