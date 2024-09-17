use leptos::*;
use log::error;
use web_sys::Event;

use crate::models::conversations::ThreadView;

#[component]
pub fn ThreadList(
    current_thread_id: ReadSignal<String>,
    set_current_thread_id: WriteSignal<String>,
    _lab: ReadSignal<String> // will use later
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

    let search_threads = create_action(move |query: &String| {
        let query = query.clone();
        async move {
            if query.is_empty() {
                fetch_threads();
            } else {
                match search_threads(query).await {
                    Ok(search_results) => {
                        set_thread_list.set(search_results);
                    }
                    Err(e) => {
                        error!("failed to search threads: {:?}", e);
                    }
                }
            }
        }
    });

    let handle_search = move |ev: Event| {
        let query = event_target_value(&ev);
        search_threads.dispatch(query);
    };

    let delete_thread_action = create_action(move |thread_id: &String| {
        let thread_id = thread_id.clone();
        let current_id = current_thread_id.get();
        async move {
            match delete_thread(thread_id.clone()).await {
                Ok(_) => {
                    match get_threads().await {
                        Ok(updated_threads) => {
                            set_thread_list(updated_threads.clone());

                            // if deleted thread was the current one, select next available thread
                            if current_id == thread_id {
                                if let Some(next_thread) = updated_threads.first() {
                                    set_current_thread_id(next_thread.id.clone());
                                } else {
                                    // TODO handle this with UI or something
                                    log::info!("no threads left gang");
                                }
                            }
                        }
                        Err(e) => {
                            error!("failed to fetch updated threads: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("failed to delete thread: {:?}", e);
                }
            }
        }
    });

    view! {
        <div class="thread-list-container flex flex-col items-start pt-2">
            <input
                type="text"
                placeholder="grep articles!"
                on:input=handle_search
                class="grep-box w-7/12 p-2 mb-2 bg-teal-400 text-mint-700
                border-2 border-mint-700"
            />
            {move || {
                thread_list()
                    .into_iter()
                    .map(|thread: ThreadView| {
                        let thread_id = thread.id.clone();
                        let is_active = current_thread_id() == thread_id;
                        let (button_class, text_class) = if is_active {
                            ("border-teal-500 bg-teal-900", "text-teal-400 hover:text-teal-500")
                        } else {
                            (
                                "border-teal-700 bg-teal-800 hover:border-teal-800 hover:bg-teal-900",
                                "text-gray-600 hover:text-teal-500",
                            )
                        };
                        let thread_id_for_set = thread_id.clone();
                        let thread_id_for_delete = thread_id.clone();
                        view! {
                            <div class="thread-list text-salmon-500 flex flex-col items-start justify-center">
                                <div class="flex w-full justify-between items-center">
                                    <button
                                        class=format!(
                                            "thread-item w-full p-2 border-2 {} transition duration-0 group",
                                            button_class,
                                        )

                                        on:click=move |_| set_current_thread_id(
                                            thread_id_for_set.clone(),
                                        )
                                    >
                                        <p class=format!(
                                            "thread-id ib pr-16 md:pr-36 text-base self-start {} transition duration-0 group",
                                            text_class,
                                        )>{thread.id.clone()}</p>
                                        <div class="stats-for-nerds hidden group-hover:flex">
                                            <p class="message-created_at ir text-xs text-gray-900">
                                                created:
                                                {thread
                                                    .created_at
                                                    .map(|dt| dt.format("%b %d, %I:%M %p").to_string())
                                                    .unwrap_or_default()}
                                            </p>
                                        </div>
                                    </button>
                                    <button
                                        class="delete-button ib text-salmon-600 hover:text-salmon-700 text-sm ml-2"
                                        on:click=move |_| {
                                            delete_thread_action.dispatch(thread_id_for_delete.clone())
                                        }
                                    >

                                        "delet"
                                    </button>
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </div>
    }
}

#[server(SearchThreads, "/api")]
pub async fn search_threads(query: String) -> Result<Vec<ThreadView>, ServerFnError> {
    use diesel::prelude::*;
    use std::fmt;

    use crate::state::AppState;
    use crate::models::conversations::Thread;
    use crate::schema::{threads, messages};

    #[derive(Debug)]
    enum SearchError {
        Pool(String),
        Database(diesel::result::Error),
        Interaction(String),
    }

    impl fmt::Display for SearchError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SearchError::Pool(e) => write!(f, "pool error: {}", e),
                SearchError::Database(e) => write!(f, "database error: {}", e),
                SearchError::Interaction(e) => write!(f, "interaction error: {}", e),
            }
        }
    }

    fn to_server_error(e: SearchError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    let app_state = use_context::<AppState>()
        .expect("failed to get AppState from context");

    let pool = app_state.pool;

    let conn = pool
        .get()
        .await
        .map_err(|e| SearchError::Pool(e.to_string()))
        .map_err(to_server_error)?;

    let result = conn
        .interact(move |conn| {
            threads::table
                .left_join(messages::table)
                .filter(
                    threads::id.like(format!("%{}%", query))
                        .or(messages::content.like(format!("%{}%", query)))
                )
                .select(threads::all_columns)
                .distinct()
                .load::<Thread>(conn)
        })
        .await
        .map_err(|e| SearchError::Interaction(e.to_string()))
        .map_err(to_server_error)?
        .map_err(SearchError::Database)
        .map_err(to_server_error)?;

    Ok(result.into_iter().map(ThreadView::from).collect())
}

#[server(DeleteThread, "/api")]
pub async fn delete_thread(thread_id: String) -> Result<(), ServerFnError> {
    use diesel::prelude::*;
    use crate::schema::{threads, messages};
    use std::fmt;
    use crate::state::AppState;

    #[derive(Debug)]
    enum ThreadError {
        Pool(String),
        Database(diesel::result::Error),
        Interaction(String),
    }

    impl fmt::Display for ThreadError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ThreadError::Pool(e) => write!(f, "pool error: {}", e),
                ThreadError::Database(e)=> write!(f, "database error: {}", e),
                ThreadError::Interaction(e) => write!(f, "interaction error: {}", e),
            }
        }
    }

    fn to_server_error(e: ThreadError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    let app_state = use_context::<AppState>()
        .expect("failed to get AppState from context");
    let pool = app_state.pool;

    let conn = pool
        .get()
        .await
        .map_err(|e| ThreadError::Pool(e.to_string()))
        .map_err(to_server_error)?;

    conn.interact(move |conn| {
        conn.transaction(|conn| {
            // first, delete all messages associated with thread
            diesel::delete(messages::table.filter(messages::thread_id.eq(&thread_id)))
                .execute(conn)?;

            // then, delete thread itself
            diesel::delete(threads::table.find(thread_id))
                .execute(conn)?;

            Ok(())
        })
    })
    .await
    .map_err(|e| ThreadError::Interaction(e.to_string()))
    .map_err(to_server_error)?
    .map_err(ThreadError::Database)
    .map_err(to_server_error)?;

    Ok(())
}

#[server(GetThreads, "/api")]
pub async fn get_threads() -> Result<Vec<ThreadView>, ServerFnError> {
    use diesel::prelude::*;
    use std::fmt;

    use crate::state::AppState;
    use crate::models::conversations::Thread;
    use crate::schema::threads::dsl::threads as threads_table;

    #[derive(Debug)]
    enum ThreadError {
        Pool(String),
        Database(diesel::result::Error),
        Interaction(String),
    }

    impl fmt::Display for ThreadError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ThreadError::Pool(e) => write!(f, "Pool error: {}", e),
                ThreadError::Database(e) => write!(f, "Database error: {}", e),
                ThreadError::Interaction(e) => write!(f, "Interaction error: {}", e),
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
        .map_err(|e| ThreadError::Pool(e.to_string()))
        .map_err(to_server_error)?;

    let result = conn
        .interact(|conn| {
            threads_table
                .order(crate::schema::threads::created_at.desc())
                .load::<Thread>(conn)
        })
        .await
        .map_err(|e| ThreadError::Interaction(e.to_string()))
        .map_err(to_server_error)?
        .map_err(ThreadError::Database)
        .map_err(to_server_error)?;

    Ok(result.into_iter().map(ThreadView::from).collect())
}
