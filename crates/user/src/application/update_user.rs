//! Use-case for updating a user.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::{UpdateUserRequest, User};
use crate::prelude::*;

/// Stores used by this use-case.
pub(crate) struct UpdateUserStores<A>
where
    A: UserStore,
{
    /// User store.
    pub user: A,
}

/// User update use-case structure.
pub(crate) struct UpdateUser<A>
where
    A: UserStore,
{
    /// List of stores used.
    stores: UpdateUserStores<A>,
}

impl<A> UpdateUser<A>
where
    A: UserStore,
{
    /// Creates a new `UpdateUser` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `UpdateUser` instance.
    pub fn new(stores: UpdateUserStores<A>) -> Self {
        Self { stores }
    }
}

impl<A> UseCase for UpdateUser<A>
where
    A: UserStore,
{
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

        let stores = UpdateUserStores { user: user_store };

        let user_id = random_id();

        let res = UpdateUser::new(stores)
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
