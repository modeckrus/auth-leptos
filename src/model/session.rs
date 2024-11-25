use std::ops::Add;

use crate::c::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Session {
    pub id: ID,
    pub user_id: ID,
    pub session_token: String,
    pub last_used_at: Timestamp,
    pub expire_at: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Session {
    pub fn new(
        id: impl Into<ID>,
        user_id: impl Into<ID>,
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

    pub fn from_user_id(user_id: impl Into<ID>) -> Self {
        Self::new(
            make_id(),
            user_id.into(),
            make_id(),
            now(),
            now().add(std::time::Duration::from_secs(60 * 60 * 24 * 7)),
        )
    }
}
