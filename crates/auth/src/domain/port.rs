//! Store port for the authentication module

use chrono::Duration;
use futures::future::BoxFuture;

use crate::domain::auth_user::{AuthUser, AuthUserConfirmation};
use crate::prelude::*;

/// Authentication store APIs.
#[cfg_attr(any(test, feature = "mock"), mockall::automock)]
pub trait AuthStore: Send + Sync {
    /// Find a user by its email.
    ///
    /// # Arguments
    /// * `email`: User's email.
    ///
    /// # Returns
    /// A result containing the user if found, or an error.
    fn find_user_by_email(&self, email: &str) -> BoxFuture<'static, ApiResult<AuthUser>>;

    /// Find a user by its ID.
    ///
    /// # Arguments
    /// * `user_id`: User's ID.
    ///
    /// # Returns
    /// A result containing the user if found, or an error.
    fn get_user_by_id(&self, user_id: &Uuid) -> BoxFuture<'static, ApiResult<AuthUser>>;

    /// Find a user's confirmation by its ID.
    ///
    /// # Arguments
    /// * `id`: User's confirmation ID.
    ///
    /// # Returns
    /// A result containing the confirmation if found, or an error.
    fn get_user_confirmation_by_id(
        &self,
        id: &Uuid,
    ) -> BoxFuture<'static, ApiResult<AuthUserConfirmation>>;

    /// Deletes a user's confirmation by its ID.
    ///
    /// # Arguments
    /// * `id`: User's confirmation ID.
    ///
    /// # Returns
    /// An empty result.
    fn delete_user_confirmation_by_id(&self, id: &Uuid) -> BoxFuture<'static, ApiResult<()>>;

    /// Deletes a user's confirmation by its user ID.
    ///
    /// # Arguments
    /// * `user_id`: User's ID.
    ///
    /// # Returns
    /// An empty result.
    fn delete_user_confirmation_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> BoxFuture<'static, ApiResult<()>>;

    /// Creates an email confirmation entry for a user.
    ///
    /// # Arguments
    /// * `user_id`: User's ID.
    /// * `confirmation_timeout_hours`: Timeout in hours for the confirmation.
    ///
    /// # Returns
    /// A result containing the ID of the confirmation created.
    fn create_user_confirmation(
        &self,
        user_id: &Uuid,
        confirmation_timeout_hours: &Duration,
    ) -> BoxFuture<'static, ApiResult<AuthUserConfirmation>>;
}
