use crate::model::user::UserId;
use crate::{components::app::Ctx, model::session::Session};
use crate::{MyError, R};
use anyhow::Context;
use leptos::{logging::warn, ServerFnError, SignalGetUntracked};
use surrealsdk::*;

pub async fn session_by_token(token: &str) -> R<Option<Session>> {
    o(select_all::<Session>()
        .q("WHERE session_token = $session_token")
        .bind("session_token", token))
    .await
}

pub async fn session_create_by_user_id(user_id: UserId) -> R<Session> {
    let s = o::<Session>(create(Session::from_user_id(user_id))).await?;
    Ok(s.context("session not created")?)
}

pub async fn session() -> crate::SR<Session> {
    let token = Ctx::cx()
        .auth
        .get_untracked()
        .ok_or::<ServerFnError<MyError>>(
            crate::MyError::from(anyhow::anyhow!("Session not found")).into(),
        )?;
    let session = match session_by_token(&token).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            let (_, set_auth) =
                leptos_use::use_cookie::<String, codee::string::FromToStringCodec>("auth");
            set_auth(None);
            Ctx::cx().auth.set(None);
            warn!("Invalid session redirecting to /auth");
            return Err(crate::MyError::from(anyhow::anyhow!("redirect_auth")).into());
        }
        Err(e) => {
            return Err(crate::MyError::from(e).into());
        }
    };

    Ok(session)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::R;

    #[tokio::test]
    async fn test_create_and_get_session() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        let session = session_create_by_user_id(UserId::new("1")).await?;
        let result_session = session_by_token(&session.session_token)
            .await?
            .ok_or(anyhow::anyhow!("No session"))?;
        assert_eq!(session, result_session);
        Ok(())
    }

    #[tokio::test]
    async fn test_session_by_token() -> R {
        surrealsdk::init();
        surrealsdk::connect("ws://localhost:8000", "test", "test").await?;
        let result_session = session_by_token("x7vrGse6wsSUVy0IczMwk")
            .await?
            .ok_or(anyhow::anyhow!("No session"))?;
        println!("{:?}", result_session);
        // assert_eq!(session, result_session);
        Ok(())
    }
}
