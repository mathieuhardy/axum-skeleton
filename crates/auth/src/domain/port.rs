//! Repository port for the authentication module

use futures::future::BoxFuture;

use crate::domain::auth_user::AuthUser;
use crate::prelude::*;

/// Authentication repository APIs.
#[cfg_attr(test, mockall::automock)]
pub trait AuthRepository: Send + Sync {
    /// Find a user by its email.
    ///
    /// # Arguments
    /// * `email`: User's email.
    ///
    /// # Returns
    /// A result containing the user if found, or an error.
    fn find_user_by_email(&self, email: String) -> BoxFuture<'static, Result<AuthUser, Error>>;

    /// Find a user by its ID.
    ///
    /// # Arguments
    /// * `user_id`: User's ID.
    ///
    /// # Returns
    /// A result containing the user if found, or an error.
    fn get_user_by_id(&self, user_id: Uuid) -> BoxFuture<'static, Result<AuthUser, Error>>;
}
