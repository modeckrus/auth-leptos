use leptos::logging::*;
use leptos::*;

use crate::components::user::MeBuilder;
use crate::model::user::User;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let f = |user: User| view! { <ProfilePageContent user=user/> };
    view! { <MeBuilder f=f/> }
}

#[component]
pub fn ProfilePageContent(user: User) -> impl IntoView {
    view! {
        <div class="flex-1 hero min-h-full bg-base-200">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <img src=user.avatar_url class="max-w-sm rounded-lg shadow-2xl"/>
                <div>
                    <h1 class="text-5xl font-bold">{user.display_name}</h1>
                    <h2 class="text-3xl font-bold">{user.login}</h2>
                    // print user created date
                    <p class="py-6">{user.created_at.to_string()}</p>
                </div>
            </div>
        </div>
    }
}
