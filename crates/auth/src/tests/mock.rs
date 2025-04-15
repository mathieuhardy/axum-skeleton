use axum_login::{AuthnBackend, UserId};
use futures::future::BoxFuture;

use crate::domain::auth_user::{AuthCredentials, AuthUser};
use crate::domain::port::AuthRepository;
use crate::prelude::*;

#[derive(Clone)]
struct MockAuthRepository {
    find_user_by_email_must_fail: bool,
    get_user_by_id_must_fail: bool,
    verify_password_must_fail: bool,
    get_user_must_fail: bool,
}

impl AuthRepository for MockAuthRepository {
    fn find_user_by_email(&self, email: String) -> BoxFuture<'static, Result<AuthUser, Error>> {
        let find_user_by_email_must_fail = self.find_user_by_email_must_fail;

        Box::pin(async move {
            if find_user_by_email_must_fail {
                Err(Error::Internal)
            } else {
                Ok(AuthUser {
                    email,
                    ..Default::default()
                })
            }
        })
    }

    fn get_user_by_id(&self, user_id: Uuid) -> BoxFuture<'static, Result<AuthUser, Error>> {
        let get_user_by_id_must_fail = self.get_user_by_id_must_fail;

        Box::pin(async move {
            if get_user_by_id_must_fail {
                Err(Error::Internal)
            } else {
                Ok(AuthUser {
                    id: user_id,
                    ..Default::default()
                })
            }
        })
    }
}

#[async_trait]
impl AuthnBackend for MockAuthRepository {
    type User = AuthUser;
    type Credentials = AuthCredentials;
    type Error = Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if self.verify_password_must_fail {
            Ok(None)
        } else {
            Ok(Some(AuthUser {
                email: credentials.email,
                password: credentials.password,
                ..Default::default()
            }))
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        if self.get_user_must_fail {
            return Err(Error::Internal);
        } else {
            Ok(Some(AuthUser {
                id: *user_id,
                ..Default::default()
            }))
        }
    }
}
