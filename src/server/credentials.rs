use anyhow::Context;
use surrealsdk::*;

use crate::{
    model::{
        credential::Credentials,
        user::{User, UserId},
    },
    R,
};

use super::surreal::upsert;

pub async fn credentials_by_login(login: impl Into<String>) -> R<Option<Credentials>> {
    o(select_all::<Credentials>()
        .q("WHERE login = $login")
        .bind("login", login.into()))
    .await
}

pub async fn credentials_by_uid(uid: impl Into<UserId>) -> R<Option<Credentials>> {
    o(select_all::<Credentials>()
        .q("WHERE uid = $uid")
        .bind("uid", uid.into()))
    .await
}

pub async fn credentials_upsert(credentials: Credentials) -> R<Credentials> {
    let mut sql = Query::default();
    let condition = "WHERE login = $login"
        .q()
        .bind("login", credentials.login.clone());
    let content = "$credentials".q().bind("credentials", credentials);
    upsert::<Credentials>(&mut sql, condition, content);
    o::<Credentials>(sql)
        .await?
        .context("failed to upsert credentials")
}

pub async fn ensure_user_with_credentials(user: User, credentials: Credentials) -> R<()> {
    let mut sql = transaction();
    let uid = user.id.clone();
    let user_condition = "WHERE id = $uid".q().bind("uid", uid.clone());
    let user_content = "$user".q().bind("user", user);
    upsert::<User>(&mut sql, user_condition, user_content);
    let credentials_condition = "WHERE login = $login AND uid = $uid"
        .q()
        .bind("login", credentials.login.clone())
        .bind("uid", uid.clone());
    let credentials_content = "$credentials".q().bind("credentials", credentials);

    upsert::<Credentials>(&mut sql, credentials_condition, credentials_content);
    sql.m("COMMIT TRANSACTION;\n");
    e(sql.commit()).await?.check()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        c::now,
        model::{credential::CredentialsId, user::User},
        R,
    };
    #[tokio::test]
    async fn test_ensure_user_with_credentials() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        // e(format!("DEFINE TABLE {}", User::table()))
        //     .await?
        //     .check()?;
        // e(format!("DEFINE TABLE {}", Credentials::table()))
        //     .await?
        //     .check()?;
        let uid = UserId::new("1");
        let user = User {
            id: uid.clone(),
            display_name: "Admin".to_string(),
            avatar_url: None,
        };
        let credentials = Credentials {
            id: CredentialsId::new("1"),
            uid: uid.clone(),
            login: "a@a.a".to_string(),
            password: "a".to_string(),
            created_at: now(),
            updated_at: now(),
        };
        ensure_user_with_credentials(user, credentials.clone()).await?;
        let by_login = credentials_by_login("a@a.a")
            .await?
            .context("credentials_by_login return None")?;
        assert_eq!(credentials.login, by_login.login);
        assert_eq!(credentials.password, by_login.password);
        let by_id = credentials_by_uid(uid.clone())
            .await?
            .context("credentials_by_uid return None")?;
        assert_eq!(credentials.login, by_id.login);
        assert_eq!(credentials.password, by_id.password);
        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_credentials() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        let credentials = Credentials {
            id: CredentialsId::new("1"),
            uid: UserId::new("1"),
            login: "a@a.a".to_string(),
            password: "a".to_string(),
            created_at: now(),
            updated_at: now(),
        };
        let credentials = credentials_upsert(credentials).await?;
        dbg!(credentials);
        Ok(())
    }
}
