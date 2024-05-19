use std::rc::Rc;

use leptos::*;
use leptos_router::{use_navigate, use_resolved_path, NavigateOptions};
/// Redirects the user to a new URL, whether on the client side or on the server
/// side. If rendered on the server, this sets a `302` status code and sets a `Location`
/// header. If rendered in the browser, it uses client-side navigation to redirect.
/// In either case, it resolves the route relative to the current route. (To use
/// an absolute path, prefix it with `/`).
///
/// **Note**: Support for server-side redirects is provided by the server framework
/// integrations ([`leptos_actix`] and [`leptos_axum`]. If you’re not using one of those
/// integrations, you should manually provide a way of redirecting on the server
/// using [`provide_server_redirect`].
///
/// [`leptos_actix`]: <https://docs.rs/leptos_actix/>
/// [`leptos_axum`]: <https://docs.rs/leptos_axum/>
pub fn redirect<P>(path: P, options: Option<NavigateOptions>)
where
    P: core::fmt::Display + 'static,
{
    let path = path.to_string();
    // redirect on the server
    if let Some(redirect_fn) = use_context::<ServerRedirectFunction>() {
        (redirect_fn.f)(&path);
    }
    // redirect on the client
    else {
        #[allow(unused)]
        let navigate = use_navigate();
        leptos::logging::warn!("Redirecting to {}", path);
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        if let Err(err) = window().location().set_href(&path) {
            leptos::logging::error!("Failed to set location: {:?}", err);
        }
        #[cfg(not(any(feature = "csr", feature = "hydrate")))]
        {
            leptos::logging::debug_warn!(
                "<Redirect/> is trying to redirect without \
                 `ServerRedirectFunction` being provided. (If you’re getting \
                 this on initial server start-up, it’s okay to ignore. It \
                 just means that your root route is a redirect.)"
            );
        }
    }
}

/// Wrapping type for a function provided as context to allow for
/// server-side redirects. See [`provide_server_redirect`]
/// and [`Redirect`].
#[derive(Clone)]
pub struct ServerRedirectFunction {
    f: Rc<dyn Fn(&str)>,
}

impl core::fmt::Debug for ServerRedirectFunction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ServerRedirectFunction").finish()
    }
}

/// Provides a function that can be used to redirect the user to another
/// absolute path, on the server. This should set a `302` status code and an
/// appropriate `Location` header.
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "trace", skip_all,)
)]
pub fn provide_server_redirect(handler: impl Fn(&str) + 'static) {
    provide_context(ServerRedirectFunction {
        f: Rc::new(handler),
    })
}
#[cfg(feature = "ssr")]
pub mod server {
    use http::{
        header::{self, ACCEPT},
        request::Parts,
        HeaderName, HeaderValue, StatusCode,
    };
    use leptos::{server_fn::redirect::REDIRECT_HEADER, use_context};
    use leptos_axum::ResponseOptions;

    /// Provides an easy way to redirect the user from within a server function. Mimicking the Remix `redirect()`,
    /// it sets a StatusCode of 302 and a LOCATION header with the provided value.
    /// If looking to redirect from the client, `leptos_router::use_navigate()` should be used instead
    pub fn redirect(path: &str) {
        if let (Some(req), Some(res)) = (use_context::<Parts>(), use_context::<ResponseOptions>()) {
            // insert the Location header in any case
            res.insert_header(
                header::LOCATION,
                header::HeaderValue::from_str(path).expect("Failed to create HeaderValue"),
            );

            let accepts_html = req
                .headers
                .get(ACCEPT)
                .and_then(|v| v.to_str().ok())
                .map(|v| v.contains("text/html"))
                .unwrap_or(false);
            if accepts_html {
                // if the request accepts text/html, it's a plain form request and needs
                // to have the 302 code set
                res.set_status(StatusCode::FOUND);
            } else {
                // otherwise, we sent it from the server fn client and actually don't want
                // to set a real redirect, as this will break the ability to return data
                // instead, set the REDIRECT_HEADER to indicate that the client should redirect
                res.insert_header(
                    HeaderName::from_static(REDIRECT_HEADER),
                    HeaderValue::from_str("").unwrap(),
                );
            }
        } else {
            tracing::warn!(
                "Couldn't retrieve either Parts or ResponseOptions while trying \
             to redirect()."
            );
        }
    }
}
