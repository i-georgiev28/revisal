use axum::extract::FromRef;
use surrealdb::{
    engine::{local::Db, local::Mem},
    Surreal,
};
use leptos_axum::AxumRouteListing;
use leptos::prelude::LeptosOptions;


#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
    pub db: Surreal<Db>,
}