#[cfg(test)]
mod test;
use std::{future::Future, result};

use surrealsdk::*;

use crate::server::user::surreal::UserId;

use super::{SessionDB, Timestamp};
#[derive(Debug, Clone)]
pub struct SessionStoreSurreal {}

impl_id!(SessionId, "session");
impl_table!(SurrealSession, "session");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SurrealSession {
    pub id: SessionId,
    pub user_id: UserId,
    pub session_token: String,
    pub last_used_at: Timestamp,
    pub expire_at: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<SurrealSession> for crate::model::session::Session {
    fn from(value: SurrealSession) -> Self {
        Self {
            id: value.id.0,
            user_id: value.user_id.0,
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
            id: SessionId::new(value.id),
            user_id: UserId::new(value.user_id),
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
        async move {
            let result: Option<SurrealSession> = o(select_all::<SurrealSession>()
                .q("WHERE session_token = $session_token")
                .bind("session_token", token))
            .await?;
            Ok(result.map(crate::model::session::Session::from))
        }
    }

    fn create_by_user_id(
        &self,
        user_id: super::IDRef,
    ) -> impl Send + Future<Output = crate::R<crate::model::session::Session>> {
        let session = crate::model::session::Session::from_user_id(user_id);
        let surreal_session: SurrealSession = session.clone().into();
        async move {
            let _: Option<SurrealSession> = o(create(surreal_session)).await?;
            Ok(session)
        }
    }
}
