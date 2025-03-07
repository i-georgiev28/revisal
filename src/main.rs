#![recursion_limit = "256"]

use revisal::app::components::cards::Card;

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::logging::log;
    use leptos_axum::LeptosRoutes;
    use axum::routing::*;
    use axum::body::Body as AxumBody;
    use axum::extract::{FromRef, Path, State};
    use axum::{
        http::Request,
        response::{IntoResponse, Response},
    };
    use axum::Router;
    use axum_session::{SessionConfig, SessionLayer, SessionStore, SessionMode, SameSite};
    use axum_session_auth::*;
    use axum_session_surreal::SessionSurrealPool;

    use leptos::prelude::{provide_context, LeptosOptions};
    use leptos_axum::handle_server_fns_with_context;
    use leptos::config::get_configuration;
    use leptos_axum::generate_route_list;
    use leptos_axum::AxumRouteListing;
    use surrealdb::{
        engine::local::Mem,
        Surreal,
    };
    use surrealdb::engine::local::Db;
    use surrealdb::engine::any::Any;

    use revisal::state::*;
    use revisal::model::*;
    use revisal::app::*;
    use revisal::auth::*;

    #[tokio::main]
    async fn main() {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        
        db.use_ns("app").use_db("revisal").await.unwrap();
    
        let session_config = SessionConfig::default()
            .with_table_name("sessions")
            .with_store_name("auth_cookie".to_owned())
            .with_cookie_same_site(SameSite::Strict)
            .with_mode(SessionMode::Persistent);
            ;

        let auth_config = AuthConfig::<String>::default().with_anonymous_user_id(None)
        .with_session_id("auth_cookie".to_string());

        let session_store: SessionStore<SessionSurrealPool<Db>> =
        SessionStore::new(Some(SessionSurrealPool::from(db.clone())), session_config)
            .await
            .unwrap();

        StudentRecord::create_students_table(&db).await;
        EventRecord::create_event_table(&db).await;
        CardsetRecord::create_cardset_table(&db).await;
        
        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr.clone();
        let site_root = conf.leptos_options.site_root.clone();
        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(App);

        let app_state = AppState {
            leptos_options,
            routes: routes.clone(),
            db: db.clone(),
        };

        let app = Router::new()
        .route(
            "/api/*fname",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(
            AuthSessionLayer::<StudentRecord, String, SessionSurrealPool<Db>, Surreal<Db>>::new(Some(db))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).    await.unwrap();
        axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    }
    pub async fn leptos_routes_handler(
        auth_session: SurrealAuthSession,
        state: State<AppState>,
        req: Request<AxumBody>,
    ) -> Response {
        let State(app_state) = state.clone();
        let handler = leptos_axum::render_route_with_context(
            state.routes.clone(),
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.db.clone());
            },
            move || shell(app_state.leptos_options.clone()),
        );
        handler(state, req).await.into_response()
    }
    pub async fn server_fn_handler(
        State(app_state): State<AppState>,
        auth_session: SurrealAuthSession,
        _path: Path<String>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.db.clone());
            },
            request,
        )
        .await
    }


} else {
    pub fn main() {}
}
}