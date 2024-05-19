pub mod session;
pub mod surreal;
pub mod user;
use axum::body::Body as AxumBody;
use axum::{
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use http::{HeaderMap, Request};
use leptos::{provide_context, use_context, view, LeptosOptions};
use tower_cookies::Cookies;

use crate::components::app::App;
use crate::model::session::Session;

use self::session::{SessionDB, SessionStore};
use self::user::UserStore;

#[derive(Debug, Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: AppDb,
}

impl AppState {
    pub fn new(leptos_options: LeptosOptions, db: AppDb) -> Self {
        Self { leptos_options, db }
    }
}

#[derive(Debug, Clone, axum::extract::FromRef)]
pub struct AppDb {
    pub session_store: SessionStore,
    pub user_store: UserStore,
}

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    cookies: Cookies,
    path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log::info!("{:?}", path);
    leptos_axum::handle_server_fns_with_context(
        move || {
            provide_context(app_state.leptos_options.clone());
            provide_context::<AppDb>(app_state.db.clone());
            provide_context(cookies.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    cookies: Cookies,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(cookies.clone());
            provide_context::<AppDb>(app_state.db.clone());
        },
        move || view! { <App/> },
    );
    handler(req).await.into_response()
}

pub fn session_store() -> SessionStore {
    use_context::<AppDb>().unwrap().session_store
}

pub fn user_store() -> UserStore {
    use_context::<AppDb>().unwrap().user_store
}
