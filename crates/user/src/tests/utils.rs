//! Utilities functions for handling users in the database.

use database::SharedDb;
use test_utils::rand::{random_email, random_string};

use crate::domain::user::{User, UserRole};
use crate::infrastructure::user::{DbUser, DbUserRole};

/// Creates a new user in the database with the given role.
///
/// # Arguments
/// * `role` - The role of the user to create.
/// * `db` - Database handle.
///
/// # Returns
/// A `Result` containing the created user or an error if the user could not be created.
pub async fn create_user(
    role: UserRole,
    db: &SharedDb,
) -> Result<User, Box<dyn std::error::Error>> {
    let db = db.lock().await;

    let first_name = random_string();
    let last_name = random_string();
    let email = random_email();
    let role: DbUserRole = role.into();

    let user = sqlx::query_as!(
        DbUser,
        "
        INSERT INTO users (first_name, last_name, email, role, password)
        VALUES ($1, $2, $3, $4, '')
        RETURNING
            id,
            first_name,
            last_name,
            email,
            role AS \"role: _\",
            password,
            created_at,
            updated_at,
            NULL AS \"pending_confirmation: _\"",
        first_name,
        last_name,
        email,
        role as DbUserRole,
    )
    .fetch_one(db.clone())
    .await?;

    Ok(user.into())
}
