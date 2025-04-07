use common_core::UseCase;
use utils::hashing::hash_password;

use crate::domain::port::UserRepository;
use crate::domain::user::{CreateUserRequest, User, UserData};
use crate::prelude::*;

#[derive(Clone)]
pub struct CreateUserRepos {
    pub user: Arc<dyn UserRepository>,
}

pub struct CreateUser {
    repos: CreateUserRepos,
}

impl CreateUser {
    pub fn new(repos: CreateUserRepos) -> Self {
        Self { repos }
    }
}

impl UseCase for CreateUser {
    type Args = CreateUserRequest;
    type Output = User;
    type Error = Error;

    async fn handle(&self, request: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = UserData {
            password: hash_password(&request.password)?,
            ..request.into()
        };

        self.repos.user.create(data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::*;

    use crate::domain::port::MockUserRepository;
    use crate::domain::user::UserRole;

    #[tokio::test]
    async fn test_create_user_nominal() {
        set_checks(Checks::default());

        let mut repo_user = MockUserRepository::new();

        repo_user
            .expect_create()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(User::default()) }));

        let repos = CreateUserRepos {
            user: Arc::new(repo_user),
        };

        let res = CreateUser::new(repos.clone())
            .handle(CreateUserRequest {
                first_name: random_string(),
                last_name: random_string(),
                email: random_email(),
                role: UserRole::Admin,
                password: random_password(),
            })
            .await;
        assert!(res.is_ok());
    }
}
