use leptos::logging::*;
use leptos::*;
use leptos_router::ActionForm;

use crate::components::app::Ctx;
use crate::model::user::User;

#[component]
pub fn UserAvatarAppBar(user: User) -> impl IntoView {
    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
                <div class="w-10 rounded-full">
                    <img alt="Tailwind CSS Navbar component" src=user.avatar_url/>
                </div>
            </div>
            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
            >
                <li>
                    <a href="/profile">{user.display_name}</a>
                </li>
                <li>
                    <button class="link link-error" onclick="logout_modal.showModal()">
                        "Logout"
                    </button>
                </li>
            </ul>
        </div>
        <UserAvatarModal/>
    }
}

#[component]
pub fn UserAvatarModal() -> impl IntoView {
    let logout_action = create_server_action::<Logout>();
    let value = logout_action.value();
    let auth = Ctx::cx().auth;
    create_effect(move |_| {
        if let Some(Ok(_)) = value.get() {
            auth.update(|auth| *auth = None);
        }
    });
    view! {
        <dialog id="logout_modal" class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-lg">"Logout"</h3>
                <p class="py-4">"Are you sure you want to logout?"</p>
                <div class="modal-action gap=1">
                    <ActionForm action=logout_action>
                        <button class="btn btn-error" type="submit">
                            "Logout"
                        </button>
                    </ActionForm>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </div>
            </div>
        </dialog>
    }
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use leptos::logging::*;
    use leptos::use_context;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;
    if let Some(cookies) = use_context::<Cookies>() {
        let mut cookie = Cookie::new("auth", "");
        cookie.set_path("/");
        cookie.set_max_age(cookie::time::Duration::milliseconds(1));
        cookies.add(cookie);
    }
    Ok(())
}
