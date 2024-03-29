//! Extractor used to get the authentication user.

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::RequestPartsExt;

use crate::prelude::*;

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticationUser
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
