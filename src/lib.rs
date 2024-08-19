#![feature(iter_intersperse)]
use cfg_if::cfg_if;
pub mod error_template;
pub mod fileserv;
pub mod handlers;

pub mod app;
pub mod state;
pub mod pages;
pub mod components;
pub mod database;
pub mod models;
pub mod schema;
pub mod services;
pub mod rogging;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;
    use crate::rogging::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        rogging::init_logging();
//        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
