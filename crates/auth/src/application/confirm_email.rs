//! Use-case for user email confirmation.

use uuid::Uuid;

use common_core::UseCase;

use crate::domain::port::AuthStore;
use crate::prelude::*;

/// Stores used by this use-case.
pub(crate) struct ConfirmEmailStores<A>
where
    A: AuthStore,
{
    /// Auth store.
    pub auth: A,
}

/// User confirmation use-case structure.
pub(crate) struct ConfirmEmail<A>
where
    A: AuthStore,
{
    /// List of stores used.
    stores: ConfirmEmailStores<A>,
}

impl<A> ConfirmEmail<A>
where
    A: AuthStore,
{
    /// Creates a `ConfirmEmail` use-case instance.
    ///
    /// # Returns
    /// A `ConfirmEmail` instance.
    pub fn new(stores: ConfirmEmailStores<A>) -> Self {
        Self { stores }
    }
}

impl<A> UseCase for ConfirmEmail<A>
where
    A: AuthStore,
{
    type Args = Uuid;
    type Output = ();
    type Error = Error;

    async fn handle(&self, confirmation_id: Self::Args) -> Result<Self::Output, Self::Error> {
        let confirmation = self
            .stores
            .auth
            .get_user_confirmation_by_id(&confirmation_id)
            .await
            .map_err(|_| Error::ConfirmationNotFound)?;

        if confirmation.is_expired() {
            // Frontend is responsible to alert the user and provide a way to resend the
            // confirmation email with a new link.
            return Err(Error::ConfirmationLinkExpired);
        }

        // Delete confirmation from the store (i.e. the user will now be considered as confirmed).
        self.stores
            .auth
            .delete_user_confirmation_by_id(&confirmation_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{DateTime, Utc};

    use crate::domain::auth_user::AuthUserConfirmation;
    use crate::domain::port::MockAuthStore;

    #[tokio::test]
    async fn test_confirm_email_nominal() -> Result<(), Box<dyn std::error::Error>> {
        let mut auth_store = MockAuthStore::new();

        auth_store
            .expect_get_user_confirmation_by_id()
            .times(1)
            .returning(move |_| {
                Box::pin(async move {
                    Ok(AuthUserConfirmation {
                        expires_at: DateTime::from_timestamp(Utc::now().timestamp() + 3600, 0)
                            .unwrap(),
                        ..Default::default()
                    })
                })
            });

        auth_store
            .expect_delete_user_confirmation_by_id()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(()) }));

        let confirmation_id = Uuid::new_v4();

        let stores = ConfirmEmailStores { auth: auth_store };

        let res = ConfirmEmail::new(stores).handle(confirmation_id).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_confirm_email_expired() -> Result<(), Box<dyn std::error::Error>> {
        let mut auth_store = MockAuthStore::new();

        auth_store
            .expect_get_user_confirmation_by_id()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(AuthUserConfirmation::default()) }));

        let confirmation_id = Uuid::new_v4();

        let stores = ConfirmEmailStores { auth: auth_store };

        let res = ConfirmEmail::new(stores).handle(confirmation_id).await;
        assert!(matches!(res, Err(Error::ConfirmationLinkExpired)));

        Ok(())
    }
}
