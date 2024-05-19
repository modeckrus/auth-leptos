use leptos::logging::*;
use leptos::*;
use std::fmt::Debug;
use std::rc::Rc;

use crate::{model::user::User, notification_center::NotificationCenter};

use super::app::Ctx;
use super::auth;

#[derive(Clone)]
pub struct UserFn(Rc<dyn Fn(User) -> View>);

impl<F, IV> From<F> for UserFn
where
    F: Fn(User) -> IV + 'static,
    IV: IntoView,
{
    fn from(value: F) -> Self {
        Self(Rc::new(move |user| value(user).into_view()))
    }
}

impl Default for UserFn {
    fn default() -> Self {
        Self(Rc::new(|_| ().into_view()))
    }
}

impl UserFn {
    fn run(&self, user: User) -> View {
        (self.0)(user)
    }
}
fn error_builder(err: impl Debug) -> View {
    view! { <div>{format!("Error: {:?}", err)}</div> }.into_view()
}

#[component]
pub fn MeBuilder(#[prop(optional, into)] f: UserFn) -> impl IntoView {
    let f = store_value(f);
    let user = create_resource(|| Ctx::cx().auth.get(), |_| async move { get_user().await });
    let loading = || view! { <div>"Loading..."</div> }.into_view();

    // Ctx::cx().auth.guard();
    let result = move || match user.get() {
        Some(Ok(Some(user))) => f.get_value().run(user),
        Some(Err(err)) => {
            use_context::<NotificationCenter>().map(|nc| {
                nc.error(err.to_string());
            });
            error_builder(err)
            // // if err.to_string().contains("redirect_auth") {
            // let script = "window.location.href = '/auth';";

            // view! { <script>{script}</script> }.into_view()
            // // } else {
            // //     error_builder(err)
            // // }
        }
        Some(Ok(None)) => {
            use leptos_use::use_cookie;
            let (cookie, set_cookie) =
                use_cookie::<String, leptos_use::utils::FromToStringCodec>("auth");
            set_cookie.set(None);
            view! { <leptos_router::Redirect path="/auth"></leptos_router::Redirect> }.into_view()
        }
        None => loading(),
    };
    view! { <Suspense fallback=loading>{move || { result() }}</Suspense> }
}

use crate::{MyError, SE};
#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError<MyError>> {
    use crate::server::user::UserDB;
    use crate::server::user_store;
    let Ok(session) = crate::server::session::session().await else {
        return Ok(None);
    };
    let users = user_store();
    let Ok(Some(user)) = users.by_id(&session.user_id).await else {
        return Ok(None);
    };
    Ok(Some(user))
}
