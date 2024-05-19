use leptos::logging::*;
use leptos::*;

#[component]
pub fn TestPage() -> impl IntoView {
    crate::components::helper::redirect::redirect("https://google.com", None);
    view! { <h1>"Test"</h1> }
}
