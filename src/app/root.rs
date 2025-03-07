use leptos::prelude::*;
use leptos::task::{spawn, spawn_local};
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *, hooks::use_navigate, NavigateOptions};
use crate::auth::is_auth;
use crate::{app::components::events::EventList};
use crate::app::pages::*;
use crate::app::components::navigation::*;
use chrono::{Datelike, Local};

#[island]
pub fn App() -> impl IntoView {

    let is_logged = Resource::new(
        || (), // Runs on mount
        |_| async {
            dbg!("Getting is_logged");
            is_auth().await.unwrap_or(false) // Defaults to false on error
        }
    );
    
    // Use `is_logged.get()` wherever needed
    let condition = move || Some(is_logged.get().unwrap_or(false));
    view! {
        <Router>
            <NavBar />
            <main>
                <Routes transition=true fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=Authenticate/>
                    <ProtectedRoute path=path!("/app/dashboard") view=Dashboard condition=condition redirect_path=|| "/"/>
                    <ProtectedRoute path=path!("/app/cards") view=Cards condition=condition redirect_path=|| "/"/>
                    <ProtectedParentRoute path=path!("/app/calendar") view=Events condition=condition redirect_path=|| "/">
                        <Route path=path!("") view=|| {
                            let navigate = leptos_router::hooks::use_navigate();
                            let current_day = Local::now().day();
                            navigate(&format!("/app/calendar/{}", current_day), Default::default());
                            view! {
                                <></>
                            }
                        }/>
                        <Route path=path!(":day") view=EventList/>
                    </ProtectedParentRoute>
                    <ProtectedRoute path=path!("/app/schedule") view=Schedule condition=condition redirect_path=|| "/"/>

                </Routes>
            </main>
        </Router>
    }
}