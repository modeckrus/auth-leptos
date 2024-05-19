pub mod mock;
pub mod surreal;

use std::future::Future;

use crate::model::user::User;
use crate::{c::*, R};

pub trait UserDB {
    fn by_id(&self, id: IDRef) -> impl Future<Output = R<Option<User>>> + Send;
    fn by_login(&self, login: &str) -> impl Future<Output = R<Option<User>>> + Send;
    fn create_user(&self, user: User) -> impl Future<Output = R<User>> + Send;
}

pub type UserStore = surreal::UserStoreSurreal;
