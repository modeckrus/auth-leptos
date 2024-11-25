use crate::{
    components::{
        appbar::AppBar,
        auth::auth_page::AuthorizePage,
        error_template::{AppError, ErrorTemplate},
        profile::ProfilePage,
        signature::Signature,
        test::TestPage,
    },
    notification_center::{NotificationCenter, NotificationCenterBuilder},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::{auth::Auth, theme::ThemeS};
#[derive(Default, Debug, Clone, Copy)]
pub struct Ctx {
    pub theme: ThemeS,
    pub auth: Auth,
}

impl Ctx {
    pub fn cx() -> Self {
        leptos::use_context::<Self>().unwrap_or_default()
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(Ctx::default());
    crate::components::helper::provide_helper_context();
    let theme = Ctx::cx().theme;
    let theme_class = move || theme.get().as_str();

    view! {
        <Html attr:data-theme=move || theme_class()/>

        <AppBar/>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/auth-leptos.css"/>
        // <meta name="view-transition" content="same-origin" />
        // sets the document title
        <Title text="Welcome to Leptos"/>

        <NotificationCenterBuilder/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="flex flex-col flex-1">
                <Routes>
                    <Route ssr=SsrMode::OutOfOrder path="" view=HomePage/>
                    <Route ssr=SsrMode::OutOfOrder path="auth" view=AuthorizePage/>
                    <Route ssr=SsrMode::OutOfOrder path="profile" view=ProfilePage/>
                    <Route ssr=SsrMode::OutOfOrder path="test" view=TestPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);
    // let dark_mode = Ctx::cx().dark_mode;
    // let token = Ctx::cx().token;
    let on_error_click = move |_| {
        let center = leptos::use_context::<NotificationCenter>().unwrap();
        center.error("error message".to_string());
    };
    view! {
        <div style="width: 100%; height: 100%;">
            <Signature/>
            <button class="btn btn-error" on:click=on_error_click>
                "Error"
            </button>
        // <UserAvatar />
        // <h1>"Welcome to Leptos!"</h1>
        // <LogoutButton/>
        // <AuthorizePage/>
        // <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
