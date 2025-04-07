use common_core::UseCase;

use crate::domain::port::UserRepository;
use crate::prelude::*;

#[derive(Clone)]
pub struct DeleteUserByIdRepos {
    pub user: Arc<dyn UserRepository>,
}

pub struct DeleteUserById {
    repos: DeleteUserByIdRepos,
}

impl DeleteUserById {
    pub fn new(repos: DeleteUserByIdRepos) -> Self {
        Self { repos }
    }
}

impl UseCase for DeleteUserById {
    type Args = Uuid;
    type Output = ();
    type Error = Error;

    async fn handle(&self, user_id: Self::Args) -> Result<Self::Output, Self::Error> {
        if !self
            .repos
            .user
            .exists(user_id)
            .await
            .map_err(|_| Error::NotFound)?
        {
            return Err(Error::NotFound);
        }

        self.repos.user.delete_by_id(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::rand::random_id;

    use crate::domain::port::MockUserRepository;

    #[tokio::test]
    async fn test_delete_user_by_id_not_found() {
        let mut repo_user = MockUserRepository::new();

        let user_id = random_id();

        repo_user.expect_exists().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(false) })
        });

        let repos = DeleteUserByIdRepos {
            user: Arc::new(repo_user),
        };

        let res = DeleteUserById::new(repos).handle(user_id).await;
        assert!(matches!(res, Err(Error::NotFound)));
    }

    #[tokio::test]
    async fn test_delete_user_by_id_nominal() {
        let mut repo_user = MockUserRepository::new();

        let user_id = random_id();

        repo_user.expect_exists().times(1).returning(move |id| {
            assert_eq!(id, user_id);
            Box::pin(async move { Ok(true) })
        });

        repo_user
            .expect_delete_by_id()
            .times(1)
            .returning(move |id| {
                assert_eq!(id, user_id);
                Box::pin(async move { Ok(()) })
            });

        let repos = DeleteUserByIdRepos {
            user: Arc::new(repo_user),
        };

        let res = DeleteUserById::new(repos).handle(user_id).await;
        assert!(res.is_ok());
    }
}
