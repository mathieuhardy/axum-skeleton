//! User store trait.

use futures::future::BoxFuture;

use crate::domain::user::{User, UserData, UserFilters};
use crate::prelude::*;

/// User store APIs.
#[cfg_attr(test, mockall::automock)]
pub trait UserStore: Send + Sync {
    /// Check if a user exists in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to check.
    ///
    /// # Returns
    /// A `Result` containing a boolean indicating if the user exists or an error if the check
    /// failed.
    fn exists(&self, user_id: Uuid) -> BoxFuture<'static, Result<bool, Error>>;

    /// Delete a user from the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to delete.
    ///
    /// # Returns
    /// A `Result` indicating if the deletion was successful or an error if it failed.
    fn delete_by_id(&self, user_id: Uuid) -> BoxFuture<'static, Result<(), Error>>;

    /// Get a user by its ID.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to get.
    ///
    /// # Returns
    /// A `Result` containing the user if found or an error if it failed.
    fn get_by_id(&self, user_id: Uuid) -> BoxFuture<'static, Result<User, Error>>;

    /// Find users using filters.
    ///
    /// # Arguments
    /// * `filters` - The filters to apply when searching for users.
    ///
    /// # Returns
    /// A `Result` containing a vector of users that match the filters or an error if it failed.
    fn get_by_filters(&self, filters: UserFilters) -> BoxFuture<'static, Result<Vec<User>, Error>>;

    /// Create a new user in the database.
    ///
    /// # Arguments
    /// * `data` - The data of the user to create.
    ///
    /// # Returns
    /// A `Result` containing the created user or an error if the creation failed.
    fn create(&self, data: UserData) -> BoxFuture<'static, Result<User, Error>>;

    /// Update an existing user in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to update.
    /// * `data` - The new data of the user.
    ///
    /// # Returns
    /// A `Result` containing the updated user or an error if the update failed.
    fn update(&self, user_id: Uuid, data: UserData) -> BoxFuture<'static, Result<User, Error>>;

    /// Update the password of an existing user in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to update.
    /// * `password` - The new password of the user.
    ///
    /// # Returns
    /// A `Result` indicating if the update was successful or an error if it failed.
    fn set_user_password(
        &self,
        user_id: Uuid,
        password: String,
    ) -> BoxFuture<'static, Result<(), Error>>;
}
