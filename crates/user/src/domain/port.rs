//! User store trait.

use futures::future::BoxFuture;

use security::password::Password;

use crate::domain::user::{User, UserData, UserFilters};
use crate::prelude::*;

/// User store APIs.
#[cfg_attr(any(test, feature = "mock"), mockall::automock)]
pub trait UserStore: Send + Sync {
    /// Check if a user exists in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to check.
    ///
    /// # Returns
    /// A `ApiResult` containing a boolean indicating if the user exists or an error if the check
    /// failed.
    fn exists(&self, user_id: Uuid) -> BoxFuture<'static, ApiResult<bool>>;

    /// Delete a user from the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to delete.
    ///
    /// # Returns
    /// A `ApiResult` indicating if the deletion was successful or an error if it failed.
    fn delete_by_id(&self, user_id: Uuid) -> BoxFuture<'static, ApiResult<()>>;

    /// Get a user by its ID.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to get.
    ///
    /// # Returns
    /// A `ApiResult` containing the user if found or an error if it failed.
    fn get_by_id(&self, user_id: Uuid) -> BoxFuture<'static, ApiResult<User>>;

    /// Find users using filters.
    ///
    /// # Arguments
    /// * `filters` - The filters to apply when searching for users.
    ///
    /// # Returns
    /// A `ApiResult` containing a vector of users that match the filters or an error if it failed.
    fn get_by_filters(&self, filters: UserFilters) -> BoxFuture<'static, ApiResult<Vec<User>>>;

    /// Create a new user in the database.
    ///
    /// # Arguments
    /// * `data` - The data of the user to create.
    ///
    /// # Returns
    /// A `ApiResult` containing the created user or an error if the creation failed.
    fn create(&self, data: UserData) -> BoxFuture<'static, ApiResult<User>>;

    /// Update an existing user in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to update.
    /// * `data` - The new data of the user.
    ///
    /// # Returns
    /// A `ApiResult` containing the updated user or an error if the update failed.
    fn update(&self, user_id: Uuid, data: UserData) -> BoxFuture<'static, ApiResult<User>>;

    /// Update the password of an existing user in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to update.
    /// * `password` - The new password of the user.
    ///
    /// # Returns
    /// A `ApiResult` indicating if the update was successful or an error if it failed.
    fn set_user_password(
        &self,
        user_id: Uuid,
        password: Password,
    ) -> BoxFuture<'static, ApiResult<()>>;
}
