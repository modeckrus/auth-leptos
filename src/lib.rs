pub mod c;
pub mod components;
pub mod model;
pub mod notification_center;
use std::{fmt::Debug, str::FromStr};

use cfg_if::cfg_if;
pub mod fileserv;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")]{
        pub mod server;
    }
}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::components::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}
use leptos::ServerFnError;
pub type R<T = (), E = anyhow::Error> = anyhow::Result<T, E>;
pub type SE = ServerFnError<MyError>;
pub type SR<T = ()> = Result<T, ServerFnError<MyError>>;
#[derive(Debug)]
pub struct MyError(pub std::sync::Arc<anyhow::Error>);

impl From<anyhow::Error> for MyError {
    fn from(e: anyhow::Error) -> Self {
        Self(e.into())
    }
}

impl From<ServerFnError<anyhow::Error>> for MyError {
    fn from(e: ServerFnError<anyhow::Error>) -> Self {
        match e {
            ServerFnError::WrappedServerError(e) => Self::from(e),
            _ => Self(anyhow::anyhow!(e.to_string()).into()),
        }
    }
}

impl FromStr for MyError {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MyError(anyhow::anyhow!(s.to_owned()).into()))
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl serde::Serialize for MyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
//impl deserialize
impl<'de> serde::Deserialize<'de> for MyError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(MyError(anyhow::anyhow!(s).into()))
    }
}

impl Clone for MyError {
    fn clone(&self) -> Self {
        MyError(self.0.clone())
    }
}