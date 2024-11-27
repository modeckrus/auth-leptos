use anyhow::Context;
use surrealsdk::*;

use crate::model::user::{User, UserId};
use crate::R;

pub async fn user_by_id(uid: UserId) -> R<Option<User>> {
    o(uid.q()).await
}

pub async fn upsert_user(user: User) -> R<User> {
    let user: Option<User> = o(
        format!("UPSERT {} CONTENT $user WHERE id = $id", User::table())
            .q()
            .bind("id", user.id.clone())
            .bind("user", user),
    )
    .await?;
    Ok(user.context("user not updated")?.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{model::user::User, R};

    #[tokio::test]
    async fn test_upsert_user() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        let user = User {
            id: UserId::new("1"),
            display_name: "Admin".to_string(),
            avatar_url: None,
        };
        let user = upsert_user(user).await?;
        dbg!(user);
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
