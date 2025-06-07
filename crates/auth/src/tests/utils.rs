//! All utilities needed to implement tests in this crate.

use database::Db;

use crate::domain::auth_user::AuthUser;
use crate::infrastructure::{DbAuthUser, DbAuthUserRole};

/// Creates a user entry in database from a struct `AuthUser`.
///
/// # Arguments
/// * `user` - User structure to be used for creation.
/// * `db` - Database handle.
///
/// # Returns
/// A result containing the created user as `AuthUser`.
pub async fn create_user(user: &AuthUser, db: &Db) -> Result<AuthUser, Box<dyn std::error::Error>> {
    let role: DbAuthUserRole = user.role.clone().into();
    let password = user.password.hashed()?;

    let user = sqlx::query_as!(
        DbAuthUser,
        "
        INSERT INTO users (email, role, password)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            email,
            role AS \"role!: _\",
            password,
            TRUE AS \"email_confirmed!: _\"",
        user.email.clone(),
        role as DbAuthUserRole,
        password.as_str(),
    )
    .fetch_one(&db.0)
    .await?;

    Ok(user.into())
}
