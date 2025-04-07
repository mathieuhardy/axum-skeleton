//! All utilities needed to implement tests in this crate.

use sqlx::postgres::PgPool;

use crate::domain::auth_user::{AuthUser, AuthUserRole};

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
    let user = sqlx::query_as!(
        AuthUser,
        "
        INSERT INTO users (email, role, password)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            email,
            role AS \"role: _\",
            password",
        user.email.clone(),
        user.role.clone() as AuthUserRole,
        utils::hashing::hash_password(&user.password)?,
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
