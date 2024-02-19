//! This file lists all traits used for the database structures.

use sqlx::postgres::Postgres;
use sqlx::QueryBuilder;

/// Trait that implements or defines some basic SQL functions.
///
/// TODO: add tests
pub trait CRUD
where
    Self::Data: SqlxPgInsertable,
    Self::Error: From<sqlx::Error>,
    Self::Id: sqlx::postgres::PgHasArrayType,
    for<'a> &'a Self::Id:
        Send + Sync + sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
    for<'a> &'a [Self::Id]: Send + Sync + sqlx::Encode<'a, sqlx::Postgres>,
    for<'a> Self::Struct: Send + Unpin + sqlx::FromRow<'a, sqlx::postgres::PgRow>,
{
    /// Structure used to insert or update records.
    type Data;

    /// Error returned by methods.
    type Error;

    /// Type of the identifier of the database tables.
    type Id;

    /// Structure that mirrors the table.
    type Struct;

    /// Get the ID column for a database structure.
    ///
    /// # Returns
    /// ID of the database structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let foo = Foo {};
    ///
    /// println!("id={:?}", foo.id());
    /// ```
    fn id(&self) -> &Self::Id;

    /// Get the table's name for this database structure.
    ///
    /// # Returns
    /// Name of the corresponding table in the database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let foo = Foo {};
    ///
    /// println!("id={}", foo.table_name());
    /// ```
    fn table_name() -> &'static str;

    /// Deletes a list of entries from the database.
    ///
    /// # Arguments
    /// * `ids` - List of IDs of the entries to remove.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Empty result or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuid_1 = uuid::Uuid::default();
    /// let uuid_2 = uuid::Uuid::default();
    ///
    /// Foo::batch_delete(&[uuid_1, uuid_2], &db).await?;
    /// ```
    async fn batch_delete(ids: &[Self::Id], db: &sqlx::PgPool) -> Result<(), Self::Error> {
        let _ = sqlx::query(&format!(
            "DELETE FROM {} WHERE id = ANY($1)",
            Self::table_name()
        ))
        .bind(ids)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Gets a list of records from the database giving their IDs.
    ///
    /// # Arguments
    /// * `ids` - List of IDs of the entries to fetch.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entries found or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuid_1 = uuid::Uuid::default();
    /// let uuid_2 = uuid::Uuid::default();
    ///
    /// Foo::batch_get(&[uuid_1, uuid_2], &db).await?;
    /// ```
    async fn batch_get(
        ids: &[Self::Id],
        db: &sqlx::PgPool,
    ) -> Result<Vec<Self::Struct>, Self::Error> {
        sqlx::query_as::<_, Self::Struct>(&format!(
            "SELECT * FROM {} WHERE id = ANY($1)",
            Self::table_name()
        ))
        .bind(ids)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Inserts a list of data into the database.
    ///
    /// # Arguments
    /// * `list` - List of data to be inserted in database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entries inserted or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let foos = vec![
    ///   Foo {},
    ///   Foo {},
    /// ];
    ///
    /// Foo::batch_insert(&foos, &db).await?;
    /// ```
    async fn batch_insert(
        list: &[Self::Data],
        db: &sqlx::PgPool,
    ) -> Result<Vec<Self::Struct>, Self::Error> {
        // Verify that all data contains the same columns
        if list.is_empty() {
            return Ok(vec![]);
        }

        let columns = list[0].columns();

        for data in list {
            if data.columns() != columns {
                return Ok(vec![]);
            }
        }

        // Prepare values to be bounded
        let columns = columns.join(", ");

        // Prepare query
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
            "INSERT INTO {} ({}) VALUES (",
            Self::table_name(),
            columns
        ));

        let as_suffix = false;
        Self::Data::bind_unnest_values(&mut query_builder, list, as_suffix);

        query_builder.push(") RETURNING *");

        // Execute query
        query_builder
            .build_query_as::<Self::Struct>()
            .fetch_all(db)
            .await
            .map_err(Into::into)
    }

    /// Updates a list of data into the database.
    ///
    /// # Arguments
    /// * `ids` - List of unique identifiers of the database records.
    /// * `list` - List of data to be updated in database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entries updated or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuids = vec![
    ///     uuid::Uuid::default(),
    ///     uuid::Uuid::default()
    /// ];
    ///
    /// let foos = vec![
    ///   Foo {},
    ///   Foo {},
    /// ];
    ///
    /// Foo::batch_update(&uuids, &foos, &db).await?;
    /// ```
    async fn batch_update(
        ids: &[Self::Id],
        list: &[Self::Data],
        db: &sqlx::PgPool,
    ) -> Result<Vec<Self::Struct>, Self::Error> {
        // Verify that all data contains the same columns
        if list.is_empty() {
            return Ok(vec![]);
        }

        let columns = list[0].columns();

        for data in list {
            if data.columns() != columns {
                return Ok(vec![]);
            }
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("UPDATE {} SET ", Self::table_name(),));

        list[0].bind_update_values(&mut query_builder, Some("tmp_table"));

        query_builder.push("FROM (SELECT UNNEST(");
        query_builder.push_bind(ids);
        query_builder.push(") AS id, ");

        let as_suffix = true;
        Self::Data::bind_unnest_values(&mut query_builder, list, as_suffix);

        query_builder.push(format!(
            ") AS tmp_table WHERE {}.id = tmp_table.id",
            Self::table_name()
        ));

        query_builder.push(" RETURNING *");

        query_builder
            .build_query_as::<Self::Struct>()
            .fetch_all(db)
            .await
            .map_err(Into::into)
    }

    /// Upserts a list of data to the database.
    ///
    /// # Arguments
    /// * `ids` - List of unique identifiers of the database records.
    /// * `list` - List of data to be upserted in database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entries upserted or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuids = vec![
    ///     uuid::Uuid::default(),
    ///     uuid::Uuid::default()
    /// ];
    ///
    /// let foos = vec![
    ///   Foo {},
    ///   Foo {},
    /// ];
    ///
    /// Foo::batch_upsert(&uuids, &foos, &db).await?;
    /// ```
    async fn batch_upsert(
        ids: &[Self::Id],
        list: &[Self::Data],
        db: &sqlx::PgPool,
    ) -> Result<Vec<Self::Struct>, Self::Error> {
        // Verify that all data contains the same columns
        if list.is_empty() {
            return Ok(vec![]);
        }

        let mut data_columns = list[0].columns();

        for data in list {
            if data.columns() != data_columns {
                return Ok(vec![]);
            }
        }

        // Prepare query
        let mut columns = vec!["id"];
        columns.append(&mut data_columns);
        let columns = columns.join(", ");

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
            "INSERT INTO {} ({}) SELECT ",
            Self::table_name(),
            columns
        ));

        query_builder.push("UNNEST(");
        query_builder.push_bind(ids);
        query_builder.push("), ");

        let as_suffix = false;
        Self::Data::bind_unnest_values(&mut query_builder, list, as_suffix);

        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");

        list[0].bind_update_values(&mut query_builder, Some("EXCLUDED"));

        query_builder.push(" RETURNING *");

        query_builder
            .build_query_as::<Self::Struct>()
            .fetch_all(db)
            .await
            .map_err(Into::into)
    }

    /// Deletes an entry from the database.
    ///
    /// # Arguments
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Empty result or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let foo = Foo {};
    ///
    /// foo.delete(&db).await?;
    /// ```
    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), Self::Error> {
        Self::delete_by_id(self.id(), db).await
    }

    /// Deletes an entry from the database giving its ID.
    ///
    /// # Arguments
    /// * `id` - ID of the entry to remove.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Empty result or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuid = uuid::Uuid::default();
    ///
    /// Foo::delete_by_id(&uuid, &db).await?;
    /// ```
    async fn delete_by_id(id: &Self::Id, db: &sqlx::PgPool) -> Result<(), Self::Error> {
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuid = uuid::Uuid::default();
    ///
    /// Foo::get(&uuid, &db).await?;
    /// ```
    async fn get(id: &Self::Id, db: &sqlx::PgPool) -> Result<Self::Struct, Self::Error> {
        sqlx::query_as::<_, Self::Struct>(&format!(
            "SELECT * FROM {} WHERE id=$1",
            Self::table_name()
        ))
        .bind(id)
        .fetch_one(db)
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    /// struct FooRequest {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let request = FooRequest {};
    ///
    /// Foo::insert(&request, &db).await?;
    /// ```
    async fn insert(data: &Self::Data, db: &sqlx::PgPool) -> Result<Self::Struct, Self::Error> {
        // Prepare values to be bounded
        let columns = data.columns().join(", ");

        // Prepare query
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
            "INSERT INTO {} ({}) VALUES ",
            Self::table_name(),
            columns
        ));

        data.bind_insert_values(&mut query_builder);

        query_builder.push(" RETURNING *");

        // Execute query
        query_builder
            .build_query_as::<Self::Struct>()
            .fetch_one(db)
            .await
            .map_err(Into::into)
    }

    /// Updates a record from the database.
    ///
    /// # Arguments
    /// * `data` - Data structure to be updated in the database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entry found or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    /// struct FooRequest {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let foo = Foo {};
    /// let request = FooRequest {};
    ///
    /// foo.update(&request, &db).await?;
    /// ```
    async fn update(
        &self,
        data: &Self::Data,
        db: &sqlx::PgPool,
    ) -> Result<Self::Struct, Self::Error> {
        Self::update_by_id(self.id(), data, db).await
    }

    /// Updates a record from the database giving its ID.
    ///
    /// # Arguments
    /// * `id` - ID of the entry to update.
    /// * `data` - Data structure to be updated in the database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entry found or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    /// struct FooRequest {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let uuid = uuid::Uuid::default();
    /// let request = FooRequest {};
    ///
    /// Foo::update(&uuid, &request, &db).await?;
    /// ```
    async fn update_by_id(
        id: &Self::Id,
        data: &Self::Data,
        db: &sqlx::PgPool,
    ) -> Result<Self::Struct, Self::Error> {
        // Prepare query
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("UPDATE {} SET ", Self::table_name()));

        data.bind_update_values(&mut query_builder, None);

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        query_builder.push(" RETURNING *");

        // Execute query
        query_builder
            .build_query_as::<Self::Struct>()
            .fetch_one(db)
            .await
            .map_err(Into::into)
    }

    /// Upsert a record in database.
    ///
    /// # Arguments
    /// * `data` - Data structure to be added to the database.
    /// * `db` - Database handle.
    ///
    /// # Returns
    /// Entry inserted/updated or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    /// struct FooRequest {}
    ///
    /// impl CRUD for Foo {}
    ///
    /// let request = FooRequest {};
    ///
    /// Foo::upsert(&request, &db).await?;
    /// ```
    async fn upsert(data: &Self::Data, db: &sqlx::PgPool) -> Result<Self::Struct, Self::Error> {
        // Prepare values to be bounded
        let columns = data.columns().join(", ");

        // Prepare query
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("INSERT INTO {} ({}) ", Self::table_name(), columns));

        data.bind_insert_values(&mut query_builder);

        query_builder.push(" ON CONFLICT(id) DO UPDATE SET ");

        data.bind_update_values(&mut query_builder, Some("EXCLUDED"));

        query_builder.push(" RETURNING *");

        // Execute query
        query_builder
            .build_query_as::<Self::Struct>()
            .fetch_one(db)
            .await
            .map_err(Into::into)
    }
}

/// Trait that defines functions used to insert entries from a structure to a SQL request.
pub trait SqlxPgInsertable {
    /// Gets the list of columns names to declare in the insert SQL request.
    ///
    /// # Returns
    /// List of columns names.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl SqlxPgInsertable for Foo {}
    ///
    /// let foo = Foo {};
    ///
    /// println!("columns={:#?}", foo.columns());
    /// ```
    fn columns(&self) -> Vec<&'static str>;

    /// Bind values from the struct to a SQLX query builder for an insert query.
    ///
    /// # Arguments
    /// * `query_builder` - Query builder to be populated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl SqlxPgInsertable for Foo {}
    ///
    /// let foo = Foo {};
    ///
    /// foo.bind_insert_values(&mut query_builder);
    /// ```
    fn bind_insert_values<'a>(
        &'a self,
        query_builder: &mut sqlx::QueryBuilder<'a, sqlx::postgres::Postgres>,
    );

    /// Bind values from the struct to a SQLX query builder for an update query.
    ///
    /// # Arguments
    /// * `query_builder` - Query builder to be populated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl SqlxPgInsertable for Foo {}
    ///
    /// let foo = Foo {};
    ///
    /// foo.bind_update_values(&mut query_builder, None);
    /// ```
    fn bind_update_values<'a>(
        &'a self,
        query_builder: &mut sqlx::QueryBuilder<'a, sqlx::postgres::Postgres>,
        prefix: Option<&str>,
    );

    /// Bind unnest values from the struct to a SQLX query builder for an update query.
    ///
    /// # Arguments
    /// * `query_builder` - Query builder to be populated.
    /// * `list` - List of data objects to be bounded to the query.
    /// * `as_suffix` - Toggle to add a `as xxx` suffix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct Foo {}
    ///
    /// impl SqlxPgInsertable for Foo {}
    ///
    /// let foos = vec![
    ///     Foo {},
    ///     Foo {},
    /// ];
    ///
    /// Foo::bind_unnest_values(&mut query_builder, &foos, false);
    /// ```
    fn bind_unnest_values(
        query_builder: &mut sqlx::QueryBuilder<sqlx::postgres::Postgres>,
        list: &[Self],
        as_suffix: bool,
    ) where
        Self: Sized;
}
