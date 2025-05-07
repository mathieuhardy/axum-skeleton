//! SQLx implementation of the UserStore trait.

use futures::future::BoxFuture;
use uuid::Uuid;

use crate::domain::port::MailerProvider;
use crate::prelude::*;

/// Fake mailer implementation (prints to the console).
/// Don't use this in production!
#[derive(Default)]
pub struct FakeMailer;

impl FakeMailer {
    /// Creates a new `FakeMailer` instance.
    pub fn new() -> Self {
        Self
    }
}

impl MailerProvider for FakeMailer {
    fn send_email_confirmation(
        &self,
        email: &str,
        token: &Uuid,
        redirect_url: &str,
    ) -> BoxFuture<'static, ApiResult<()>> {
        println!("Sending email confirmation to {email} with url {redirect_url}?token={token}");

        Box::pin(async move { Ok(()) })
    }
}
