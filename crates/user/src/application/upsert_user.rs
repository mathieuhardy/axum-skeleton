//! Use-case for upserting a user.

use chrono::Duration;

use auth::AuthStore;
use common_core::UseCase;
use configuration::Config;
use mailer::MailerProvider;

use crate::domain::port::UserStore;
use crate::domain::user::{UpsertUserRequest, User, UserData};
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct UpsertUserStores {
    /// User store.
    pub user: Arc<dyn UserStore>,

    /// Mailer provider.
    pub mailer: Arc<dyn MailerProvider>,

    /// Auth store.
    pub auth: Arc<dyn AuthStore>,
}

/// User creation/update use-case structure.
pub struct UpsertUser {
    /// Application configuration.
    config: Config,

    /// List of stores used.
    stores: UpsertUserStores,
}

impl UpsertUser {
    /// Creates a new `UpsertUser` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `UpsertUser` instance.
    pub fn new(config: Config, stores: UpsertUserStores) -> Self {
        Self { config, stores }
    }
}

impl UseCase for UpsertUser {
    type Args = UpsertUserRequest;
    type Output = User;
    type Error = Error;

    async fn handle(&self, request: Self::Args) -> Result<Self::Output, Self::Error> {
        match request.user_id {
            Some(user_id) => self.stores.user.update(user_id, request.into()).await,

            None => {
                // User creation
                let password = request.password.as_ref().ok_or(Error::MissingPassword)?;

                let data = UserData {
                    password: password.hashed()?,
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use auth::{AuthUserConfirmation, MockAuthStore};
    use configuration::Config;
    use mailer::MockMailerProvider;
    use security::password::{set_checks, Checks, Password};
    use test_utils::rand::{random_email, random_id, random_string};

    use crate::domain::port::MockUserStore;
    use crate::domain::user::{UpdateUserRequest, UserRole};

    #[tokio::test]
    async fn test_upsert_user_create_nominal() -> Result<(), Box<dyn std::error::Error>> {
        dotenvy::dotenv()?;

        set_checks(Checks::default());

        let mut user_store = MockUserStore::new();
        let mut mailer = MockMailerProvider::new();
        let mut auth_store = MockAuthStore::new();

        user_store
            .expect_create()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(User::default()) }));

        auth_store
            .expect_create_user_confirmation()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(AuthUserConfirmation::default()) }));

        mailer
            .expect_send_email_confirmation()
            .times(1)
            .returning(move |_, _, _| Box::pin(async move { Ok(()) }));

        let config = Config::new()?;

        let stores = UpsertUserStores {
            user: Arc::new(user_store),
            mailer: Arc::new(mailer),
            auth: Arc::new(auth_store),
        };

        let res = UpsertUser::new(config, stores.clone())
            .handle(UpsertUserRequest {
                password: Some(Password::default()),
                user: UpdateUserRequest {
                    first_name: random_string(),
                    last_name: random_string(),
                    email: random_email(),
                    role: UserRole::Admin,
                },
                ..Default::default()
            })
            .await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_user_update_nominal() -> Result<(), Box<dyn std::error::Error>> {
        set_checks(Checks::default());

        let mut user_store = MockUserStore::new();
        let mailer = MockMailerProvider::new();
        let auth_store = MockAuthStore::new();

        user_store
            .expect_update()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(User::default()) }));

        let config = Config::new()?;

        let stores = UpsertUserStores {
            user: Arc::new(user_store),
            mailer: Arc::new(mailer),
            auth: Arc::new(auth_store),
        };

        let user_id = random_id();

        let res = UpsertUser::new(config, stores.clone())
            .handle(UpsertUserRequest {
                user_id: Some(user_id),
                password: Some(Password::default()),
                user: UpdateUserRequest {
                    first_name: random_string(),
                    last_name: random_string(),
                    email: random_email(),
                    role: UserRole::Admin,
                },
            })
            .await;
        assert!(res.is_ok());

        Ok(())
    }
}
