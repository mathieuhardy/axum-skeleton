//! Use-case for creating a user.

use chrono::Duration;

use common_core::UseCase;
use configuration::Config;
use mailer::MailerProvider;

use crate::domain::port::UserStore;
use crate::domain::user::{CreateUserRequest, User, UserData};
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct CreateUserStores {
    /// User store.
    pub user: Arc<dyn UserStore>,

    /// Mailer provider.
    pub mailer: Arc<dyn MailerProvider>,
}

/// User creation use-case structure.
pub struct CreateUser {
    /// Application configuration.
    config: Config,

    /// List of stores used.
    stores: CreateUserStores,
}

impl CreateUser {
    /// Creates a new `CreateUser` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `CreateUser` instance.
    pub fn new(config: Config, stores: CreateUserStores) -> Self {
        Self { config, stores }
    }
}

impl UseCase for CreateUser {
    type Args = CreateUserRequest;
    type Output = User;
    type Error = Error;

    async fn handle(&self, request: Self::Args) -> Result<Self::Output, Self::Error> {
        // User creation
        let data = UserData {
            password: request.password.hashed()?,
            ..request.into()
        };

        let confirmation_timeout_hours =
            Duration::hours(self.config.auth.email_confirmation_timeout_hours.into());

        let user = self
            .stores
            .user
            .create(data, confirmation_timeout_hours)
            .await?;

        // Send the email confirmation
        // TODO: to be done in the same transaction as the user creation
        let redirect_url = std::env::var("FRONTEND_URL")?;

        let token = user
            .pending_confirmation
            .as_ref()
            .ok_or(Error::MissingEmailConfirmation)?
            .id;

        self.stores
            .mailer
            .send_email_confirmation(&user.email, &token, &redirect_url)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use configuration::Config;
    use mailer::MockMailerProvider;
    use security::password::{set_checks, Checks};
    use test_utils::rand::*;

    use crate::domain::port::MockUserStore;
    use crate::domain::user::UserRole;
    use crate::domain::user_confirmation::UserConfirmation;

    #[tokio::test]
    async fn test_create_user_nominal() -> Result<(), Box<dyn std::error::Error>> {
        dotenvy::dotenv()?;

        set_checks(Checks::default());

        let mut repo_user = MockUserStore::new();
        let mut mailer = MockMailerProvider::new();

        repo_user.expect_create().times(1).returning(move |_, _| {
            Box::pin(async move {
                let user = User {
                    pending_confirmation: Some(UserConfirmation::default()),
                    ..Default::default()
                };

                Ok(user)
            })
        });

        mailer
            .expect_send_email_confirmation()
            .times(1)
            .returning(move |_, _, _| Box::pin(async move { Ok(()) }));

        let config = Config::new()?;

        let stores = CreateUserStores {
            user: Arc::new(repo_user),
            mailer: Arc::new(mailer),
        };

        let res = CreateUser::new(config, stores.clone())
            .handle(CreateUserRequest {
                first_name: random_string(),
                last_name: random_string(),
                email: random_email(),
                role: UserRole::Admin,
                password: random_password(),
            })
            .await;
        println!("{res:?}");
        assert!(res.is_ok());

        Ok(())
    }
}
