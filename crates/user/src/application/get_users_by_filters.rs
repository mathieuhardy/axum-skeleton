//! Use-case for getting users by filters.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::{User, UserFilters};
use crate::prelude::*;

/// Stores used by this use-case.
pub struct GetUsersByFiltersStores<A>
where
    A: UserStore,
{
    /// User store.
    pub user: A,
}

/// User searching use-case structure.
pub struct GetUsersByFilters<A>
where
    A: UserStore,
{
    /// List of stores used.
    stores: GetUsersByFiltersStores<A>,
}

impl<A> GetUsersByFilters<A>
where
    A: UserStore,
{
    /// Creates a new `GetUsersByFilters` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `GetUsersByFilters` instance.
    pub fn new(stores: GetUsersByFiltersStores<A>) -> Self {
        Self { stores }
    }
}

impl<A> UseCase for GetUsersByFilters<A>
where
    A: UserStore,
{
    type Args = UserFilters;
    type Output = Vec<User>;
    type Error = Error;

    async fn handle(&self, filters: Self::Args) -> Result<Self::Output, Self::Error> {
        self.stores.user.get_by_filters(filters).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::port::MockUserStore;
    use crate::domain::user::UserRole;
    use test_utils::rand::random_string;

    #[tokio::test]
    async fn test_get_user_by_id_nominal() {
        let mut user_store = MockUserStore::new();

        let filters = UserFilters {
            first_name: Some(random_string()),
            last_name: Some(random_string()),
            email: Some(random_string()),
            role: Some(UserRole::Guest),
        };

        user_store
            .expect_get_by_filters()
            .times(1)
            .returning(move |filters| {
                assert_eq!(filters, filters);
                Box::pin(async move { Ok(vec![]) })
            });

        let stores = GetUsersByFiltersStores { user: user_store };

        let res = GetUsersByFilters::new(stores).handle(filters).await;
        assert!(res.is_ok());
    }
}
