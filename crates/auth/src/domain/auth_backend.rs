//! Session and backend structure passed as middleware to Axum in order to proceed to
//! authentication.

use axum_login::{AuthnBackend, UserId};

use crate::domain::auth_user::AuthUser;
use crate::prelude::*;

/// Authentication session convenient type.
pub type AuthSession = axum_login::AuthSession<AuthBackend>;

/// Structure used to store the credentials that must be provided by a user to check it's
/// existence. This should match a form displayed to the user where he can enter his email and
/// password.
#[derive(Clone, Deserialize, Serialize, derive_more::Debug)]
pub struct AuthCredentials {
    /// Email used during authentication.
    pub email: String,

    /// Password used during authentication.
    #[debug(skip)]
    pub password: String,
}

/// Authentication backend structure that contains all needed data (e.g. a connection to the
/// database in order to fetch users information).
#[derive(Clone, Debug)]
pub struct AuthBackend {
    /// Database handle.
    db: PgPool,
}

impl AuthBackend {
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

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = AuthUser;
    type Credentials = AuthCredentials;
    type Error = Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Try to find the user in database, return Unauthorized if not found.
        let user =
            sqlx::query_file_as!(Self::User, "sql/find_user_by_email.sql", credentials.email)
                .fetch_one(&self.db)
                .await
                .map_err(|_| Error::Unauthorized)?;

        // Verify password
        match utils::hashing::verify(&credentials.password, &user.password).await {
            Ok(true) => Ok(Some(user)),
            Ok(false) => Ok(None),
            _ => Err(Error::Unauthorized),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(Some(
            sqlx::query_file_as!(Self::User, "sql/get_user_by_id.sql", user_id)
                .fetch_one(&self.db)
                .await?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use test_utils::database::setup_test_database;
    use test_utils::rand::*;

    use crate::tests::create_user;

    use super::*;

    #[tokio::test]
    async fn test_backend_authenticate_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;

        let backend = AuthBackend::new(&db);

        let res = backend
            .authenticate(AuthCredentials {
                email: random_email(),
                password: random_password(),
            })
            .await;

        assert!(matches!(res, Err(Error::Unauthorized)));

        Ok(())
    }

    #[tokio::test]
    async fn test_backend_authenticate_invalid_password() -> Result<(), Box<dyn std::error::Error>>
    {
        let db = setup_test_database().await?;

        let backend = AuthBackend::new(&db);

        let auth_user = AuthUser {
            email: random_email(),
            password: random_password(),
            ..Default::default()
        };

        let _ = create_user(&auth_user, &db).await?;

        let res = backend
            .authenticate(AuthCredentials {
                email: auth_user.email.clone(),
                password: random_password(),
            })
            .await;

        assert!(matches!(res, Ok(None)));

        Ok(())
    }

    #[tokio::test]
    async fn test_backend_authenticate_nominal() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;

        let backend = AuthBackend::new(&db);

        let auth_user = AuthUser {
            email: random_email(),
            password: random_password(),
            ..Default::default()
        };

        let _ = create_user(&auth_user, &db).await?;

        let res = backend
            .authenticate(AuthCredentials {
                email: auth_user.email.clone(),
                password: auth_user.password.clone(),
            })
            .await;

        assert!(matches!(res, Ok(Some(_))));

        Ok(())
    }

    #[tokio::test]
    async fn test_backend_get_user_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;

        let backend = AuthBackend::new(&db);

        let res = backend.get_user(&random_id()).await;
        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_backend_get_user_nominal() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;

        let backend = AuthBackend::new(&db);

        let auth_user = AuthUser {
            email: random_email(),
            password: random_password(),
            ..Default::default()
        };

        let auth_user = create_user(&auth_user, &db).await?;

        let res = backend.get_user(&auth_user.id).await;
        assert!(matches!(res, Ok(Some(_))));

        Ok(())
    }
}
