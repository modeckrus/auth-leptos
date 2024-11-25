use super::*;
use crate::{model::user::User, R};
use surrealsdk::*;

#[tokio::test]
async fn test_create_and_get_user() -> R {
    surrealsdk::init();
    surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
    let store = UserStoreSurreal {};
    let user = User::login_pass("test", "test");
    let res = store.create_user(user.clone()).await?;
    assert_eq!(user, res);
    Ok(())
}
#[tokio::test]
async fn test_by_id() -> R {
    surrealsdk::init();
    surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
    let store = UserStoreSurreal {};
    let user = store
        .by_id("1")
        .await?
        .ok_or(anyhow::anyhow!("User not found"))?;
    println!("User: {:?}", user);
    Ok(())
}
