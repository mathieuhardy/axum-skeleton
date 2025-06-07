//! Use-case for user email confirmation sending.

use chrono::Duration;

use common_core::UseCase;
use configuration::Config;
use mailer::MailerProvider;

use crate::domain::auth_user::AuthUser;
use crate::domain::port::AuthStore;
use crate::prelude::*;

/// Stores used by this use-case.
pub(crate) struct SendEmailConfirmationStores<A, B>
where
    A: MailerProvider,
    B: AuthStore,
{
    /// Mailer provider.
    pub mailer: A,

    /// Auth store.
    pub auth: B,
}

/// User confirmation use-case structure.
pub(crate) struct SendEmailConfirmation<A, B>
where
    A: MailerProvider,
    B: AuthStore,
{
    /// Application configuration.
    config: Config,

    /// List of stores used.
    stores: SendEmailConfirmationStores<A, B>,
}

impl<A, B> SendEmailConfirmation<A, B>
where
    A: MailerProvider,
    B: AuthStore,
{
    /// Creates a `SendEmailConfirmation` use-case instance.
    ///
    /// # Returns
    /// A `SendEmailConfirmation` instance.
    pub fn new(config: Config, stores: SendEmailConfirmationStores<A, B>) -> Self {
        Self { config, stores }
    }
}

impl<A, B> UseCase for SendEmailConfirmation<A, B>
where
    A: MailerProvider,
    B: AuthStore,
{
    type Args = AuthUser;
    type Output = ();
    type Error = Error;

    async fn handle(&self, user: Self::Args) -> Result<Self::Output, Self::Error> {
        let confirmation_timeout_hours =
            Duration::hours(self.config.auth.email_confirmation_timeout_hours.into());

        if confirmation_timeout_hours.num_hours() > 0 {
            // Delete existing confirmation if any
            self.stores
                .auth
                .delete_user_confirmation_by_user_id(&user.id)
                .await?;

            // Create a new confirmation
            let confirmation = self
                .stores
                .auth
                .create_user_confirmation(&user.id, &confirmation_timeout_hours)
                .await?;

            // Send the email confirmation
            // TODO: to be done in the same transaction as the user creation
            let redirect_url = std::env::var("FRONTEND_URL")?;

            self.stores
                .mailer
                .send_email_confirmation(&user.email, &confirmation.id, &redirect_url)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use configuration::Config;
    use mailer::MockMailerProvider;

    use crate::domain::auth_user::AuthUserConfirmation;
    use crate::domain::port::MockAuthStore;

    #[tokio::test]
    async fn test_send_email_confirmation_nominal() -> Result<(), Box<dyn std::error::Error>> {
        dotenvy::dotenv()?;

        let mut mailer = MockMailerProvider::new();
        let mut auth_store = MockAuthStore::new();

        auth_store
            .expect_delete_user_confirmation_by_user_id()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(()) }));

        auth_store
            .expect_create_user_confirmation()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(AuthUserConfirmation::default()) }));

        mailer
            .expect_send_email_confirmation()
            .times(1)
            .returning(move |_, _, _| Box::pin(async move { Ok(()) }));

        let config = Config::new()?;

        let stores = SendEmailConfirmationStores {
            mailer,
            auth: auth_store,
        };

        let user = AuthUser::default();

        let res = SendEmailConfirmation::new(config, stores)
            .handle(user)
            .await;
        assert!(res.is_ok());

        Ok(())
    }
}
