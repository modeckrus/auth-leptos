use crate::model::user::User;
use crate::{c::*, R};
use std::future::Future;
use std::sync::{Arc, RwLock};
#[derive(Clone, Debug)]
pub struct UserStoreMock {
    pub users: Arc<RwLock<Vec<User>>>,
}

impl Default for UserStoreMock {
    fn default() -> Self {
        Self {
            users: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl super::UserDB for UserStoreMock {
    fn by_id(&self, id: IDRef) -> impl Future<Output = R<Option<User>>> + Send {
        let users = self.users.clone();
        async move {
            let users = users
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to get lock"))?;
            Ok(users.iter().find(|u| u.id == id).cloned())
        }
    }

    fn by_login(&self, login: &str) -> impl Future<Output = R<Option<User>>> + Send {
        let users = self.users.clone();
        async move {
            let users = users
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to get lock"))?;
            Ok(users.iter().find(|u| u.login == login).cloned())
        }
    }

    fn create_user(&self, user: User) -> impl Future<Output = R<User>> + Send {
        let users = self.users.clone();
        async move {
            let mut users = users
                .write()
                .map_err(|_| anyhow::anyhow!("Failed to get lock"))?;
            users.push(user.clone());
            Ok(user)
        }
    }
}
