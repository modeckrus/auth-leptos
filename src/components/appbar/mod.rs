pub mod login_button_bar;
pub mod user_avatar_bar;
pub mod user_avatar;
use leptos::*;
use leptos::logging::*;
use crate::components::appbar::user_avatar::UserAvatar;
use crate::components::theme::DarkModeToggle;

#[component]
pub fn AppBar() -> impl IntoView{
    view! {
        <header class="navbar bg-base-100 sticky top-1 z-50">
            <div class="flex-1">
                <a class="btn btn-ghost text-xl" href="/">
                    "Auth Example"
                </a>
            </div>
            <div class="flex-none gap-2">
                <DarkModeToggle/>
                // <div class="dropdown dropdown-end">
                // <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                // <div class="indicator">
                // <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
                // <span class="badge badge-sm indicator-item">8</span>
                // </div>
                // </div>
                // </div>
                <UserAvatar/>

            </div>
        </header>
    }
}