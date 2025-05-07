//! Use-case for setting a user's password.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::PasswordUpdateRequest;
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct SetUserPasswordStores {
    /// User store.
    pub user: Arc<dyn UserStore>,
}

/// Password update use-case structure.
pub struct SetUserPassword {
    /// List of stores used.
    stores: SetUserPasswordStores,
}

impl SetUserPassword {
    /// Creates a new `SetUserPassword` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `SetUserPassword` instance.
    pub fn new(stores: SetUserPasswordStores) -> Self {
        Self { stores }
    }
}

impl UseCase for SetUserPassword {
    type Args = (Uuid, PasswordUpdateRequest);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (user_id, request) = args;

        let user = self.stores.user.get_by_id(user_id).await?;

        if !request.current.matches(&user.password).await? {
            return Err(Error::InvalidPassword);
        }

        self.stores
            .user
            .set_user_password(user_id, request.new.hashed()?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::*;

    use crate::domain::port::MockUserStore;
    use crate::domain::user::User;

    #[tokio::test]
    async fn test_set_user_password_validation() {
        set_checks(Checks::default());

        let mut user_store = MockUserStore::new();

        user_store.expect_get_by_id().times(1).returning(move |_| {
            Box::pin(async move {
                Ok(User {
                    password: random_password().hashed()?,
                    ..Default::default()
                })
            })
        });

        let stores = SetUserPasswordStores {
            user: Arc::new(user_store),
        };

        let user_id = random_id();

        // Invalid current password
        let res = SetUserPassword::new(stores.clone())
            .handle((
                user_id,
                PasswordUpdateRequest {
                    current: random_password(),
                    ..Default::default()
                },
            ))
            .await;
        assert!(matches!(res, Err(Error::InvalidPassword)));
    }

    #[tokio::test]
    async fn test_set_user_password_nominal() {
        set_checks(Checks::default());

        let mut user_store = MockUserStore::new();

        let user_id = random_id();
        let password = random_password();
        let current = password.clone();
        let new = random_password();

        user_store.expect_get_by_id().times(1).returning(move |_| {
            let password = password.clone();

            Box::pin(async move {
                Ok(User {
                    password: password.hashed()?,
                    ..Default::default()
                })
            })
        });

        user_store
            .expect_set_user_password()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(()) }));

        let stores = SetUserPasswordStores {
            user: Arc::new(user_store),
        };

        let res = SetUserPassword::new(stores.clone())
            .handle((user_id, PasswordUpdateRequest { current, new }))
            .await;
        assert!(res.is_ok());
    }
}
