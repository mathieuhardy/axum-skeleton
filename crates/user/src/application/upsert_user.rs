use common_core::UseCase;
use utils::hashing::hash_password;

use crate::domain::port::UserRepository;
use crate::domain::user::{UpsertUserRequest, User, UserData};
use crate::prelude::*;

#[derive(Clone)]
pub struct UpsertUserRepos {
    pub user: Arc<dyn UserRepository>,
}

pub struct UpsertUser {
    repos: UpsertUserRepos,
}

impl UpsertUser {
    pub fn new(repos: UpsertUserRepos) -> Self {
        Self { repos }
    }
}

impl UseCase for UpsertUser {
    type Args = UpsertUserRequest;
    type Output = User;
    type Error = Error;

    async fn handle(&self, request: Self::Args) -> Result<Self::Output, Self::Error> {
        match request.user_id {
            Some(user_id) => self.repos.user.update(user_id, request.into()).await,

            None => {
                // Creation
                let password = request.password.as_ref().ok_or(Error::MissingPassword)?;

                let data = UserData {
                    password: hash_password(password)?,
                    ..request.into()
                };

                self.repos.user.create(data).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::{random_email, random_id, random_string};

    use crate::domain::port::MockUserRepository;
    use crate::domain::user::{UpdateUserRequest, UserRole};

    #[tokio::test]
    async fn test_upsert_user_create_nominal() {
        set_checks(Checks::default());

        let mut repo_user = MockUserRepository::new();

        repo_user
            .expect_create()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(User::default()) }));

        let repos = UpsertUserRepos {
            user: Arc::new(repo_user),
        };

        let res = UpsertUser::new(repos.clone())
            .handle(UpsertUserRequest {
                password: Some("".to_string()),
                user: UpdateUserRequest {
                    first_name: random_string(),
                    last_name: random_string(),
                    email: random_email(),
                    role: UserRole::Admin,
                },
                ..Default::default()
            })
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_upsert_user_update_nominal() {
        set_checks(Checks::default());

        let mut repo_user = MockUserRepository::new();

        repo_user
            .expect_update()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(User::default()) }));

        let repos = UpsertUserRepos {
            user: Arc::new(repo_user),
        };

        let user_id = random_id();

        let res = UpsertUser::new(repos.clone())
            .handle(UpsertUserRequest {
                user_id: Some(user_id),
                password: Some(String::new()),
                user: UpdateUserRequest {
                    first_name: random_string(),
                    last_name: random_string(),
                    email: random_email(),
                    role: UserRole::Admin,
                },
            })
            .await;
        assert!(res.is_ok());
    }
}
