//! List all structures and methods used to manage users in database.

use database_derives::*;

use crate::prelude::*;

/// Mirrors the `users`'s' table.
///
/// TODO: create a derive macro that implements create, update
#[derive(
    Clone, Debug, Default, PartialEq, FromRow, Deserialize, Serialize, TryFromVec, Export, Validate,
)]
#[export(Data, Request)]
#[export(derives(Data(Debug, SqlxPgInsertable, Validate)))]
#[export(derives(Request(Default, Debug, Deserialize, Serialize, Validate)))]
pub struct User {
    /// Unique record identifier.
    #[optional_in(Request)]
    pub id: Uuid,

    /// First name of the user.
    #[optional_in(Data, Request)]
    #[validate(length(min = 1))]
    pub first_name: String,

    /// Last name of the user.
    #[optional_in(Data, Request)]
    #[validate(length(min = 1))]
    pub last_name: String,

    /// Email of the user.
    #[optional_in(Data, Request)]
    #[validate(email)]
    pub email: String,

    /// Date of record's creation.
    pub created_at: DateTime<Utc>,

    /// Date of record's last update.
    pub updated_at: DateTime<Utc>,
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
        Self {
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            email: request.email.clone(),
        }
    }
}

// TODO: add tests
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
    ///       email: None
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
    pub async fn find_by_filters(filters: &Filters, db: &PgPool) -> Res<Vec<Self>> {
        let users = sqlx::query_as::<_, User>(SQL_USERS_FIND_BY_FILTERS)
            .bind(&filters.first_name)
            .bind(&filters.last_name)
            .bind(&filters.email)
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
    pub async fn all(db: &PgPool) -> Res<Vec<Self>> {
        Self::find_by_filters(&Filters::default(), db).await
    }
}
