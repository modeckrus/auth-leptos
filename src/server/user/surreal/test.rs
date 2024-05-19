use crate::{model::user::User, R};

use super::*;
use surrealdb::{engine::remote::ws::Ws, *};

#[tokio::test]
async fn test_create_and_get_user() -> R {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    client.use_ns("test").use_db("test").await?;
    let store = UserStoreSurreal::new(client);
    let user = User::login_pass("test", "test");
    let res = store.create_user(user.clone()).await?;
    assert_eq!(user, res);
    Ok(())
}
#[tokio::test]
async fn test_by_id() -> R {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    client.use_ns("test").use_db("test").await?;
    let store = UserStoreSurreal::new(client);
    let user = store
        .by_id("1")
        .await?
        .ok_or(anyhow::anyhow!("User not found"))?;
    println!("User: {:?}", user);
    Ok(())
}
