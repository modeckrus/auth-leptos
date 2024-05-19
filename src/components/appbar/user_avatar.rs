use leptos::logging::*;
use leptos::*;

use crate::components::app::Ctx;
use crate::components::appbar::user_avatar_bar::UserAvatarAppBar;
use crate::model::user::User;

#[component]
pub fn UserAvatar() -> impl IntoView {
    let token = Ctx::cx().auth;
    let t = create_memo(move |_| token.get());
    view! {
        {move || match t.get() {
            Some(_) => view! { <UserAvatarContent/> }.into_view(),
            None => {
                view! {
                    <div>
                        <a class="btn btn-primary" href="/auth">
                            "Authorize"
                        </a>
                    </div>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
pub fn UserAvatarContent() -> impl IntoView {
    use crate::components::user::MeBuilder;
    let f = |user: User| {
        return view! { <UserAvatarAppBar user=user/> };
    };
    view! { <MeBuilder f=f/> }
}
