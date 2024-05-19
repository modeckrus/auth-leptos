use leptos::html::col;
use leptos::logging::*;
use leptos::*;
use leptos_meta::Meta;
use leptos_router::ActionForm;

use crate::components::app::Ctx;
use crate::model::theme::Theme;
#[derive(Debug, Clone, Copy)]
pub struct ThemeS(RwSignal<Theme>);

impl std::ops::Deref for ThemeS {
    type Target = RwSignal<Theme>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ThemeS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for ThemeS {
    fn default() -> Self {
        Self(RwSignal::new(initial_prefers_dark()))
    }
}

#[server(ThemeMode, "/api")]
pub async fn theme_mode(theme: Theme) -> Result<Theme, ServerFnError> {
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;

    if let Some(cookies) = use_context::<Cookies>() {
        warn!("cookies: {:?}", cookies);
        let mut cookie = Cookie::new("theme", theme.as_str());
        cookie.set_max_age(tower_cookies::cookie::time::Duration::seconds(60 * 60 * 24));
        cookie.set_path("/");
        cookies.add(cookie);
    }
    Ok(theme)
}

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark() -> Theme {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    warn!("Cookies client: {:?}", cookie);
    cookie
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            if cookie.is_empty() {
                return None;
            }
            let mut splitted = cookie.split('=');
            if let Some(name) = splitted.next() {
                if name.trim() == "theme" {
                    Theme::from_str(splitted.next().unwrap_or_default())
                }else{
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_default()
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark() -> Theme {
    use tower_cookies::Cookies;
    if let Some(cookies) = use_context::<Cookies>() {
        let darkmode = cookies
            .get("theme")
            .map(|cookie| Theme::from_str(cookie.value()).unwrap_or_default())
            .unwrap_or_default();
        warn!("Cookies Theme: {:?}", darkmode);
        darkmode
    } else {
        warn!("Cookies Theme: not found");
        Theme::default()
    }
}

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let initial = initial_prefers_dark();

    let theme_mode_action = create_server_action::<ThemeMode>();
    // input is `Some(value)` when pending, and `None` if not pending
    let input = theme_mode_action.input();
    // value contains most recently-returned value
    let value = theme_mode_action.value();

    // NOTE: if you're following along the with video, this was implemented
    // incorrectly at the time I made it, due to a bug in <ActionForm/> that
    // was not resetting input. This is how it should have been implemented
    // all along, which would also have fixed the bug at 49:24!
    let theme = Ctx::cx().theme;
    create_effect(move |_| {
        theme.set(match (input(), value()) {
            // if there's some current input, use that optimistically
            (Some(submission), _) => submission.theme,
            // otherwise, if there was a previous value confirmed by server, use that
            (_, Some(Ok(value))) => value,
            // otherwise, use the initial value
            _ => initial,
        });
    });

    let color_scheme = move || {
        let theme = theme();
        warn!("prefers_dark: {:?}", theme);
        return theme.as_str().to_owned();
    };

    view! {
        <header id="header">
            <Meta name="color-scheme" content=color_scheme/>
            <ActionForm action=theme_mode_action>
                <select
                    name="theme"
                    class="select select-accent w-full max-w-xs"
                    onchange="this.form.submit()"
                >

                    {move || {
                        let currect_theme = color_scheme();
                        Theme::list()
                            .into_iter()
                            .map(|theme| {
                                let selected = theme == currect_theme;
                                if selected {
                                    view! {
                                        <option value=theme.clone() selected>
                                            {theme}
                                        </option>
                                    }
                                } else {
                                    view! { <option value=theme.clone()>{theme}</option> }
                                }
                            })
                            .collect_view()
                    }}

                </select>
            </ActionForm>
        </header>
    }
}
