//! SQLx implementation of the `AuthStore` trait.

use futures::future::BoxFuture;
use sqlx::{FromRow, Type};

use crate::domain::auth_user::{AuthUser, AuthUserRole};
use crate::domain::port::AuthStore;
use crate::prelude::*;

/// List of users roles in the DB enum.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum DbAuthUserRole {
    /// See `AuthUserRole::Admin`.
    Admin,

    /// See `AuthUserRole::Normal`.
    Normal,

    /// See `AuthUserRole::Guest`.
    #[default]
    Guest,
}

impl From<DbAuthUserRole> for AuthUserRole {
    fn from(db_role: DbAuthUserRole) -> Self {
        match db_role {
            DbAuthUserRole::Admin => AuthUserRole::Admin,
            DbAuthUserRole::Normal => AuthUserRole::Normal,
            DbAuthUserRole::Guest => AuthUserRole::Guest,
        }
    }
}

impl From<AuthUserRole> for DbAuthUserRole {
    fn from(role: AuthUserRole) -> Self {
        match role {
            AuthUserRole::Admin => DbAuthUserRole::Admin,
            AuthUserRole::Normal => DbAuthUserRole::Normal,
            AuthUserRole::Guest => DbAuthUserRole::Guest,
        }
    }
}

/// Mirrors the `users`'s' table.
#[derive(Clone, Default, FromRow, Deserialize, Serialize, derive_more::Debug)]
pub struct DbAuthUser {
    /// See `User::id`.
    pub id: Uuid,

    /// See `User::email`.
    pub email: String,

    /// See `User::role`.
    pub role: DbAuthUserRole,

    /// See `User::password`.
    #[debug(skip)]
    pub password: String,
}

impl From<DbAuthUser> for AuthUser {
    fn from(db_user: DbAuthUser) -> Self {
        Self {
            id: db_user.id,
            email: db_user.email,
            role: db_user.role.into(),
            password: db_user.password,
        }
    }
}

/// SLQx's implementation of the `AuthStore` trait.
#[derive(Clone, Debug)]
pub struct SQLxAuthStore {
    /// Database connection pool.
    db: PgPool,
}

impl SQLxAuthStore {
    /// Creates a new instance of the SQLx authentication store.
    #[must_use]
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }
}

impl AuthStore for SQLxAuthStore {
    fn find_user_by_email(&self, email: &str) -> BoxFuture<'static, Result<AuthUser, Error>> {
        let db = self.db.clone();
        let email = email.to_string();

        Box::pin(async move {
            let user = sqlx::query_file_as!(DbAuthUser, "sql/find_user_by_email.sql", email)
                .fetch_one(&db)
                .await?;

            Ok(user.into())
        })
    }

    fn get_user_by_id(&self, user_id: &Uuid) -> BoxFuture<'static, Result<AuthUser, Error>> {
        let db = self.db.clone();
        let user_id = *user_id;

        Box::pin(async move {
            let user = sqlx::query_file_as!(DbAuthUser, "sql/get_user_by_id.sql", user_id)
                .fetch_one(&db)
                .await?;

            Ok(user.into())
        })
    }
}

#[cfg(test)]
mod tests {
    use test_utils::database::setup_test_database;
    use test_utils::rand::*;

    use crate::tests::utils::*;

    use super::*;

    #[tokio::test]
    async fn test_repo_find_by_email() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;

        let repo = SQLxAuthStore::new(&db);

        let auth_user = AuthUser {
            email: random_email(),
            password: random_password(),
            ..Default::default()
        };

        let _ = create_user(&auth_user, &db).await?;

        let user = repo.find_user_by_email(&auth_user.email).await?;
        assert_eq!(user.email, auth_user.email);

        let res = repo.find_user_by_email(&random_email()).await;
        assert!(res.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_repo_get_by_id() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;

        let repo = SQLxAuthStore::new(&db);

        let auth_user = AuthUser {
            email: random_email(),
            password: random_password(),
            ..Default::default()
        };

        let user = create_user(&auth_user, &db).await?;

        let fetched = repo.get_user_by_id(&user.id).await?;
        assert_eq!(fetched.id, user.id);

        let res = repo.get_user_by_id(&random_id()).await;
        assert!(res.is_err());

        Ok(())
    }
}
