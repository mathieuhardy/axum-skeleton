//! Use-case for updating a user.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::{UpdateUserRequest, User};
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct UpdateUserStores {
    /// User store.
    pub user: Arc<dyn UserStore>,
}

/// User update use-case structure.
pub struct UpdateUser {
    /// List of stores used.
    stores: UpdateUserStores,
}

impl UpdateUser {
    /// Creates a new `UpdateUser` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `UpdateUser` instance.
    pub fn new(stores: UpdateUserStores) -> Self {
        Self { stores }
    }
}

impl UseCase for UpdateUser {
    type Args = (Uuid, UpdateUserRequest);
    type Output = User;
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (user_id, request) = args;

        self.stores.user.update(user_id, request.into()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::{random_email, random_id, random_string};

    use crate::domain::port::MockUserStore;
    use crate::domain::user::UserRole;

    #[tokio::test]
    async fn test_upsert_user_update_nominal() {
        set_checks(Checks::default());

        let mut user_store = MockUserStore::new();

        user_store
            .expect_update()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(User::default()) }));

        let stores = UpdateUserStores {
            user: Arc::new(user_store),
        };

        let user_id = random_id();

        let res = UpdateUser::new(stores.clone())
            .handle((
                user_id,
                UpdateUserRequest {
                    first_name: random_string(),
                    last_name: random_string(),
                    email: random_email(),
                    role: UserRole::Admin,
                },
            ))
            .await;
        assert!(res.is_ok());
    }
}
