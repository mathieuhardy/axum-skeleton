//! SQLx implementation of the UserStore trait.

use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use sqlx::types::Json;
use sqlx::{FromRow, Type};

use auth::AuthUserConfirmation;
use database::SharedDb;
use security::password::Password;

use crate::domain::port::UserStore;
use crate::domain::user::{User, UserData, UserFilters, UserRole};
use crate::prelude::*;

/// List of users roles in the DB enum.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub(crate) enum DbUserRole {
    /// See `UserRole::Admin`.
    Admin,

    /// See `UserRole::Normal`.
    Normal,

    /// See `UserRole::Guest`.
    #[default]
    Guest,
}

impl From<DbUserRole> for UserRole {
    fn from(db_role: DbUserRole) -> Self {
        match db_role {
            DbUserRole::Admin => UserRole::Admin,
            DbUserRole::Normal => UserRole::Normal,
            DbUserRole::Guest => UserRole::Guest,
        }
    }
}

impl From<UserRole> for DbUserRole {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => DbUserRole::Admin,
            UserRole::Normal => DbUserRole::Normal,
            UserRole::Guest => DbUserRole::Guest,
        }
    }
}

/// Mirrors the `users`'s' table.
#[derive(Clone, Default, FromRow, Deserialize, Serialize, derive_more::Debug)]
pub(crate) struct DbUser {
    /// See `User::id`.
    pub id: Uuid,

    /// See `User::first_name`.
    pub first_name: Option<String>,

    /// See `User::last_name`.
    pub last_name: Option<String>,

    /// See `User::email`.
    pub email: String,

    /// See `User::role`.
    pub role: DbUserRole,

    /// See `User::password`.
    #[debug(skip)]
    pub password: String,

    /// See `User::created_at`.
    pub created_at: DateTime<Utc>,

    /// See `User::updated_at`.
    pub updated_at: DateTime<Utc>,

    /// See `User::pending_confirmation`.
    pub pending_confirmation: Option<Json<AuthUserConfirmation>>,
}

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        Self {
            id: db_user.id,
            first_name: db_user.first_name.unwrap_or_default(),
            last_name: db_user.last_name.unwrap_or_default(),
            email: db_user.email,
            role: db_user.role.into(),
            password: Password::from(db_user.password),
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
            pending_confirmation: db_user.pending_confirmation.map(|e| e.0),
        }
    }
}

/// SQLx version of the UserStore trait.
pub struct SQLxUserStore {
    /// Database connection pool.
    db: SharedDb,
}

impl SQLxUserStore {
    /// Creates a new SQLxUserStore instance.
    ///
    /// # Arguments
    /// * `db`: Database handle.
    ///
    /// # Returns
    /// A new instance of SQLxUserStore.
    #[must_use]
    pub fn new(db: SharedDb) -> Self {
        Self { db }
    }
}

impl UserStore for SQLxUserStore {
    fn exists(&self, user_id: Uuid) -> BoxFuture<'static, ApiResult<bool>> {
        let db = self.db.clone();

        Box::pin(async move {
            let res = sqlx::query_file_scalar!("sql/exists.sql", user_id)
                .fetch_one(db.lock().await.clone())
                .await
                .is_ok();

            Ok(res)
        })
    }

    fn delete_by_id(&self, user_id: Uuid) -> BoxFuture<'static, ApiResult<()>> {
        let db = self.db.clone();

        Box::pin(async move {
            sqlx::query_file!("sql/delete_by_id.sql", user_id)
                .execute(db.lock().await.clone())
                .await?;

            Ok(())
        })
    }

    fn get_by_id(&self, user_id: Uuid) -> BoxFuture<'static, ApiResult<User>> {
        let db = self.db.clone();

        Box::pin(async move {
            let user = sqlx::query_file_as!(DbUser, "sql/get_by_id.sql", user_id)
                .fetch_one(db.lock().await.clone())
                .await?;

            Ok(user.into())
        })
    }

    fn get_by_filters(&self, filters: UserFilters) -> BoxFuture<'static, ApiResult<Vec<User>>> {
        let db = self.db.clone();
        let role = filters.role.map(Into::into);

        Box::pin(async move {
            let users = sqlx::query_file_as!(
                DbUser,
                "sql/get_by_filters.sql",
                filters.first_name,
                filters.last_name,
                filters.email,
                role as Option<DbUserRole>,
            )
            .fetch_all(db.lock().await.clone())
            .await?;

            Ok(users.into_iter().map(User::from).collect())
        })
    }

    fn create(&self, data: UserData) -> BoxFuture<'static, ApiResult<User>> {
        let db = self.db.clone();
        let role: DbUserRole = data.role.into();

        Box::pin(async move {
            let user = sqlx::query_file_as!(
                DbUser,
                "sql/create.sql",
                data.first_name,
                data.last_name,
                data.email,
                role as DbUserRole,
                data.password.as_str()
            )
            .fetch_one(db.lock().await.clone())
            .await?;

            Ok(user.into())
        })
    }

    fn update(&self, user_id: Uuid, data: UserData) -> BoxFuture<'static, ApiResult<User>> {
        let db = self.db.clone();
        let role: DbUserRole = data.role.into();

        Box::pin(async move {
            let user = sqlx::query_file_as!(
                DbUser,
                "sql/update.sql",
                user_id,
                data.first_name,
                data.last_name,
                data.email,
                role as DbUserRole,
                data.password.as_str()
            )
            .fetch_one(db.lock().await.clone())
            .await?;

            Ok(user.into())
        })
    }

    fn set_user_password(
        &self,
        user_id: Uuid,
        password: Password,
    ) -> BoxFuture<'static, ApiResult<()>> {
        let db = self.db.clone();

        Box::pin(async move {
            sqlx::query_file!("sql/set_password.sql", user_id, password.as_str())
                .execute(db.lock().await.clone())
                .await?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::database::setup_test_database;
    use test_utils::rand::*;

    use crate::tests::utils::create_user;

    #[tokio::test]
    async fn test_user_exists() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        assert!(!repo.exists(random_id()).await?);

        let user = create_user(UserRole::Admin, &db).await?;
        assert!(repo.exists(user.id).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_id() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        let user = create_user(UserRole::Admin, &db).await?;
        assert!(repo.get_by_id(user.id).await.is_ok());

        repo.delete_by_id(user.id).await?;
        assert!(repo.get_by_id(user.id).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_id() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        assert!(repo.get_by_id(random_id()).await.is_err());

        let user = create_user(UserRole::Admin, &db).await?;
        let fetched = repo.get_by_id(user.id).await?;
        assert_eq!(fetched, user);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_filters() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        let user_1 = create_user(UserRole::Admin, &db).await?;
        let user_2 = create_user(UserRole::Guest, &db).await?;

        let users = repo
            .get_by_filters(UserFilters {
                first_name: Some(user_1.first_name.clone()),
                ..Default::default()
            })
            .await?;
        assert_eq!(users.len(), 1);
        assert!(users.iter().any(|u| u.id == user_1.id));

        let users = repo
            .get_by_filters(UserFilters {
                last_name: Some(user_2.last_name.clone()),
                ..Default::default()
            })
            .await?;
        assert_eq!(users.len(), 1);
        assert!(users.iter().any(|u| u.id == user_2.id));

        let users = repo
            .get_by_filters(UserFilters {
                email: Some(user_1.email.clone()),
                ..Default::default()
            })
            .await?;
        assert_eq!(users.len(), 1);
        assert!(users.iter().any(|u| u.id == user_1.id));

        let users = repo
            .get_by_filters(UserFilters {
                role: Some(UserRole::Guest),
                ..Default::default()
            })
            .await?;
        assert!(users.iter().any(|u| u.id == user_2.id));

        let users = repo
            .get_by_filters(UserFilters {
                first_name: Some(user_1.first_name.clone()),
                last_name: Some(user_2.last_name.clone()),
                ..Default::default()
            })
            .await?;
        assert!(users.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        let user = repo
            .create(UserData {
                first_name: Some(random_string()),
                last_name: Some(random_string()),
                email: random_string(),
                role: UserRole::Normal,
                password: random_password(),
            })
            .await?;

        let fetched = repo.get_by_id(user.id).await?;
        assert_eq!(fetched, user);

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        let user = create_user(UserRole::Admin, &db).await?;

        let data = UserData {
            first_name: Some(random_string()),
            last_name: Some(random_string()),
            email: random_string(),
            role: UserRole::Normal,
            password: random_password(),
        };

        let updated = repo.update(user.id, data.clone()).await?;
        assert_eq!(updated.first_name, data.first_name.unwrap());
        assert_eq!(updated.last_name, data.last_name.unwrap());
        assert_eq!(updated.email, data.email);
        assert_eq!(updated.role, data.role);
        assert_eq!(updated.password, data.password);

        Ok(())
    }

    #[tokio::test]
    async fn test_set_password() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_database().await?;
        let repo = SQLxUserStore::new(db.clone());

        let user = create_user(UserRole::Admin, &db).await?;
        let password = random_password();

        repo.set_user_password(user.id, password.clone()).await?;

        let fetched = repo.get_by_id(user.id).await?;
        assert_eq!(fetched.password, password);

        Ok(())
    }
}
