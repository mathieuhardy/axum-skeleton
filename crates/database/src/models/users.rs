//! List all structures and methods used to manage users in database.

use database_derives::*;

use crate::password::checks;
use crate::prelude::*;

/// Mirrors the `users`'s' table.
#[derive(Clone, Debug, PartialEq, FromRow, Deserialize, Serialize, TryFromVec, Validate)]
pub struct User {
    /// Unique record identifier.
    pub id: Uuid,

    /// First name of the user.
    pub first_name: String,

    /// Last name of the user.
    pub last_name: String,

    /// Email of the user.
    pub email: String,

    /// Password of the user (hashed of course).
    pub password: String,

    /// Date of record's creation.
    pub created_at: DateTime<Utc>,

    /// Date of record's last update.
    pub updated_at: DateTime<Utc>,
}

/// Structure used by HTTP endpoint to query a modification in  the database.
/// This structure is not expected to be used directly in queries. It must be converted first to a
/// `UserData`.
#[derive(Clone, Default, Debug, Deserialize, Serialize, Validate)]
pub struct UserRequest {
    /// See `User::sid`.
    pub id: Option<Uuid>,

    /// See `User::first_name`.
    #[validate(length(min = 1))]
    pub first_name: Option<String>,

    /// See `User::last_name`.
    #[validate(length(min = 1))]
    pub last_name: Option<String>,

    /// See `User::email`.
    #[validate(email)]
    pub email: Option<String>,

    /// See `User::password`.
    #[validate(custom = "validate_password")]
    pub password: Option<String>,
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

    /// See `User::password`.
    pub password: Option<String>,
}

/// Structure provided to update the user's password
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PasswordUpdateRequest {
    /// Current password of the user. Not validated as it will be simply compared with the entry in
    /// database before updating.
    pub current: String,

    /// New password to be set in database.
    #[validate(custom = "validate_password")]
    pub new: String,
}

/// Structure that list all filters available for querying database.
#[derive(Debug, Default, Deserialize)]
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

    /// Password hash of the user (or None).
    #[serde_as(as = "NoneAsEmptyString")]
    pub password: Option<String>,
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

impl From<UserRequest> for UserData {
    fn from(request: UserRequest) -> Self {
        // Don't copy the password field from request. Keep this field empty by default as it must
        // be hashed before written to database.
        Self {
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            email: request.email.clone(),
            password: None,
        }
    }
}

impl From<&UserRequest> for UserData {
    fn from(request: &UserRequest) -> Self {
        (*request).clone().into()
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
    ///       password: None,
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
            .bind(&filters.password)
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

/// Validate a password accoring to application rules.
///
/// # Arguments
/// * `password` - Password to be checked.
///
/// # Returns
/// No output if the password is correct, an error otherwise.
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let checks = checks().map_err(|_| ValidationError::new("cannot_access_checks"))?;

    if utils::password::verify(password, checks) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}
