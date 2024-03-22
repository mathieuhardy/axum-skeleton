//! This file contains all authentication related structures and functions.
//! It also provides the Axum layer needed to enable the authentication in the
//! server.

use async_trait::async_trait;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId};
use sqlx::PgPool;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use database::models::users::*;
use database::traits::sqlx::postgres::crud::CRUD;
use utils::hashing;

use crate::prelude::*;

/// User structure used during authentication (simply a wrapper around the database's type).
#[derive(Debug, Clone)]
pub struct AuthenticationUser(User);

impl AuthUser for AuthenticationUser {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.0.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // We're using the password as a unique hash so that if the user changes its password,
        // the session is invalidated.
        self.0.password.as_bytes()
    }
}

/// Structure used to store the credentials that must be provided by a user to check it's
/// existence. This should match a form displayed to the user where he can enter his email and
/// password.
#[derive(Clone)]
pub struct Credentials {
    /// Email used during authentication.
    email: String,

    /// Password used during authentication.
    password: String,
}

/// Authentication backend structure that contains all needed data (e.g. a connection to the
/// database in order to fetch users information).
#[derive(Clone)]
pub struct Backend {
    /// Database handle.
    db: PgPool,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = AuthenticationUser;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Try to find the user in database, return Unauthorized if not found.
        let user: User = User::find_by_filters(
            &Filters {
                email: Some(credentials.email),
                ..Filters::default()
            },
            &self.db,
        )
        .await
        .map_err(|_| Error::Unauthorized)?
        .try_into()
        .map_err(|_| Error::Unauthorized)?;

        // Verify password
        hashing::verify(&credentials.password, &user.password).await?;

        Ok(Some(AuthenticationUser(user.clone())))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(Some(AuthenticationUser(
            User::get(user_id, &self.db).await?,
        )))
    }
}

impl Backend {
    /// Creates a new backend providing the needed data.
    ///
    /// # Arguments
    /// * `db` - Database connection.
    ///
    /// # Returns
    /// New instance of the authentication backend.
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }
}

/// Gets the Axum layer used to enable authentication in the HTTP server.
///
/// # Arguments
/// * `db` - Database connection.
///
/// # Returns
/// The authentication layer.
pub fn authentication_layer(db: &PgPool) -> AuthManagerLayer<Backend, MemoryStore> {
    // Session storage backend
    // TODO: use reddis to store the values
    let session_store = MemoryStore::default();

    // Session layer
    // TODO: with_secure ?
    // TODO: get expiration date from configuration
    // TODO: with_signed ?
    let session_layer = SessionManagerLayer::new(session_store)
        //.with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));
    //.with_signed(cookie::Key::generate());

    // Authentication backend
    let backend = Backend::new(db);

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
