//! Extractor used to obtain the `AuthUser` from the `AuthSession`.
//! This allows to obtain information of the caller of a request in order to check its accesses
//! later.

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::RequestPartsExt;

use common_core::AppState;

use crate::domain::auth_backend::AuthSession;
use crate::domain::auth_user::AuthUser;

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_session = parts
            .extract::<AuthSession>()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match auth_session.user {
            Some(user) => Ok(user),
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
