use query::{impl_id, impl_table};

use crate::c::*;

use super::user::UserId;

impl_id!(CredentialsId, "credentials");
impl_table!(Credentials, "credentials");

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub id: CredentialsId,
    pub uid: UserId,
    pub login: String,
    pub password: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Credentials {
    pub fn new(
        id: impl Into<CredentialsId>,
        uid: impl Into<UserId>,
        login: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            uid: uid.into(),
            login: login.into(),
            password: password.into(),
            created_at: now(),
            updated_at: now(),
        }
    }
    pub fn admin() -> Self {
        Self::new(
            CredentialsId::new("1"),
            UserId::new("1"),
            "a@a".to_string(),
            "a".to_string(),
        )
    }
}
