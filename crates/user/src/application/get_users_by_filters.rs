use common_core::UseCase;

use crate::domain::port::UserRepository;
use crate::domain::user::{User, UserFilters};
use crate::prelude::*;

#[derive(Clone)]
pub struct GetUsersByFiltersRepos {
    pub user: Arc<dyn UserRepository>,
}

pub struct GetUsersByFilters {
    repos: GetUsersByFiltersRepos,
}

impl GetUsersByFilters {
    pub fn new(repos: GetUsersByFiltersRepos) -> Self {
        Self { repos }
    }
}

impl UseCase for GetUsersByFilters {
    type Args = UserFilters;
    type Output = Vec<User>;
    type Error = Error;

    async fn handle(&self, filters: Self::Args) -> Result<Self::Output, Self::Error> {
        self.repos.user.get_by_filters(filters).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::port::MockUserRepository;
    use crate::domain::user::UserRole;
    use test_utils::rand::random_string;

    #[tokio::test]
    async fn test_get_user_by_id_nominal() {
        let mut repo_user = MockUserRepository::new();

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

        let repos = GetUsersByFiltersRepos {
            user: Arc::new(repo_user),
        };

        let res = GetUsersByFilters::new(repos).handle(filters).await;
        assert!(res.is_ok());
    }
}
