//! Use-case for creating a user.

use chrono::Duration;

use auth::AuthStore;
use common_core::UseCase;
use configuration::Config;
use mailer::MailerProvider;

use crate::domain::port::UserStore;
use crate::domain::user::{CreateUserRequest, User, UserData};
use crate::prelude::*;

/// Stores used by this use-case.
pub(crate) struct CreateUserStores<A, B, C>
where
    A: UserStore,
    B: MailerProvider,
    C: AuthStore,
{
    /// User store.
    pub user: A,

    /// Mailer provider.
    pub mailer: B,

    /// Auth store.
    pub auth: C,
}

/// User creation use-case structure.
pub(crate) struct CreateUser<A, B, C>
where
    A: UserStore,
    B: MailerProvider,
    C: AuthStore,
{
    /// Application configuration.
    config: Config,

    /// List of stores used.
    stores: CreateUserStores<A, B, C>,
}

impl<A, B, C> CreateUser<A, B, C>
where
    A: UserStore,
    B: MailerProvider,
    C: AuthStore,
{
    /// Creates a new `CreateUser` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `CreateUser` instance.
    pub fn new(config: Config, stores: CreateUserStores<A, B, C>) -> Self {
        Self { config, stores }
    }
}

impl<A, B, C> UseCase for CreateUser<A, B, C>
where
    A: UserStore,
    B: MailerProvider,
    C: AuthStore,
{
    type Args = CreateUserRequest;
    type Output = User;
    type Error = Error;

    // TODO: to be done in the same transaction
    async fn handle(&self, request: Self::Args) -> Result<Self::Output, Self::Error> {
        // User creation
        let data = UserData {
            password: request.password.hashed()?,
            ..request.into()
        };

        let mut user = self.stores.user.create(data).await?;

        // Create user confirmation
        let confirmation_timeout_hours =
            Duration::hours(self.config.auth.email_confirmation_timeout_hours.into());

        let confirmation = self
            .stores
            .auth
            .create_user_confirmation(&user.id, &confirmation_timeout_hours)
            .await?;

        user.pending_confirmation = Some(confirmation.clone());

        // Send the email confirmation
        let redirect_url = std::env::var("FRONTEND_URL")?;

        self.stores
            .mailer
            .send_email_confirmation(&user.email, &confirmation.id, &redirect_url)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use auth::{AuthUserConfirmation, MockAuthStore};
    use configuration::Config;
    use mailer::MockMailerProvider;
    use security::password::{set_checks, Checks};
    use test_utils::rand::*;

    use crate::domain::port::MockUserStore;
    use crate::domain::user::UserRole;

    #[tokio::test]
    async fn test_create_user_nominal() -> Result<(), Box<dyn std::error::Error>> {
        dotenvy::dotenv()?;

        set_checks(Checks::default());

        let mut user_store = MockUserStore::new();
        let mut mailer = MockMailerProvider::new();
        let mut auth_store = MockAuthStore::new();

        user_store.expect_create().times(1).returning(move |_| {
            Box::pin(async move {
                let user = User {
                    pending_confirmation: Some(AuthUserConfirmation::default()),
                    ..Default::default()
                };

                Ok(user)
            })
        });

        auth_store
            .expect_create_user_confirmation()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(AuthUserConfirmation::default()) }));

        mailer
            .expect_send_email_confirmation()
            .times(1)
            .returning(move |_, _, _| Box::pin(async move { Ok(()) }));

        let config = Config::new()?;

        let stores = CreateUserStores {
            user: user_store,
            mailer,
            auth: auth_store,
        };

        let res = CreateUser::new(config, stores)
            .handle(CreateUserRequest {
                first_name: random_string(),
                last_name: random_string(),
                email: random_email(),
                role: UserRole::Admin,
                password: random_password(),
            })
            .await;
        assert!(res.is_ok());

        Ok(())
    }
}
