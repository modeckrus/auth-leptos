use leptos::logging::*;
use leptos::*;
use leptos_router::ActionForm;

use crate::components::app::Ctx;
use crate::components::helper::redirect::redirect;
use crate::model::session::Session;
use crate::notification_center::NotificationCenter;
use crate::MyError;

#[component]
pub fn AuthorizePage() -> impl IntoView {
    let login_action = create_server_action::<Login>();
    let value = login_action.value();
    let auth = Ctx::cx().auth;
    let nc = use_context::<NotificationCenter>();
    //take leptos path and extract redirect_url

    let redirect_component = move || {
        let d = view! { <p>"Waiting Login..."</p> }.into_view();
        match value() {
            Some(Ok(Ok(session))) => {
                leptos::logging::warn!("Login success: {:?}", session);
                auth.set(Some(session.session_token));
                let path = leptos_router::use_location();
                let query = path.query.get();
                let Some(redirect_url) = query.get("redirect_url").cloned() else {
                    leptos::logging::warn!("No redirect_url in query");
                    return d;
                };
                leptos::logging::warn!("Redirecting to {}", redirect_url);
                redirect(redirect_url.clone(), None);
                // return view! { <leptos_router::Redirect path=redirect_url></leptos_router::Redirect> }.into_view();
                return view! {
                    <div>
                        <p>"Redirecting to "</p>
                        <a href=redirect_url.clone()>{redirect_url.clone()}</a>
                    </div>
                }
                .into_view();
            }
            Some(Ok(Err(e))) => {
                warn!("Login failed: {}", e);
                nc.map(|nc| nc.error(e.to_string()));
            }
            _ => {}
        }
        return d;
    };

    // create_effect(move |_| match value() {
    //     Some(Ok(Ok(session))) => {
    //         auth.set(Some(session.session_token));
    //     }
    //     Some(Ok(Err(e))) => {
    //         warn!("Login failed: {}", e);
    //         nc.map(|nc| nc.error(e.to_string()));
    //     }
    //     _ => {}
    // });
    view! {
        {move || redirect_component()}
        <div class="hero">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <div class="text-center lg:text-left">
                    <h1 class="text-5xl font-bold">"Login now!"</h1>
                    <p class="py-6">
                        "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."
                    </p>
                </div>
                <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                    <ActionForm action=login_action class="card-body">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Email"</span>
                            </label>
                            <input
                                name="login"
                                type="text"
                                placeholder="login"
                                class="input input-bordered"
                                value="a@a.a"
                                required
                            />
                        </div>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Password"</span>
                            </label>
                            <input
                                name="password"
                                type="password"
                                placeholder="password"
                                class="input input-bordered"
                                value="a"
                                required
                            />
                        // <label class="label">
                        // <a href="#" class="label-text-alt link link-hover">
                        // "Forgot password?"
                        // </a>
                        // </label>
                        </div>
                        <div class="form-control mt-6">
                            <button class="btn btn-primary">"Login"</button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, Copy)]
pub enum LoginError {
    #[error("Wrong credentials")]
    WrongCredentials,
}

#[server(Login, "/api")]
pub async fn login(
    login: String,
    password: String,
) -> Result<Result<Session, LoginError>, ServerFnError<MyError>> {
    let user = crate::server::user::user_by_login(&login)
        .await
        .map_err(|e| crate::MyError::from(e))?
        .ok_or(ServerFnError::WrappedServerError(MyError::from(
            anyhow::anyhow!("user not found"),
        )))?;

    if user.password != password {
        return Ok(Err(LoginError::WrongCredentials));
    }
    let session = crate::server::session::session_create_by_user_id(user.id)
        .await
        .map_err(|e| crate::MyError::from(e))?;
    crate::components::auth::save_auth_to_cookie(session.session_token.clone());
    Ok(Ok(session))
}
