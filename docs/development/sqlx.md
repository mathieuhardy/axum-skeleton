# 🗃 SQLx

> **Note**
> SQLx relies on the environment variable `DATABASE_URL`. If you want to manage
> another database, you can export the variable in your shell.

## Create/delete database

```shell
sqlx database create
sqlx database drop
```

## List available/installed migrations

```shell
sqlx migrate info --source crates/database/migrations/
```

## Add migration

```shell
sqlx migrate add --source crates/database/migrations/ <name>
```

## Run/revert

> **Warning**
> The migration should not be done manually as the application embeds the
> migrations and will try to apply them on startup.

```shell
# Run all non-installed migrations
sqlx migrate run --source crates/database/migrations/ 

# Revert last migration
sqlx migrate revert
```

## Fake data

You can insert fake data to start using the application more quickly by using
the SQL script used to populate the database during unit testing:

```shell
psql -a -d axum -f ./data/tests/populate.sql
```
