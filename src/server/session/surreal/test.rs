use crate::R;

use super::*;
use surrealdb::{engine::remote::ws::Ws, *};

#[tokio::test]
async fn test_create_and_get_session() -> R {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    client.use_ns("test").use_db("test").await?;
    let store = SessionStoreSurreal::new(client);
    let session = store.create_by_user_id("1").await?;
    let result_session = store
        .by_token(&session.session_token)
        .await?
        .ok_or(anyhow::anyhow!("No session"))?;
    assert_eq!(session, result_session);
    Ok(())
}

#[tokio::test]
async fn test_session_by_token() -> R {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    client.use_ns("test").use_db("test").await?;
    let store = SessionStoreSurreal::new(client);
    let result_session = store
        .by_token("b72cf6b3-7970-486a-802f-34e544ccffba")
        .await?
        .ok_or(anyhow::anyhow!("No session"))?;
    println!("{:?}", result_session);
    // assert_eq!(session, result_session);
    Ok(())
}
