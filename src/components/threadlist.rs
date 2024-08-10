use leptos::*;
use log::error;

use crate::models::conversations::ThreadView;

#[component]
pub fn ThreadList(
    current_thread_id: ReadSignal<String>,
    set_current_thread_id: WriteSignal<String>
) -> impl IntoView {
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
        <div class="thread-list pt-2">
            {move || {
                thread_list.get().into_iter().map(|thread: ThreadView| {
                    let thread_id = thread.id.clone();
                    let is_active = current_thread_id.get() == thread_id;
                    let (button_class, text_class) = if is_active {
                        ("border-teal-500 bg-teal-900", "text-teal-400 hover:text-teal-500")
                    } else {
                        ("border-teal-700 bg-teal-800 hover:border-teal-800 hover:bg-teal-900", "text-gray-600 hover:text-teal-500")
                    };
                    view! {
                        <div class="thread-list-container text-salmon-500 flex flex-col items-start justify-center">
                            <button 
                                class=format!("thread-item w-full p-2 border-2 {} transition duration-0 group", button_class)
                                on:click=move |_| set_current_thread_id(thread_id.clone())
                            > 
                            <p class=format!("thread-id ib pr-16 md:pr-36 text-base self-start {} transition duration-0 group", text_class)>
                                {thread.id.clone()}
                            </p>
                            <div class="stats-for-nerds hidden group-hover:flex">
                                <p class="message-created_at ir text-xs text-gray-900 hover:text-gray-700">
                                  created: {thread.created_at.map(|dt| dt.format("%b %d, %I:%M %p").to_string()).unwrap_or_default()}
                                </p>
                            </div>
                            </button>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}

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
