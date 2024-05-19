#[cfg(test)]
mod test;
use std::future::Future;

use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::server::surreal::{id_to_thing, thing_to_id, SESSIONDB, USERDB};

use super::{SessionDB, Timestamp};
#[derive(Debug, Clone)]
pub struct SessionStoreSurreal {
    pub client: Surreal<Client>,
}

impl SessionStoreSurreal {
    pub fn new(client: Surreal<Client>) -> Self {
        Self { client }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SurrealSession {
    pub id: Thing,
    pub user_id: Thing,
    pub session_token: String,
    pub last_used_at: Timestamp,
    pub expire_at: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<SurrealSession> for crate::model::session::Session {
    fn from(value: SurrealSession) -> Self {
        Self {
            id: thing_to_id(&value.id),
            user_id: thing_to_id(&value.user_id),
            session_token: value.session_token,
            last_used_at: value.last_used_at,
            expire_at: value.expire_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<crate::model::session::Session> for SurrealSession {
    fn from(value: crate::model::session::Session) -> Self {
        Self {
            id: id_to_thing(value.id, SESSIONDB),
            user_id: id_to_thing(value.user_id, USERDB),
            session_token: value.session_token,
            last_used_at: value.last_used_at,
            expire_at: value.expire_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl SessionDB for SessionStoreSurreal {
    fn by_token(
        &self,
        token: &str,
    ) -> impl Send + Future<Output = crate::R<Option<crate::model::session::Session>>> {
        let c = self.client.clone();
        async move {
            let sql = format!("SELECT * FROM {SESSIONDB} WHERE session_token = $session_token");
            let res: Option<SurrealSession> = c
                .query(sql)
                .bind(("session_token", token))
                .await
                .map_err(|err| anyhow::anyhow!(err))?
                .take(0)
                .map_err(|err| anyhow::anyhow!(err))?;
            Ok(res.map(Into::into))
        }
    }

    fn create_by_user_id(
        &self,
        user_id: super::IDRef,
    ) -> impl Send + Future<Output = crate::R<crate::model::session::Session>> {
        let session = crate::model::session::Session::from_user_id(user_id);
        let surreal_session: SurrealSession = session.clone().into();
        let c = self.client.clone();
        async move {
            let _: Vec<SurrealSession> = c.create(SESSIONDB).content(surreal_session).await?;
            Ok(session)
        }
    }
}
