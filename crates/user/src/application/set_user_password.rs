use common_core::UseCase;
use utils::hashing::{hash_password, verify};

use crate::domain::port::UserRepository;
use crate::domain::user::PasswordUpdateRequest;
use crate::prelude::*;

#[derive(Clone)]
pub struct SetUserPasswordRepos {
    pub user: Arc<dyn UserRepository>,
}

pub struct SetUserPassword {
    repos: SetUserPasswordRepos,
}

impl SetUserPassword {
    pub fn new(repos: SetUserPasswordRepos) -> Self {
        Self { repos }
    }
}

impl UseCase for SetUserPassword {
    type Args = (Uuid, PasswordUpdateRequest);
    type Output = ();
    type Error = Error;

    async fn handle(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let (user_id, request) = args;

        let user = self.repos.user.get_by_id(user_id).await?;

        if !verify(&request.current, &user.password).await? {
            return Err(Error::InvalidPassword);
        }

        let password = hash_password(&request.new)?;

        self.repos.user.set_user_password(user_id, password).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::*;

    use crate::domain::port::MockUserRepository;
    use crate::domain::user::User;

    #[tokio::test]
    async fn test_set_user_password_validation() {
        set_checks(Checks::default());

        let mut repo_user = MockUserRepository::new();

        repo_user.expect_get_by_id().times(1).returning(move |_| {
            Box::pin(async move {
                Ok(User {
                    password: hash_password(&random_password())?,
                    ..Default::default()
                })
            })
        });

        let repos = SetUserPasswordRepos {
            user: Arc::new(repo_user),
        };

        let user_id = random_id();

        // Invalid current password
        let res = SetUserPassword::new(repos.clone())
            .handle((
                user_id,
                PasswordUpdateRequest {
                    current: random_string(),
                    ..Default::default()
                },
            ))
            .await;
        assert!(matches!(res, Err(Error::InvalidPassword)));
    }

    #[tokio::test]
    async fn test_set_user_password_nominal() {
        set_checks(Checks::default());

        let mut repo_user = MockUserRepository::new();

        let user_id = random_id();
        let password = random_password();
        let current = password.clone();
        let new = random_string();

        repo_user.expect_get_by_id().times(1).returning(move |_| {
            let password = password.clone();

            Box::pin(async move {
                let hashed = hash_password(&password)?;

                Ok(User {
                    password: hashed,
                    ..Default::default()
                })
            })
        });

        repo_user
            .expect_set_user_password()
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(()) }));

        let repos = SetUserPasswordRepos {
            user: Arc::new(repo_user),
        };

        let res = SetUserPassword::new(repos.clone())
            .handle((user_id, PasswordUpdateRequest { current, new }))
            .await;
        assert!(res.is_ok());
    }
}
