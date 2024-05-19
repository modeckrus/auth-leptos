pub mod redirect;

pub fn provide_helper_context() {
    #[cfg(feature = "ssr")]
    redirect::provide_server_redirect(redirect::server::redirect);
}
