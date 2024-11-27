use anyhow::Context;
use surrealsdk::*;

use crate::model::user::{User, UserId};
use crate::R;

pub async fn user_by_id(uid: UserId) -> R<Option<User>> {
    o(uid.q()).await
}

pub async fn user_by_login(login: &str) -> R<Option<User>> {
    o(select_all::<User>()
        .q("WHERE login = $login")
        .bind("login", login))
    .await
}

pub async fn create_user(user: User) -> R<User> {
    let user: Option<User> = o(create(user)).await?;
    Ok(user.context("user not created")?.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{model::user::User, R};

    #[tokio::test]
    async fn test_create_and_get_user() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        let user = User::login_pass("test", "test");
        let res = create_user(user.clone()).await?;
        assert_eq!(user, res);
        Ok(())
    }
    #[tokio::test]
    async fn test_by_id() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        let user = user_by_id(UserId::new("1"))
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;
        println!("User: {:?}", user);
        Ok(())
    }
}
