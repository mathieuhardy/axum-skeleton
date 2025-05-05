//! Use-case for getting users by filters.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::{User, UserFilters};
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct GetUsersByFiltersStores {
    /// User store.
    pub user: Arc<dyn UserStore>,
}

/// User searching use-case structure.
pub struct GetUsersByFilters {
    /// List of stores used.
    stores: GetUsersByFiltersStores,
}

impl GetUsersByFilters {
    /// Creates a new `GetUsersByFilters` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `GetUsersByFilters` instance.
    pub fn new(stores: GetUsersByFiltersStores) -> Self {
        Self { stores }
    }
}

impl UseCase for GetUsersByFilters {
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
        let mut repo_user = MockUserStore::new();

        let filters = UserFilters {
            first_name: Some(random_string()),
            last_name: Some(random_string()),
            email: Some(random_string()),
            role: Some(UserRole::Guest),
        };

        repo_user
            .expect_get_by_filters()
            .times(1)
            .returning(move |filters| {
                assert_eq!(filters, filters);
                Box::pin(async move { Ok(vec![]) })
            });

        let stores = GetUsersByFiltersStores {
            user: Arc::new(repo_user),
        };

        let res = GetUsersByFilters::new(stores).handle(filters).await;
        assert!(res.is_ok());
    }
}
