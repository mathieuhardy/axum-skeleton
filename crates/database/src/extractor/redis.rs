//! Extractor used to obtain the connection instance to the Redis database.

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;

use common_state::AppState;

/// Redis connection extractor.
///
/// Examples:
///
/// ```ignore
/// async fn using_connection_extractor(
///     RedisConnection(mut conn): RedisConnection
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
///
/// TODO: to be removed as soon as Redis is used
#[allow(dead_code)]
struct RedisConnection(PooledConnection<'static, RedisConnectionManager>);

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let conn = state
            .redis
            .get_owned()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Self(conn))
    }
}
