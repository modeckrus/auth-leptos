#[cfg(test)]
mod test;
use std::future::Future;

use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::{
    server::surreal::{id_to_thing, thing_to_id, USERDB},
    R,
};

use super::{Timestamp, UserDB};
#[derive(Debug, Clone)]
pub struct UserStoreSurreal {
    pub client: Surreal<Client>,
}

impl UserStoreSurreal {
    pub fn new(client: Surreal<Client>) -> Self {
        Self { client }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SurrealUser {
    pub id: Thing,
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
            id: thing_to_id(&value.id),
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
            id: id_to_thing(value.id, USERDB),
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
        let client = self.client.clone();
        async move {
            let sql = format!("SELECT * FROM {} WHERE id = $id", USERDB);
            let res: Option<SurrealUser> = client
                .query(&sql)
                .bind(("id", id_to_thing(id, USERDB)))
                .await
                .map_err(|e| anyhow::anyhow!(e))?
                .take(0)
                .map_err(|e| anyhow::anyhow!(e))?;
            Ok(res.map(crate::model::user::User::from))
        }
    }

    fn by_login(
        &self,
        login: &str,
    ) -> impl Future<Output = R<Option<crate::model::user::User>>> + Send {
        let client = self.client.clone();
        async move {
            let sql = format!("SELECT * FROM {} WHERE login = $login", USERDB);
            let res: Option<SurrealUser> = client
                .query(&sql)
                .bind(("login", login))
                .await
                .map_err(|e| anyhow::anyhow!(e))?
                .take(0)
                .map_err(|e| anyhow::anyhow!(e))?;
            Ok(res.map(crate::model::user::User::from))
        }
    }

    fn create_user(
        &self,
        user: crate::model::user::User,
    ) -> impl Future<Output = R<crate::model::user::User>> + Send {
        let client = self.client.clone();
        async move {
            let i: SurrealUser = user.into();
            let created: Vec<SurrealUser> = client
                .create(USERDB)
                .content(i)
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            if let Some(user) = created.into_iter().next() {
                return Ok(user.into());
            }
            Err(anyhow::anyhow!("Failed to create user"))
        }
    }
}
