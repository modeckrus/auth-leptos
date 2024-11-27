use std::ops::Add;

use query::{impl_id, impl_table};

use crate::c::*;

use super::user::UserId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub session_token: String,
    pub last_used_at: Timestamp,
    pub expire_at: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
impl_id!(SessionId, "session");
impl_table!(Session, "session");

impl Session {
    pub fn new(
        id: impl Into<SessionId>,
        user_id: impl Into<UserId>,
        token: impl Into<String>,
        last_used_at: Timestamp,
        expire_at: Timestamp,
    ) -> Self {
        Self {
            id: id.into(),
            user_id: user_id.into(),
            session_token: token.into(),
            last_used_at,
            expire_at,
            created_at: now(),
            updated_at: now(),
        }
    }

    pub fn from_user_id(user_id: impl Into<UserId>) -> Self {
        Self::new(
            SessionId::none(),
            user_id.into(),
            nanoid::nanoid!(),
            now(),
            now().add(std::time::Duration::from_secs(60 * 60 * 24 * 7)),
        )
    }
}
