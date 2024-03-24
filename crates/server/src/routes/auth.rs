//! Here you'll find all handlers related to user login or logout. Note that this is a very basic
//! implementation. You'll want, for example enable any kind of 2FA here.
//!
//! Let's say we want to send one-time code by email to the user before allowing its login. In the
//! `login` handler, after verifying the password of the user, and before creating the session,
//! we'll proceed this way:
//!
//! 1. create a one-time code or select one in a predefined secret list.
//! 2. send an email with this code to the user.
//! 3. redirect the caller to a page to verify the code.
//!
//! The frontend will then post the code entered by the user to another handler that will (upon
//! success) create the session. The user is now logged in officially.

use crate::prelude::*;

/// Builds a router for the entire application.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    let router = Router::new();

    router
        .route("/login", post(post::login))
        .route("/logout", post(post::logout))
}

/// List of all POST routes.
mod post {
    use super::*;

    /// Login handler.
    #[axum::debug_handler]
    #[instrument]
    pub async fn login(
        mut auth_session: AuthSession,
        FormOrJson(credentials): FormOrJson<Credentials>,
    ) -> impl IntoResponse {
        // Try to authenticate the user
        let user = match auth_session.authenticate(credentials.clone()).await {
            Ok(Some(user)) => user,

            Ok(None) => {
                event!(Level::ERROR, "Invalid credentials");

                return StatusCode::UNAUTHORIZED.into_response();
            }

            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        // Create the sesion for this user
        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        event!(Level::INFO, "Successfully logged in as {}", user.0.email);

        StatusCode::OK.into_response()
    }

    /// Logout handler.
    #[axum::debug_handler]
    #[instrument]
    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => {
                event!(Level::INFO, "Successfully logged out");

                StatusCode::OK.into_response()
            }

            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
