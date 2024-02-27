//! This file lists all traits used for  the database structures.

/// Trait that implements or defines some basic SQL functions.
/// TODO: add tests
pub trait CRUD
where
    Self::Error: From<sqlx::Error>,
    Self::Id: sqlx::postgres::PgHasArrayType,
    for<'a> &'a Self::Id:
        Send + Sync + sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
    for<'a> &'a [Self::Id]: Send + Sync + sqlx::Encode<'a, sqlx::Postgres>,
    for<'a> &'a Self::Pool: sqlx::Executor<'a, Database = sqlx::Postgres>,
    for<'a> Self::Struct: Send + Unpin + sqlx::FromRow<'a, sqlx::postgres::PgRow>,
{
    /// Structure used to insert or update records.
    type Data;

    /// Error returned by methods.
    type Error;

    /// Type of the identifier of the database tables.
    type Id;

    /// Database handle type.
    type Pool;

    /// Structure that mirrors the table.
    type Struct;

    /// Get the ID column for a database structure.
    ///
    /// # Returns
    /// ID of the database structure.
    fn id(&self) -> &Self::Id;

    /// Get the table's name for this database structure.
    ///
    /// # Returns
    /// Name of the corresponding table in the database.
    fn table_name() -> &'static str;

    /// Deletes an entry from the database.
    ///
    /// # Arguments
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Empty result or an error.
    async fn delete(&self, db: &Self::Pool) -> Result<(), Self::Error> {
        Self::delete_by_id(self.id(), db).await
    }

    /// Deletes a list of entries from the database.
    ///
    /// # Arguments
    /// * `ids` - List of IDs of the entries to remove.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Empty result or an error.
    async fn delete_batch(ids: &[Self::Id], db: &Self::Pool) -> Result<(), Self::Error> {
        let _ = sqlx::query(&format!(
            "DELETE FROM {} WHERE id=ANY($1)",
            Self::table_name()
        ))
        .bind(ids)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Deletes an entry from the database giving its ID.
    ///
    /// # Arguments
    /// * `id` - ID of the entry to remove.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Empty result or an error.
    async fn delete_by_id(id: &Self::Id, db: &Self::Pool) -> Result<(), Self::Error> {
        let _ = sqlx::query(&format!("DELETE FROM {} WHERE id=$1", Self::table_name()))
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    /// Gets a record from the database giving its ID.
    ///
    /// # Arguments
    /// * `id` - ID of the entry to fetch.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entry found or an error.
    async fn get(id: &Self::Id, db: &Self::Pool) -> Result<Self::Struct, Self::Error> {
        sqlx::query_as::<_, Self::Struct>(&format!(
            "SELECT * FROM {} WHERE id=$1",
            Self::table_name()
        ))
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }

    /// Gets a list of records from the database giving their IDs.
    ///
    /// # Arguments
    /// * `ids` - List of IDs of the entries to fetch.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entries found or an error.
    async fn get_batch(
        ids: &[Self::Id],
        db: &Self::Pool,
    ) -> Result<Vec<Self::Struct>, Self::Error> {
        sqlx::query_as::<_, Self::Struct>(&format!(
            "SELECT * FROM {} WHERE id=ANY($1)",
            Self::table_name()
        ))
        .bind(ids)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Insert a new record in database.
    ///
    /// # Arguments
    /// * `data` - Data structure to be added to the database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entry inserted or an error.
    async fn insert(data: &Self::Data, db: &Self::Pool) -> Result<Self::Struct, Self::Error>;

    // TODO: rename: batch_delete, batch_get
    // TODO: upsert, insert, update
    // TODO: batch_insert
    // TODO: update_by_sid, batch_update
}
