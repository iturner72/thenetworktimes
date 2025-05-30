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

    // New action for generating thread titles
    let generate_title_action = create_action(move |params: &(String, String)| {
        let (thread_id, provider) = params.clone();
        async move {
            match generate_thread_title(thread_id.clone(), provider).await {
                Ok(generated_title) => {
                    log::info!("Generated title: {}", generated_title);
                    // Refresh the thread list to show the updated title
                    match get_threads().await {
                        Ok(updated_threads) => {
                            set_thread_list(updated_threads);
                        }
                        Err(e) => {
                            error!("Failed to refresh threads after title generation: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to generate title: {:?}", e);
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
                class="grep-box w-7/12 p-2 mb-2 bg-gray-100 dark:bg-teal-800 text-teal-600 dark:text-mint-400
                border-2 border-gray-300 dark:border-teal-600 focus:border-teal-500 dark:focus:border-mint-300
                focus:outline-none transition duration-300 ease-in-out"
            />
            {move || {
                thread_list()
                    .into_iter()
                    .map(|thread: ThreadView| {
                        let thread_id = thread.id.clone();
                        let is_active = current_thread_id() == thread_id;
                        let (button_class, text_class) = if is_active {
                            (
                                "border-teal-500 bg-teal-600 dark:bg-teal-800",
                                "text-mint-400 group-hover:text-white dark:text-mint-300 dark:group-hover:text-white",
                            )
                        } else {
                            (
                                "border-teal-700 bg-gray-300 dark:bg-teal-800 hover:border-teal-800 hover:bg-gray-900",
                                "text-gray-300 group-hover:text-white dark:text-gray-100 dark:group-hover:text-white",
                            )
                        };
                        let thread_id_for_set = thread_id.clone();
                        let thread_id_for_delete = thread_id.clone();
                        let thread_id_for_title_openai = thread_id.clone();
                        let thread_id_for_title_anthropic = thread_id.clone();
                        let display_text = thread
                            .title
                            .clone()
                            .unwrap_or_else(|| thread.id.clone());
                        let has_custom_title = thread.title.is_some();
                        view! {
                            // Check if thread has a title or just show ID

                            <div class="thread-list text-teal-500 dark:text-mint-400 flex flex-col items-start justify-center w-full mb-2">
                                <div class="flex w-full justify-between items-center">
                                    <button
                                        class=format!(
                                            "thread-item w-full p-2 border-2 {} transition duration-300 ease-in-out group",
                                            button_class,
                                        )

                                        on:click=move |_| set_current_thread_id(
                                            thread_id_for_set.clone(),
                                        )
                                    >

                                        <div class="flex flex-col items-start w-full">
                                            <p class=format!(
                                                "thread-title text-base font-medium {} transition duration-300 ease-in-out {}",
                                                text_class,
                                                if has_custom_title { "" } else { "italic opacity-75" },
                                            )>{display_text}</p>

                                            // Show thread ID if we have a custom title
                                            {if has_custom_title {
                                                view! {
                                                    <p class="thread-id text-xs text-teal-400 dark:text-mint-300 opacity-60">
                                                        {thread.id.clone()}
                                                    </p>
                                                }
                                                    .into_view()
                                            } else {
                                                view! {}.into_view()
                                            }}

                                        </div>

                                        <div class="stats-for-nerds hidden group-hover:flex flex-col items-start mt-2">
                                            <p class="message-created_at text-xs text-teal-300 dark:text-mint-200 group-hover:text-teal-100 dark:group-hover:text-mint-100">
                                                created:
                                                {thread
                                                    .created_at
                                                    .map(|dt| dt.format("%b %d, %I:%M %p").to_string())
                                                    .unwrap_or_default()}
                                            </p>
                                        </div>
                                    </button>

                                    <div class="button-group flex items-center gap-1">
                                        // Title generation buttons
                                        <div class="title-buttons flex flex-col gap-1">
                                            <button
                                                class="generate-title-btn text-xs px-2 py-1 text-teal-600 dark:text-mint-400 
                                                hover:text-teal-400 dark:hover:text-mint-300 bg-gray-200 dark:bg-teal-900 
                                                hover:bg-gray-300 dark:hover:bg-teal-800 rounded transition duration-300 ease-in-out
                                                disabled:opacity-50 disabled:cursor-not-allowed"
                                                on:click=move |_| {
                                                    generate_title_action
                                                        .dispatch((
                                                            thread_id_for_title_openai.clone(),
                                                            "openai".to_string(),
                                                        ));
                                                }

                                                disabled=move || generate_title_action.pending().get()
                                                title="Generate title with OpenAI"
                                            >
                                                {move || {
                                                    if generate_title_action.pending().get() {
                                                        "..."
                                                    } else {
                                                        "GPT"
                                                    }
                                                }}

                                            </button>

                                            <button
                                                class="generate-title-btn text-xs px-2 py-1 text-teal-600 dark:text-mint-400 
                                                hover:text-teal-400 dark:hover:text-mint-300 bg-gray-200 dark:bg-teal-900 
                                                hover:bg-gray-300 dark:hover:bg-teal-800 rounded transition duration-300 ease-in-out
                                                disabled:opacity-50 disabled:cursor-not-allowed"
                                                on:click=move |_| {
                                                    generate_title_action
                                                        .dispatch((
                                                            thread_id_for_title_anthropic.clone(),
                                                            "anthropic".to_string(),
                                                        ));
                                                }

                                                disabled=move || generate_title_action.pending().get()
                                                title="Generate title with Anthropic"
                                            >
                                                {move || {
                                                    if generate_title_action.pending().get() {
                                                        "..."
                                                    } else {
                                                        "Claude"
                                                    }
                                                }}

                                            </button>
                                        </div>

                                        // Delete button
                                        <button
                                            class="delete-button text-teal-600 dark:text-mint-400 hover:text-teal-400 dark:hover:text-mint-300 
                                            text-sm p-2 bg-gray-400 dark:bg-teal-900 hover:bg-gray-500 dark:hover:bg-teal-800 
                                            rounded transition duration-300 ease-in-out"
                                            on:click=move |_| {
                                                delete_thread_action.dispatch(thread_id_for_delete.clone())
                                            }
                                        >

                                            "delet"
                                        </button>
                                    </div>
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

#[server(RenameThread, "/api")]
pub async fn rename_thread(
    thread_id: String,
    new_title: String,
) -> Result<(), ServerFnError> {
    use diesel::prelude::*;
    use crate::models::conversations::Thread;
    use crate::schema::threads;
    use std::fmt;
    use crate::state::AppState;

    #[derive(Debug)]
    enum ThreadError {
        Pool(String),
        Database(diesel::result::Error),
        Interaction(String),
        NotFound
    }

    impl fmt::Display for ThreadError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ThreadError::Pool(e) => write!(f, "pool error: {}", e),
                ThreadError::Database(e)=> write!(f, "database error: {}", e),
                ThreadError::Interaction(e) => write!(f, "interaction error: {}", e),
                ThreadError::NotFound => write!(f, "thread not found"),
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

    let thread_id_clone = thread_id.clone();
    let new_title_clone = new_title.clone();

    conn.interact(move |conn| {
        conn.transaction(|conn| {
            let thread_exists = threads::table
                .find(&thread_id_clone)
                .first::<Thread>(conn)
                .optional()?
                .is_some();

            if !thread_exists {
                return Err(diesel::result::Error::NotFound);
            }

            diesel::update(threads::table.find(&thread_id_clone))
                .set((
                    threads::title.eq(&new_title_clone),
                    threads::updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            Ok(())
        })
    })
    .await
    .map_err(|e| ThreadError::Interaction(e.to_string()))
    .map_err(to_server_error)?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => ThreadError::NotFound,
        other => ThreadError::Database(other),
    })
    .map_err(to_server_error)?;

    Ok(())
}

#[server(GenerateThreadTitle, "/api")]
pub async fn generate_thread_title(
    thread_id: String,
    provider: String,
) -> Result<String, ServerFnError> {
    use crate::components::chat::{OpenAIService, AnthropicService, fetch_message_history};
    use std::fmt;
    #[derive(Debug)]
    enum TitleGenError {
        History(String),
    }

    impl fmt::Display for TitleGenError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TitleGenError::History(e) => write!(f, "history error: {}", e),
            }
        }
    }

    fn to_server_error(e: TitleGenError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    let history = fetch_message_history(&thread_id).await
        .map_err(|e| to_server_error(TitleGenError::History(e.to_string())))?;

    let context_messages = history.iter()
        .take(3)
        .map(|msg| {
            let role = &msg.role;
            let content = msg.content.as_deref().unwrap_or_default();
            format!("{}: {}", role, content)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let title_prompt = format!(
        "Based on this conversation, generate a short, descriptive title (max 50 characters):\n\n{}",
        context_messages
    );

    let title = match provider.as_str() {
        "openai" => {
            let service = OpenAIService::new("gpt-3.5-turbo".to_string());
            generate_title_openai(&service, &title_prompt).await?
        }
        "anthropic" => {
            let service = AnthropicService::new("claude-3-haiku-20240307".to_string());
            generate_title_anthropic(&service, &title_prompt).await?
        }
        _ => return Err(ServerFnError::ServerError("Invalid provider".to_string())),
    };

    rename_thread(thread_id, title.clone()).await?;


    async fn generate_title_openai(
        service: &OpenAIService,
        prompt: &str,
    ) -> Result<String, ServerFnError> {
        use std::fmt;
    
        #[derive(Debug)]
        enum TitleGenError {
            History(String),
            Json(String)
        }
    
        impl fmt::Display for TitleGenError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    TitleGenError::History(e) => write!(f, "history error: {}", e),
                    TitleGenError::Json(e) => write!(f, "json prob: {}", e),
                }
            }
        }
    
        fn to_server_error(e: TitleGenError) -> ServerFnError {
            ServerFnError::ServerError(e.to_string())
        }
    
        let response = service.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", service.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": service.model,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "max_tokens": 60,
                "temperature": 0.7
            }))
            .send()
            .await
            .map_err(|e| to_server_error(TitleGenError::History(e.to_string())))?;
    
        let json: serde_json::Value = response.json().await
            .map_err(|e| to_server_error(TitleGenError::Json(e.to_string())))?;
    
        let title = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("New Conversation")
            .trim()
            .trim_matches('"')
            .to_string();
    
        Ok(title)
    }
    
    
    async fn generate_title_anthropic(
        service: &AnthropicService,
        prompt: &str,
    ) -> Result<String, ServerFnError> {
        use std::fmt;
    
        #[derive(Debug)]
        enum TitleGenError {
            History(String),
            Json(String)
        }
    
        impl fmt::Display for TitleGenError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    TitleGenError::History(e) => write!(f, "history error: {}", e),
                    TitleGenError::Json(e) => write!(f, "json prob: {}", e),
                }
            }
        }
    
        fn to_server_error(e: TitleGenError) -> ServerFnError {
            ServerFnError::ServerError(e.to_string())
        }
    
        let response = service.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &service.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": service.model,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "max_tokens": 60
            }))
            .send()
            .await
            .map_err(|e| to_server_error(TitleGenError::History(e.to_string())))?;
    
        let json: serde_json::Value = response.json().await
            .map_err(|e| to_server_error(TitleGenError::Json(e.to_string())))?;
    
        let title = json["content"][0]["text"]
            .as_str()
            .unwrap_or("New Conversation")
            .trim()
            .trim_matches('"')
            .to_string();
        
    
        Ok(title)
    }
    
    Ok(title)
}
