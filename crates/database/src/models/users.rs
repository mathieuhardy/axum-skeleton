//! List all structures and methods used to manage users in database.

use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

use database_derives::*;

use crate::prelude::*;

/// List of users roles.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    /// User with all privileges.
    Admin,

    /// Normal user.
    Normal,

    /// User with very limited privileges.
    #[default]
    Guest,
}

impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_user_role")
    }
}

/// Mirrors the `users`'s' table.
#[derive(
    Clone, Debug, Default, PartialEq, FromRow, Deserialize, Serialize, TryFromVec, Validate,
)]
pub struct User {
    /// Unique record identifier.
    pub id: Uuid,

    /// First name of the user.
    pub first_name: String,

    /// Last name of the user.
    pub last_name: String,

    /// Email of the user.
    pub email: String,

    /// Role of the user.
    pub role: UserRole,

    /// Password of the user (hashed of course).
    pub password: String,

    /// Date of record's creation.
    pub created_at: DateTime<Utc>,

    /// Date of record's last update.
    pub updated_at: DateTime<Utc>,
}

/// Data structure passed to database queries when inserting or updating entries.
#[derive(Clone, Default, Debug, Serialize, SqlxPgInsertable)]
pub struct UserData {
    /// See `User::first_name`.
    pub first_name: Option<String>,

    /// See `User::last_name`.
    pub last_name: Option<String>,

    /// See `User::email`.
    pub email: Option<String>,

    /// See `User::role`.
    pub role: Option<UserRole>,

    /// See `User::password`.
    pub password: Option<String>,
}

/// Structure that list all filters available for querying database.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde_as]
pub struct Filters {
    /// First name of the user (or None).
    #[serde_as(as = "NoneAsEmptyString")]
    pub first_name: Option<String>,

    /// Last name of the user (or None).
    #[serde_as(as = "NoneAsEmptyString")]
    pub last_name: Option<String>,

    /// Email of the user (or None).
    #[serde_as(as = "NoneAsEmptyString")]
    pub email: Option<String>,

    /// Role of the user (or None).
    pub role: Option<UserRole>,
}

impl CRUD for User {
    type Data = UserData;
    type Error = Error;
    type Id = Uuid;
    type Struct = Self;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn table_name() -> &'static str {
        "users"
    }
}

impl User {
    /// Finds some users matching some filters.
    ///
    /// # Arguments
    /// * `filters` - Filters used for matching.
    /// * `db` - Database connection.
    ///
    /// # Returns
    /// A List of users or an Error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    ///   use database::models::users::*;
    ///   use sqlx::postgres::*;
    ///
    ///   let filters = Filters {
    ///       first_name: Some("foo".to_string()),
    ///       last_name: Some("foo".to_string()),
    ///       email: None,
    ///       role: Some(UserRole::Normal),
    ///   };
    ///
    ///   let db = PgPoolOptions::new()
    ///     .max_connections(8)
    ///     .connect("database_url")
    ///     .await?;
    ///
    ///   let results = User::find_by_filters(&filters, &db).await?;
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn find_by_filters(filters: &Filters, db: &PgPool) -> Res<Vec<User>> {
        let users = sqlx::query_as::<_, User>(SQL_USERS_FIND_BY_FILTERS)
            .bind(&filters.first_name)
            .bind(&filters.last_name)
            .bind(&filters.email)
            .bind(&filters.role)
            .fetch_all(db)
            .await
            .inspect_err(|e| event!(Level::ERROR, "{e}"))
            .map_err(|_| Error::NotFound)?;

        if users.is_empty() {
            return Err(Error::NotFound);
        }

        Ok(users)
    }

    /// Finds all users.
    ///
    /// # Arguments
    /// * `db` - Database connection.
    ///
    /// # Returns
    /// A List of users or an Error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    ///   use database::models::users::*;
    ///   use sqlx::postgres::*;
    ///
    ///   let db = PgPoolOptions::new()
    ///     .max_connections(8)
    ///     .connect("database_url")
    ///     .await?;
    ///
    ///   let results = User::all(&db).await?;
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn all(db: &PgPool) -> Res<Vec<User>> {
        Self::find_by_filters(&Filters::default(), db).await
    }
}
