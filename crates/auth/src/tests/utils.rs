//! All utilities needed to implement tests in this crate.

use sqlx::postgres::PgPool;

use crate::domain::auth_user::AuthUser;
use crate::infrastructure::{DbAuthUser, DbAuthUserRole};

/// Creates a user entry in database from a struct `AuthUser`.
///
/// # Arguments
/// * `user` - User structure to be used for creation.
/// * `pool` - Database handle.
///
/// # Returns
/// A result containing the created user as `AuthUser`.
pub async fn create_user(
    user: &AuthUser,
    pool: &PgPool,
) -> Result<AuthUser, Box<dyn std::error::Error>> {
    let role: DbAuthUserRole = user.role.clone().into();

    let user = sqlx::query_as!(
        DbAuthUser,
        "
        INSERT INTO users (email, role, password)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            email,
            role AS \"role: _\",
            password",
        user.email.clone(),
        role as DbAuthUserRole,
        utils::hashing::hash_password(&user.password)?,
    )
    .fetch_one(pool)
    .await?;

    Ok(user.into())
}
