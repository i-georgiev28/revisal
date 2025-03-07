use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};

use crate::app::components::dashboard::*;
#[island]
pub fn Dashboard() -> impl IntoView {
    view! {
        <main class="antialiased" id="dashboard">
            <Calendar />

            <Events />
          
            <Schedule />
          
            <Homework />
          
            <Daily />
        </main>
    }
}