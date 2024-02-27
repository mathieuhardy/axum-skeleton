//! This file contains all structures and methods used to manage users in
//! database.

use database_derives::*;

use crate::prelude::*;

/// Mirrors the `users`'s' table.
#[derive(Clone, Debug, Default, FromRow, Deserialize, Serialize, TryFromVec)]
pub struct User {
    /// Unique record identifier.
    pub id: Uuid,

    /// Name of the user.
    pub name: String,

    /// Email of the user.
    pub email: String,

    /// Date of record's creation.
    pub created_at: DateTime<Utc>,

    /// Date of record's last update.
    pub updated_at: DateTime<Utc>,
}

/// Structure used in routes to create or update a record.
#[derive(Debug, Deserialize)]
pub struct UserRequest {
    /// Name to be set.
    pub name: String,

    /// Email to be set.
    pub email: String,
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
