use futures::future::BoxFuture;

use crate::domain::user::{User, UserData, UserFilters};
use crate::prelude::*;

#[cfg_attr(test, mockall::automock)]
pub trait UserRepository: Send + Sync {
    fn exists(&self, user_id: Uuid) -> BoxFuture<'static, Result<bool, Error>>;

    fn delete_by_id(&self, user_id: Uuid) -> BoxFuture<'static, Result<(), Error>>;

    fn get_by_id(&self, user_id: Uuid) -> BoxFuture<'static, Result<User, Error>>;

    fn get_by_filters(&self, filters: UserFilters) -> BoxFuture<'static, Result<Vec<User>, Error>>;

    fn create(&self, data: UserData) -> BoxFuture<'static, Result<User, Error>>;

    fn update(&self, user_id: Uuid, data: UserData) -> BoxFuture<'static, Result<User, Error>>;

    fn set_user_password(
        &self,
        user_id: Uuid,
        password: String,
    ) -> BoxFuture<'static, Result<(), Error>>;
}
