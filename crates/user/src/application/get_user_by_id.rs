//! Use-case for getting a user by ID.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::User;
use crate::prelude::*;

/// Stores used by this use-case.
pub(crate) struct GetUserByIdStores<A>
where
    A: UserStore,
{
    /// User store.
    pub user: A,
}

/// User fetching use-case structure.
pub(crate) struct GetUserById<A>
where
    A: UserStore,
{
    /// List of stores used.
    stores: GetUserByIdStores<A>,
}

impl<A> GetUserById<A>
where
    A: UserStore,
{
    /// Creates a new `GetUserById` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `GetUserById` instance.
    pub fn new(stores: GetUserByIdStores<A>) -> Self {
        Self { stores }
    }
}

impl<A> UseCase for GetUserById<A>
where
    A: UserStore,
{
    type Args = Uuid;
    type Output = User;
    type Error = Error;

    async fn handle(&self, user_id: Self::Args) -> Result<Self::Output, Self::Error> {
        if !self
            .stores
            .user
            .exists(user_id)
            .await
            .map_err(|_| Error::NotFound)?
        {
            return Err(Error::NotFound);
        }

        self.stores.user.get_by_id(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::rand::random_id;

    use crate::domain::port::MockUserStore;

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        let mut user_store = MockUserStore::new();

        let user_id = random_id();

        user_store.expect_exists().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(false) })
        });

        let stores = GetUserByIdStores { user: user_store };

        let res = GetUserById::new(stores).handle(user_id).await;
        assert!(matches!(res, Err(Error::NotFound)));
    }

    #[tokio::test]
    async fn test_get_user_by_id_nominal() {
        let mut user_store = MockUserStore::new();

        let user_id = random_id();

        user_store.expect_exists().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(true) })
        });

        user_store.expect_get_by_id().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(User::default()) })
        });

        let stores = GetUserByIdStores { user: user_store };

        let res = GetUserById::new(stores).handle(user_id).await;
        assert!(res.is_ok());
    }
}
