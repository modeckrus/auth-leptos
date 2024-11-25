use crate::R;

use super::*;
use surrealsdk::*;

#[tokio::test]
async fn test_create_and_get_session() -> R {
    surrealsdk::init();
    surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
    let store = SessionStoreSurreal {};
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
    surrealsdk::init();
    surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
    let store = SessionStoreSurreal {};
    let result_session = store
        .by_token("x7vrGse6wsSUVy0IczMwk")
        .await?
        .ok_or(anyhow::anyhow!("No session"))?;
    println!("{:?}", result_session);
    // assert_eq!(session, result_session);
    Ok(())
}
