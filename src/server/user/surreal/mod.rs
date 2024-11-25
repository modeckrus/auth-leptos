#[cfg(test)]
mod test;
use std::future::Future;

use anyhow::Context;
use surrealsdk::*;

use crate::R;

use super::{Timestamp, UserDB};
#[derive(Debug, Clone)]
pub struct UserStoreSurreal {}

impl_id!(UserId, "user");
impl_table!(SurrealUser, "user");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SurrealUser {
    pub id: UserId,
    pub display_name: String,
    pub login: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<SurrealUser> for crate::model::user::User {
    fn from(value: SurrealUser) -> Self {
        Self {
            id: value.id.0,
            display_name: value.display_name,
            login: value.login,
            password: value.password,
            avatar_url: value.avatar_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<crate::model::user::User> for SurrealUser {
    fn from(value: crate::model::user::User) -> Self {
        Self {
            id: UserId::new(value.id),
            display_name: value.display_name,
            login: value.login,
            password: value.password,
            avatar_url: value.avatar_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl UserDB for UserStoreSurreal {
    fn by_id(
        &self,
        id: super::IDRef,
    ) -> impl Future<Output = R<Option<crate::model::user::User>>> + Send {
        async move {
            let uid = UserId::new(id);
            let result: Option<SurrealUser> = o(uid.q()).await?;
            Ok(result.map(crate::model::user::User::from))
        }
    }

    fn by_login(
        &self,
        login: &str,
    ) -> impl Future<Output = R<Option<crate::model::user::User>>> + Send {
        async move {
            let result: Option<SurrealUser> = o(select_all::<SurrealUser>()
                .q("WHERE login = $login")
                .bind("login", login))
            .await?;
            Ok(result.map(crate::model::user::User::from))
        }
    }

    fn create_user(
        &self,
        user: crate::model::user::User,
    ) -> impl Future<Output = R<crate::model::user::User>> + Send {
        async move {
            let i: SurrealUser = user.into();
            let result: Option<SurrealUser> = o(create(i)).await?;
            Ok(result.context("user not created")?.into())
        }
    }
}
