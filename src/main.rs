#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use auth_leptos::components::app::*;
    use auth_leptos::fileserv::file_and_error_handler;
    use auth_leptos::server::session::mock::SessionStoreMock;
    use auth_leptos::server::session::SessionDB;
    use auth_leptos::server::user::UserDB;
    use auth_leptos::server::{leptos_routes_handler, server_fn_handler, AppState};
    use axum::routing::get;
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_cookies::CookieManagerLayer;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let client = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>("localhost:8000")
        .await
        .unwrap();
    client.use_ns("test").use_db("test").await.unwrap();
    let session_store = auth_leptos::server::session::SessionStore::new(client.clone());
    let user_store = auth_leptos::server::user::UserStore::new(client.clone());
    match user_store
        .create_user(auth_leptos::model::user::User::admin())
        .await
        .map_err(|e| anyhow::anyhow!("failed to create admin user: {}", e))
    {
        Ok(admin) => log::info!("admin user created: {:?}", admin),
        Err(e) => log::error!("failed to create admin user: {}", e),
    }

    let app_db = auth_leptos::server::AppDb {
        session_store: session_store,
        user_store: user_store,
    };
    let app_state = AppState::new(leptos_options, app_db);
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(CookieManagerLayer::new())
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        // .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
