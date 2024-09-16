pub mod mock;
mod surreal;
use std::future::Future;
use std::sync::Arc;

use leptos::{logging::warn, ServerFnError, SignalGetUntracked};
use tokio::sync::RwLock;

use super::session_store;
use crate::{c::*, MyError, R};
use crate::{components::app::Ctx, model::session::Session};

pub trait SessionDB {
    fn by_token(&self, token: &str) -> impl Send + Future<Output = R<Option<Session>>>;
    fn create_by_user_id(&self, user_id: IDRef) -> impl Send + Future<Output = R<Session>>;
}

pub type SessionStore = surreal::SessionStoreSurreal;

pub async fn session() -> crate::SR<Session> {
    let token = Ctx::cx()
        .auth
        .get_untracked()
        .ok_or::<ServerFnError<MyError>>(
            crate::MyError::from(anyhow::anyhow!("Session not found")).into(),
        )?;
    let sessions = session_store();
    let session = match sessions.by_token(&token).await {
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
