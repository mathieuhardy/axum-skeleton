//! User store trait.

use futures::future::BoxFuture;
use uuid::Uuid;

use crate::prelude::*;

/// User store APIs.
#[cfg_attr(any(test, feature = "mock"), mockall::automock)]
pub trait MailerProvider: Send + Sync {
    /// Send an email to a user in order to confirm its email (login is not possible if the user
    /// hasn't confirm).
    ///
    /// # Arguments
    /// * `email`: Email address of the user to send the confirmation email to.
    /// * `token`: Token of the confirmation.
    /// * `redirect_url`: URL to redirect the user to for email confirmation.
    ///
    /// # Returns
    /// An error or no result.
    fn send_email_confirmation(
        &self,
        email: &str,
        token: &Uuid,
        redirect_url: &str,
    ) -> BoxFuture<'static, Result<(), Error>>;
}
