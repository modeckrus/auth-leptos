use leptos::{create_effect, RwSignal, SignalGet, SignalGetUntracked, SignalSet};

pub mod auth_page;

pub type AuthContext = Option<String>;

#[derive(Debug, Clone, Copy)]
pub struct Auth(RwSignal<AuthContext>);

impl std::ops::Deref for Auth {
    type Target = RwSignal<AuthContext>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Auth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self(RwSignal::new(initial_auth()))
    }
}

impl Auth {
    pub fn guard(&self) -> impl leptos::IntoView {
        let auth = self.clone();
        let result = move || {
            let auth = auth.clone();
            if auth.get().is_none() {
                leptos::logging::warn!("Redirecting to /auth");
                return leptos::IntoView::into_view(
                    leptos::view! { <leptos_router::Redirect path="/auth"></leptos_router::Redirect> },
                );
            }
            return leptos::IntoView::into_view(leptos::view! { <></> });
        };
        leptos::view! {{move || result()}}
    }

    pub fn set(&self, token: Option<String>) {
        self.0.set(token);
    }
}

#[cfg(not(feature = "ssr"))]
fn initial_auth() -> AuthContext {
    let (cookie, _) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("auth");
    if let Some(token) = cookie.get_untracked() {
        return Some(token);
    }
    None
}

#[cfg(feature = "ssr")]
fn initial_auth() -> AuthContext {
    use leptos::logging::*;
    use leptos::use_context;
    use tower_cookies::Cookies;
    if let Some(cookies) = use_context::<Cookies>() {
        let auth = cookies.get("auth").map(|cookie| cookie.value().to_owned());
        warn!("Cookies Auth: {:?}", auth);
        auth
    } else {
        warn!("Cookies Auth: not found");
        None
    }
}

#[cfg(feature = "ssr")]
fn save_auth_to_cookie(token: String) {
    use leptos::logging::*;
    use leptos::use_context;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;
    if let Some(cookies) = use_context::<Cookies>() {
        let mut cookie = Cookie::new("auth", token);
        cookie.set_path("/");
        cookie.set_max_age(cookie::time::Duration::days(365));
        cookies.add(cookie);
        warn!("Cookies {:?}", cookies);
    } else {
        warn!("Cookies not found");
    }
}
