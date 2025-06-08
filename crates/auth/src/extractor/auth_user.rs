//! Extractor used to obtain the `Auth` object that contains a `AuthUser`.
//! This allows to obtain information of the caller of a request in order to check its accesses
//! later.

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use tower_sessions::Session;
use tracing::{event, Level};

use common_state::AppState;
use database::Db;

use crate::domain::auth::Auth;
use crate::domain::auth_user::AuthUser;
use crate::domain::error::Error;
use crate::domain::port::AuthStore;
use crate::infrastructure::SQLxAuthStore;

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Fetch user information from the session
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::SessionNotFound)?;

        let user: Option<AuthUser> = session.get(Self::KEY).await?;

        // Get handle to the user store
        let AppState { pool, .. } = AppState::from_ref(state);
        let db = Db::new(pool);
        let store = SQLxAuthStore::new(&db.into_shared());

        // Fetch user from store (in case it has changed since session creation)
        let user = if let Some(session_user) = user {
            let user = store.get_user_by_id(&session_user.id).await?;

            if user.hash() != session_user.hash() {
                event!(Level::WARN, "User hash mismatch: invalidate session");

                session.flush().await?;

                return Err(Error::Unauthorized);
            }

            Some(user)
        } else {
            None
        };

        Ok(Auth { user, session })
    }
}
