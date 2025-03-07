pub mod components;
pub mod pages;
pub mod atoms;

mod root;
pub use root::App;

use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};

use crate::model::*;
use crate::auth::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Title text="Revisal"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <Stylesheet id="leptos" href="/pkg/revisal.css"/>
                <script src="https://cdn.tailwindcss.com"></script>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}