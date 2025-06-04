//! Extractor used to obtain the pool instances to the PostgreSQL database.

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;

use common_state::AppState;

use crate::domain::db::Db;

#[async_trait]
impl<S> FromRequestParts<S> for Db
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        Ok(Self::new(state.pool))
    }
}
