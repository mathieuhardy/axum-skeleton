//! Extractor used to obtain the pool instances to the PostgreSQL database.

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use sqlx::postgres::PgPool;

use common_core::AppState;

/// PostgreSQL pool extractor.
///
/// Note that when using this extractor in handlers that have the `debug_handler` attribute, it
/// must specifies the state type by doing: `#[axum::debug_handler(state = AppState)]`.
///
/// Examples:
///
/// ```ignore
/// async fn using_db_extractor(
///     DbPool(pool): DbPool
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
pub struct DbPool(pub PgPool);

#[async_trait]
impl<S> FromRequestParts<S> for DbPool
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        Ok(Self(state.db))
    }
}
