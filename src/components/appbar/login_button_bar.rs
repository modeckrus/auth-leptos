use leptos::*;
use leptos::logging::*;
#[component]
pub fn LoginButtonBar() -> impl IntoView{
    view! {
        <a class="btn btn-primary" href="/login">
            "Login"
        </a>
    }
}