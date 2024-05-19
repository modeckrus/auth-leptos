pub mod test;
pub mod helper;
use leptos_router::{use_navigate, NavigateOptions};

pub mod app;
pub mod appbar;
pub mod auth;
pub mod error_template;
pub mod profile;
pub mod signature;
pub mod theme;
pub mod user;

// pub fn process_err<T>(r: &Result<T, leptos::ServerFnError>) {
//     if let Err(ref e) = r {
//         if e.to_string() == "redirect_auth" {
//             redirect("google.com");
//         }
//     }
// }

// #[cfg(feature = "ssr")]
// pub fn redirect(url: &str){
//     leptos::logging::warn!("Redirecting to {}", url);
//     leptos_axum::redirect(url)
// }

// #[cfg(not(feature = "ssr"))]
// pub fn redirect(url: &str){
//     leptos::logging::warn!("Redirecting to {}", url);
//     let navigate = use_navigate();
//     navigate(url, leptos_router::NavigateOptions::default());
// }
