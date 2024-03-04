//! Actions to be performed regarding the users management.

use database::models::users::{User, UserData, UserRequest};
use utils::hashing::hash_password;

use crate::prelude::*;

/// Creates a new user.
///
/// # Arguments
/// * `request` - User request used for creation.
/// * `db` - Database handle.
///
/// # Returns
/// The user inserted or an error.
pub async fn create_user(request: &UserRequest, db: &PgPool) -> Res<User> {
    let password = request.password.as_ref().ok_or(Error::InvalidPassword)?;

    let data = UserData {
        password: Some(hash_password(password)?),
        ..request.into()
    };

    User::insert(&data, db).await.map_err(Into::into)
}

/// Updates an existing user.
///
/// # Arguments
/// * `id` - User ID to be updated.
/// * `request` - User request used for creation.
/// * `db` - Database handle.
///
/// # Returns
/// The user updated or an error.
pub async fn update_user(id: &Uuid, request: &UserRequest, db: &PgPool) -> Res<User> {
    let password = match &request.password {
        Some(password) => Some(hash_password(password)?),
        None => None,
    };

    let data = UserData {
        password,
        ..request.into()
    };

    User::update_by_id(id, &data, db).await.map_err(Into::into)
}
