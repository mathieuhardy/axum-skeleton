use std::marker::PhantomData;
use std::net::SocketAddr;

use async_trait::async_trait;
use axum::error_handling::HandleErrorLayer;
use axum::middleware::{self, Next};
use axum::routing::get;
use axum::Router;
use axum_core::extract::{FromRef, FromRequestParts};
use axum_core::response::{IntoResponse, Response};
use axum_core::BoxError;
use http::request::Parts;
use http::{Request, StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite, FromRow, Sqlite, SqlitePool};
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

#[async_trait]
trait UserStore<User, UserId> {
    type Error: std::error::Error;

    async fn load(&self, user_id: &UserId) -> Result<Option<User>, Self::Error>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AuthData<User, UserId> {
    user: Option<User>,
    user_id: Option<UserId>,
}

#[derive(Debug, Clone)]
struct AuthState<User, UserId, Store> {
    user_store: Store,
    _user: PhantomData<User>,
    _user_id: PhantomData<UserId>,
}

impl<User, UserId, Store> AuthState<User, UserId, Store> {
    fn new(user_store: Store) -> Self {
        Self {
            user_store,
            _user: PhantomData,
            _user_id: PhantomData,
        }
    }
}

#[derive(Debug)]
struct Auth<User, UserId, Store>
where
    Store: UserStore<User, UserId>,
{
    session: Session,
    auth_data: AuthData<User, UserId>,
    user_store: Store,
}

// impl<User, UserId, Store> Auth<User, UserId, Store>
// where
//     User: Clone + Serialize + for<'a> Deserialize<'a>,
//     UserId: Clone + Serialize + for<'a> Deserialize<'a>,
//     Store: UserStore<User, UserId>,
// {
//     const AUTH_DATA_KEY: &'static str = "_auth_data";
//
//     async fn login(&mut self, user_id: &UserId) {
//         if let Some(user) = self.user_store.load(user_id).await.unwrap() {
//             self.auth_data.user = Some(user);
//             self.auth_data.user_id = Some(user_id.clone());
//             self.update_session();
//         }
//     }
//
//     fn logout(&mut self) {
//         self.session
//             .remove::<AuthData<User, UserId>>(Self::AUTH_DATA_KEY)
//             .expect("infallible");
//         self.auth_data = AuthData {
//             user: None,
//             user_id: None,
//         };
//         self.update_session();
//     }
//
//     fn user(&self) -> Option<User> {
//         self.auth_data.user.clone()
//     }
//
//     fn update_session(&self) {
//         self.session
//             .insert(Self::AUTH_DATA_KEY, self.auth_data.clone())
//             .expect("infallible")
//     }
// }

#[async_trait]
impl<S, User, UserId, Store> FromRequestParts<S> for Auth<User, UserId, Store>
where
    S: Send + Sync,
    User: Serialize + for<'a> Deserialize<'a> + Clone + Send,
    UserId: Serialize + for<'a> Deserialize<'a> + Clone + Send + Sync,
    Store: UserStore<User, UserId> + Send + Sync,
    AuthState<User, UserId, Store>: FromRef<S>,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        let mut auth_data: AuthData<User, UserId> = session
            .get(Self::AUTH_DATA_KEY)
            .expect("infallible")
            .unwrap_or(AuthData {
                user: None,
                user_id: None,
            });

        let AuthState { user_store, .. } = AuthState::from_ref(state);

        // Poll store to refresh current user.
        if let Some(ref user_id) = auth_data.user_id {
            match user_store.load(user_id).await {
                Ok(user) => auth_data.user = user,

                Err(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        "Could not load from user store. Is the store online?",
                    ))
                }
            }
        };

        Ok(Auth {
            session,
            auth_data,
            user_store,
        })
    }
}

// async fn require_auth<User, UserId, Store, B>(
//     auth: Auth<User, UserId, Store>,
//     request: Request<B>,
//     next: Next<B>,
// ) -> Result<Response, StatusCode>
// where
//     User: Serialize + for<'a> Deserialize<'a> + Clone + Send,
//     UserId: Serialize + for<'a> Deserialize<'a> + Clone + Send + Sync,
//     Store: UserStore<User, UserId> + Send + Sync,
// {
//     if auth.user().is_some() {
//         let response = next.run(request).await;
//         Ok(response)
//     } else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// struct MyUser {
//     id: i64,
//     name: String,
// }

// #[derive(Debug, Clone)]
// struct SqliteUserStore {
//     pool: SqlitePool,
// }

// #[derive(thiserror::Error, Debug)]
// pub enum UserStoreError {
//     #[error("Error")]
//     Error,
// }

// #[async_trait]
// impl<User, UserId> UserStore<User, UserId> for SqliteUserStore
// where
//     User: Send + Sync + Unpin + for<'r> FromRow<'r, sqlite::SqliteRow>,
//     UserId: Sync + sqlx::Type<Sqlite> + for<'q> sqlx::Encode<'q, Sqlite>,
// {
//     type Error = UserStoreError;
//
//     async fn load(&self, user_id: &UserId) -> Result<Option<User>, Self::Error> {
//         let user = sqlx::query_as("select * from users where id = ?")
//             .bind(user_id)
//             .fetch_optional(&self.pool)
//             .await
//             .unwrap();
//         Ok(user)
//     }
// }

type MyAuth = Auth<MyUser, i64, SqliteUserStore>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let session_store = MemoryStore::default();
    // let session_service = ServiceBuilder::new()
    //     .layer(HandleErrorLayer::new(|_| async { StatusCode::BAD_REQUEST }))
    //     .layer(SessionManagerLayer::new(session_store).with_max_age(Duration::days(1)));

    let pool = SqlitePool::connect("sqlite::memory:").await?;

    // sqlx::query(r#"create table users (id integer primary key not null, name text not null)"#)
    //     .execute(&pool)
    //     .await?;
    // sqlx::query(r#"insert into users (id, name) values (?, ?)"#)
    //     .bind(42)
    //     .bind("Ferris")
    //     .execute(&pool)
    //     .await?;

    let user_store = SqliteUserStore { pool };
    let auth_state = AuthState::new(user_store);

    let app = Router::new()
        .route("/admin", get(admin_handler))
        .route_layer(middleware::from_fn_with_state(
            auth_state.clone(),
            require_auth,
        ))
        .route("/login", get(login_handler))
        .route("/logout", get(logout_handler))
        .layer(session_service)
        .with_state(auth_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
async fn login_handler(mut auth: MyAuth) -> impl IntoResponse {
    auth.login(&42).await;
    format!("Logged in as: {:?}", auth.user().unwrap().name)
}

async fn logout_handler(mut auth: MyAuth) -> impl IntoResponse {
    auth.logout();
    "Logged out."
}

async fn admin_handler(auth: MyAuth) -> impl IntoResponse {
    format!("Hi, {}!", auth.user().unwrap().name)
}
