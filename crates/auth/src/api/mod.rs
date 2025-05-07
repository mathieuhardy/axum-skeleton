//! List of endpoints provided by this crate.

mod auth;
mod user_confirmation;

/// Builds a router for the authorization crate.
///
/// # Returns
/// An Axum router.
pub fn router() -> axum::Router<common_state::AppState> {
    axum::Router::new()
        .merge(auth::router())
        .merge(user_confirmation::router())
}
