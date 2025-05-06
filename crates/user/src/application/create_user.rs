//! Use-case for creating a user.

use common_core::UseCase;

use crate::domain::port::UserStore;
use crate::domain::user::{CreateUserRequest, User, UserData};
use crate::prelude::*;

/// Stores used by this use-case.
#[derive(Clone)]
pub struct CreateUserStores {
    /// User store.
    pub user: Arc<dyn UserStore>,
}

/// User creation use-case structure.
pub struct CreateUser {
    /// List of stores used.
    stores: CreateUserStores,
}

impl CreateUser {
    /// Creates a new `CreateUser` use-case instance.
    ///
    /// # Arguments
    /// * `stores`: List of stores used by this use-case.
    ///
    /// # Returns
    /// A `CreateUser` instance.
    pub fn new(stores: CreateUserStores) -> Self {
        Self { stores }
    }
}

impl UseCase for CreateUser {
    type Args = CreateUserRequest;
    type Output = User;
    type Error = Error;

    async fn handle(&self, request: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = UserData {
            password: request.password.hashed()?,
            ..request.into()
        };

        self.stores.user.create(data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use security::password::{set_checks, Checks};
    use test_utils::rand::*;

    use crate::domain::port::MockUserStore;
    use crate::domain::user::UserRole;

    #[tokio::test]
    async fn test_create_user_nominal() {
        set_checks(Checks::default());

        let mut repo_user = MockUserStore::new();

        repo_user
            .expect_create()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(User::default()) }));

        let stores = CreateUserStores {
            user: Arc::new(repo_user),
        };

        let res = CreateUser::new(stores.clone())
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
