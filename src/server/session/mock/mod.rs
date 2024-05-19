use std::{
    future::Future,
    sync::{Arc, RwLock},
};

use crate::model::session::Session;

use super::{now, IDRef};

#[derive(Clone, Debug)]
pub struct SessionStoreMock {
    pub sessions: Arc<RwLock<Vec<Session>>>,
}

impl Default for SessionStoreMock {
    fn default() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl super::SessionDB for SessionStoreMock {
    fn by_token(&self, token: &str) -> impl Send + Future<Output = crate::R<Option<Session>>> {
        async move {
            Ok(self
                .sessions
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to get lock"))?
                .iter()
                .find(|s| s.session_token == token)
                .cloned())
        }
    }

    fn create_by_user_id(&self, user_id: IDRef) -> impl Send + Future<Output = crate::R<Session>> {
        async move {
            let session = Session::from_user_id(user_id);
            self.sessions
                .write()
                .map_err(|_| anyhow::anyhow!("Failed to get lock"))?
                .push(session.clone());
            Ok(session)
        }
    }
    // async fn by_token(&self, token: &str) -> Option<Session> {
    //     self.sessions
    //         .read()
    //         .await
    //         .iter()
    //         .find(|s| s.token == token)
    //         .cloned()
    // }

    // async fn create_by_user_id(&self, user_id: IDRef<'_>) -> Session {
    //     let session = Session::from_user_id(user_id);
    //     self.sessions.write().await.push(session.clone());
    //     session
    // }
}
