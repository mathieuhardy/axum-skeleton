//! Use-case for deleting a user.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::prelude::*;

/// Stores used by this use-case.
pub(crate) struct DeleteUserByIdStores<A>
where
    A: UserStore,
{
    /// User store.
    pub user: A,
}

/// User deletion use-case structure.
pub(crate) struct DeleteUserById<A>
where
    A: UserStore,
{
    /// List of stores used.
    stores: DeleteUserByIdStores<A>,
}

impl<A> DeleteUserById<A>
where
    A: UserStore,
{
    /// Creates a new `DeleteUserById` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `DeleteUserById` instance.
    pub fn new(stores: DeleteUserByIdStores<A>) -> Self {
        Self { stores }
    }
}

impl<A> UseCase for DeleteUserById<A>
where
    A: UserStore,
{
    type Args = Uuid;
    type Output = ();
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

        self.stores.user.delete_by_id(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::rand::random_id;

    use crate::domain::port::MockUserStore;

    #[tokio::test]
    async fn test_delete_user_by_id_not_found() {
        let mut user_store = MockUserStore::new();

        let user_id = random_id();

        user_store.expect_exists().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(false) })
        });

        let stores = DeleteUserByIdStores { user: user_store };

        let res = DeleteUserById::new(stores).handle(user_id).await;
        assert!(matches!(res, Err(Error::NotFound)));
    }

    #[tokio::test]
    async fn test_delete_user_by_id_nominal() {
        let mut user_store = MockUserStore::new();

        let user_id = random_id();

        user_store.expect_exists().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(true) })
        });

        user_store
            .expect_delete_by_id()
            .times(1)
            .returning(move |id| {
                assert_eq!(id, user_id);
                Box::pin(async move { Ok(()) })
            });

        let stores = DeleteUserByIdStores { user: user_store };

        let res = DeleteUserById::new(stores).handle(user_id).await;
        assert!(res.is_ok());
    }
}
