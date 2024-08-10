use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            body::Body as AxumBody,
            extract::{Query, State},
            http::Request,
            response::IntoResponse,
            routing::{get, post},
            Router,
            response::sse::Sse,
        };
        use dotenv::dotenv;
        use env_logger::Env;
        use leptos::*;
        use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
        use tokio::sync::mpsc;
        use std::collections::HashMap;
        use thenetworktimes::app::*;
        use thenetworktimes::fileserv::file_and_error_handler;
        use thenetworktimes::components::chat::{SseStream, send_message_stream};
        use thenetworktimes::database::db::establish_connection;
        use thenetworktimes::state::AppState;
        use thenetworktimes::handlers::create_message;
        use thenetworktimes::services::hubble::*;

        #[tokio::main]
        async fn main() {
        
            // this is a sanity check (simple create_message function) which conflicts 
            // with the server function of the same name, so i break my no-comments rule
            // here since it's just so convenient (i should write tests for these things)
        
            dotenv().ok();
            env_logger::init_from_env(Env::default().default_filter_or("info"));
        
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options.clone();
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(App);
        
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = establish_connection(&database_url);
        
            let app_state = AppState {
                leptos_options: leptos_options.clone(),
                pool: pool.clone(),
            };
        
        
            async fn server_fn_handler(
                State(app_state): State<AppState>,
                request: Request<AxumBody>,
            ) -> impl IntoResponse {
                handle_server_fns_with_context(
                    move || {
                        provide_context(app_state.clone());
                    },
                    request,
                )
                .await
            }
        
            let app = Router::new()
                .route(
                    "/api/*fn_name",
                    get(server_fn_handler).post(server_fn_handler),
                )
                .route("/api/create_message", post(create_message))
                .route("/api/userNameProofsByFid/:fid", get(get_username_proofs_by_fid))
                .route("/api/userDataByFid", get(get_user_data_by_fid))
                .route("/api/castById/:fid/:hash", get(get_cast_by_id))
                .route("/api/castsByFid/:fid", get(get_casts_by_fid))
                .route("/api/channels", get(get_channels))
                .route("/api/castsByChannel/:channel", get(get_casts_by_parent))
                .route("/api/castsByMention/:fid", get(get_casts_by_mention))
                .route("/api/reactionsByCast", get(get_reactions_by_cast))
                .leptos_routes_with_handler(routes, get(|State(app_state): State<AppState>, request: Request<AxumBody>| async move {
                    let handler = leptos_axum::render_app_async_with_context(
                        app_state.leptos_options.clone(),
                        move || {
                            provide_context(app_state.clone());
                        },
                        move || view! { <App/> },
                    );
        
                    handler(request).await.into_response()
                }))
                .route("/api/send_message_stream", axum::routing::get(|Query(params): Query<HashMap<String, String>>| async move {
                    let (tx, rx) = mpsc::channel(1);
                    if let (Some(thread_id), Some(model)) = (params.get("thread_id"), params.get("model")) {
                        let thread_id = thread_id.clone();
                        let model = model.clone();
                        tokio::spawn(async move {
                            send_message_stream(thread_id, model, tx).await;
                        });
                    }
                    Sse::new(SseStream { receiver: rx })
                }))
                .fallback(file_and_error_handler)
                .with_state(app_state);
        
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            logging::log!("listening on http://{}", &addr);
            axum::serve(listener, app.into_make_service()).await.unwrap();
        }
    } else {
        pub fn main() {
            // no client-side main function
            // unless we want this to work with e.g., Trunk for a purely client-side app
            // see lib.rs for hydration function instead
        }
    }
}
