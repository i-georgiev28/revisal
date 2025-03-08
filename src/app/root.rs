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
            let a = is_auth().await;
            dbg!(format!("Checking is_logged: {:?}", a.clone()));
            a.unwrap_or(false) // Defaults to false on error
        }
    );
    
    // Use `is_logged.get()` wherever needed
    let condition = move || {
        let is_logged = is_logged.get();
        dbg!(format!("condition is_logged: {:?}", is_logged.clone()));
        Some(is_logged.unwrap_or(false))
    };

    let not_auth = move || {
        let is_logged = is_logged.get();
        dbg!(format!("condition is_logged: {:?}", is_logged.clone()));
        Some(!is_logged.unwrap_or(true))
    };
    view! {
        <Router>
            <NavBar />
            <main>
                <Routes transition=true fallback=|| "Page not found.".into_view()>
                    <ProtectedRoute path=path!("") view=Authenticate condition=not_auth redirect_path=|| "/app/dashboard"/>
                    <Route path=path!("/app/dashboard") view=Dashboard/>
                    <Route path=path!("/app/cards") view=Cards/>
                    <ParentRoute path=path!("/app/calendar") view=Events>
                        <Route path=path!("") view=|| {
                            let navigate = leptos_router::hooks::use_navigate();
                            let current_day = Local::now().day();
                            navigate(&format!("/app/calendar/{}", current_day), Default::default());
                            view! {
                                <></>
                            }
                        }/>
                        <Route path=path!(":day") view=EventList/>
                    </ParentRoute>
                    <Route path=path!("/app/schedule") view=Schedule/>

                </Routes>
            </main>
        </Router>
    }
}