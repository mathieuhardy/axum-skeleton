//! Use-case for updating a user.

use common_core::UseCase;

use crate::domain::port::UserRepository;
use crate::domain::user::{UpdateUserRequest, User};
use crate::prelude::*;

/// Repositories used by this use-case.
#[derive(Clone)]
pub struct UpdateUserRepos {
    /// User repository.
    pub user: Arc<dyn UserRepository>,
}

/// User update use-case structure.
pub struct UpdateUser {
    /// List of repositories used.
    repos: UpdateUserRepos,
}

impl UpdateUser {
    /// Creates a new `UpdateUser` use-case instance.
    ///
    /// # Arguments
    /// * `repos`: List of repositories used by this use-case.
    ///
    /// # Returns
    /// A `UpdateUser` instance.
    pub fn new(repos: UpdateUserRepos) -> Self {
        Self { repos }
    }
}

impl UseCase for UpdateUser {
    type Args = (Uuid, UpdateUserRequest);
    type Output = User;
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (user_id, request) = args;

        self.repos.user.update(user_id, request.into()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::{random_email, random_id, random_string};

    use crate::domain::port::MockUserRepository;
    use crate::domain::user::UserRole;

    #[tokio::test]
    async fn test_upsert_user_update_nominal() {
        set_checks(Checks::default());

        let mut repo_user = MockUserRepository::new();

        repo_user
            .expect_update()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(User::default()) }));

        let repos = UpdateUserRepos {
            user: Arc::new(repo_user),
        };

        let user_id = random_id();

        let res = UpdateUser::new(repos.clone())
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
