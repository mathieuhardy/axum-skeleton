//! Use-case for getting a user by ID.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::User;
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct GetUserByIdStores {
    /// User store.
    pub user: Arc<dyn UserStore>,
}

/// User fetching use-case structure.
pub struct GetUserById {
    /// List of stores used.
    stores: GetUserByIdStores,
}

impl GetUserById {
    /// Creates a new `GetUserById` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `GetUserById` instance.
    pub fn new(stores: GetUserByIdStores) -> Self {
        Self { stores }
    }
}

impl UseCase for GetUserById {
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

        let stores = GetUserByIdStores {
            user: Arc::new(user_store),
        };

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

        let stores = GetUserByIdStores {
            user: Arc::new(user_store),
        };

        let res = GetUserById::new(stores).handle(user_id).await;
        assert!(res.is_ok());
    }
}
