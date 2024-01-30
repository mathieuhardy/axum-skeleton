//! This file contains all structures and methods used to manage users in
//! database.

use database_derives::*;

use crate::prelude::*;

/// Mirrors the `users`'s' table.
/// TODO: create a derive macro that implements create, update
#[derive(Clone, Debug, Default, FromRow, Deserialize, Serialize, TryFromVec, Export)]
#[export(Request)]
#[export(derives(Request(Debug, Deserialize)))]
pub struct User {
    /// Unique record identifier.
    pub id: Uuid,

    /// Name of the user.
    #[optional_in(Request)]
    pub name: String,

    /// Email of the user.
    #[optional_in(Request)]
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
    /// Name of the user (or None).
    #[serde_as(as = "NoneAsEmptyString")]
    pub name: Option<String>,

    /// Email of the user (or None).
    #[serde_as(as = "NoneAsEmptyString")]
    pub email: Option<String>,
}

impl CRUD for User {
    type Data = UserRequest;
    type Error = Error;
    type Id = Uuid;
    type Pool = PgPool;
    type Struct = User;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn table_name() -> &'static str {
        "users"
    }

    async fn insert(data: &Self::Data, db: &Self::Pool) -> Result<Self::Struct, Self::Error> {
        sqlx::query_as::<_, Self::Struct>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        )
        .bind(data.name.clone())
        .bind(data.email.clone())
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }
}

impl User {
    /// Finds some users matching some filters.
    ///
    /// # Arguments:
    /// * `filters` - Filters used for matching.
    /// * `db` - Database connection.
    ///
    /// # Returns:
    /// A List of users or an Error.
    pub async fn find_by_filters(filters: &Filters, db: &PgPool) -> Res<Vec<Self>> {
        let users = sqlx::query_as::<_, User>(SQL_USERS_FIND_BY_FILTERS)
            .bind(&filters.name)
            .bind(&filters.email)
            .fetch_all(db)
            .await
            .inspect_err(|e| log::error!("{e}"))
            .map_err(|_| Error::NotFound)?;

        if users.is_empty() {
            return Err(Error::NotFound);
        }

        Ok(users)
    }

    /// Finds all users.
    ///
    /// # Arguments:
    /// * `db` - Database connection.
    ///
    /// # Returns:
    /// A List of users or an Error.
    pub async fn all(db: &PgPool) -> Res<Vec<Self>> {
        Self::find_by_filters(&Filters::default(), db).await
    }
}
