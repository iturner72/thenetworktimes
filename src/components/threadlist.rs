use leptos::*;
use crate::models::conversations::ThreadView;
use log::error;

#[server(GetThreads, "/api")]
pub async fn get_threads() -> Result<Vec<ThreadView>, ServerFnError> {
    use diesel::prelude::*;
    use crate::state::AppState;
    use crate::models::conversations::Thread;
    use crate::schema::threads::dsl::threads as threads_table;
    use std::fmt;

    #[derive(Debug)]
    enum ThreadError {
        PoolError(String),
        DatabaseError(diesel::result::Error),
        InteractionError(String),
    }

    impl fmt::Display for ThreadError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ThreadError::PoolError(e) => write!(f, "Pool error: {}", e),
                ThreadError::DatabaseError(e) => write!(f, "Database error: {}", e),
                ThreadError::InteractionError(e) => write!(f, "Interaction error: {}", e),
            }
        }
    }

    fn to_server_error(e: ThreadError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    let app_state = use_context::<AppState>()
        .expect("Failed to get AppState from context");

    let pool = app_state.pool;

    let conn = pool
        .get()
        .await
        .map_err(|e| ThreadError::PoolError(e.to_string()))
        .map_err(to_server_error)?;

    let result = conn
        .interact(|conn| threads_table.load::<Thread>(conn))
        .await
        .map_err(|e| ThreadError::InteractionError(e.to_string()))
        .map_err(to_server_error)?
        .map_err(ThreadError::DatabaseError)
        .map_err(to_server_error)?;

    Ok(result.into_iter().map(ThreadView::from).collect())
}

#[component]
pub fn ThreadList() -> impl IntoView {
    let (thread_list, set_thread_list) = create_signal(Vec::new());
    
    let fetch_threads = move || {
        spawn_local(async move {
            match get_threads().await {
                Ok(fetched_threads) => {
                    set_thread_list.set(fetched_threads);
                }
                Err(e) => {
                    error!("Failed to fetch threads: {:?}", e);
                }
            }
        });
    };
    
    fetch_threads();
    
    view! {
        <div class="thread-list">
            {move || {
                thread_list.get().into_iter().map(|thread: ThreadView| {
                    view! {
                        <div class="thread-list-container flex flex-col items-center justify-center">
                            <h2 class="container-title ib text-2xl text-salmon-300">thread list</h2>
                            <div class="thread-item w-4/12 border-2 border-teal-700">
                                <p class="thread-id ib text-xl text-teal-400">{thread.id.clone()}</p>
                                <p class="thread-created ir text-xs text-gray-800">created on:{thread.created_at.map(|dt| dt.to_string()).unwrap_or_default()}</p>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}
