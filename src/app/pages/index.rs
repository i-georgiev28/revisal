use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};

#[island]
pub fn Index() -> impl IntoView {

    view! {
        <main id="dashboard">
            <h1> Home </h1>
        </main>
    }
}

#[island]
pub fn Schedule() -> impl IntoView {

    view! {
        <main id="schedule">
            <h1> Schedule </h1>
        </main>
    }
}