#![allow(unused)]
#![recursion_limit = "256"]

pub mod app;
pub mod model;

pub mod auth;

#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}