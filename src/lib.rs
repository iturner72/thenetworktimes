use cfg_if::cfg_if;
pub mod app;
pub mod error_template;
pub mod state;
pub mod handlers;
pub mod fileserv;

pub mod models;
pub mod database;
pub mod components;
pub mod services;
pub mod pages;
pub mod schema;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
