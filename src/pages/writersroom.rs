use leptos::*;

use crate::components::chat::Chat;
use crate::components::threadlist::{ThreadList, get_threads};
use crate::components::messagelist::MessageList;
use crate::components::toast::Toast;

#[component]
pub fn WritersRoom() -> impl IntoView {
    let (show_threads, set_show_threads) = create_signal(false);
    let (model, set_model) = create_signal("gpt-4o-mini".to_string());
    let (lab, set_lab) = create_signal("openai".to_string());
    let (thread_id, set_thread_id) = create_signal("0001".to_string());
    let (toast_visible, set_toast_visible) = create_signal(false);
    let (toast_message, set_toast_message) = create_signal(String::new());

    let threads = create_resource(
        || (),
        |_| async move { get_threads().await }
    );

    let handle_model_change = move |ev| {
        let value = event_target_value(&ev);
        set_model(value.clone());
    
        let new_lab = if value.contains("claude") {
            "anthropic"
        } else {
            "openai"
        };
        set_lab(new_lab.to_string());
    };

    let create_new_thread = create_action(move |_: &()| {

        async move {
            match create_thread().await {
                Ok(new_thread_id) => {
                    set_thread_id(new_thread_id.clone());
                    set_toast_message(format!("New thread created: {}", new_thread_id));
                    set_toast_visible(true);

                    // refetch threads to update thread list
                    threads.refetch();
                    
                    // Automatically hide the toast after 3 seconds
                    set_timeout(
                        move || set_toast_visible(false),
                        std::time::Duration::from_secs(3)
                    );
                    
                    Ok(())
                }
                Err(e) => Err(format!("Failed to create thread: {}", e))
            }
        }
    });

    create_effect(move |_| {
        if let Some(Err(error)) = create_new_thread.value().get() {
            set_toast_message(error);
            set_toast_visible(true);
            
            // Automatically hide the error toast after 5 seconds
            set_timeout(
                move || set_toast_visible(false),
                std::time::Duration::from_secs(5)
            );
        }
    });

    view! {
        <div class="w-full flex flex-col justify-start pt-2 pl-2 pr-2 h-full">
            <div class="flex flex-row items-center justify-between">
                <div class="flex flex-row items-center justify-center space-x-4">
                    <button
                        class="self-start ib text-xs md:text-sm text-gray-800 hover:text-gray-900 p-2 border-2 bg-teal-800 hover:bg-teal-900 border-gray-700 hover:border-gray-900"
                        on:click=move |_| set_show_threads.update(|v| *v = !*v)
                    >
                        {move || if show_threads.get() { "hide threads" } else { "show threads" }}
                    </button>
                    <button
                        class="ib text-xs md:text-sm text-mint-700 hover:text-aqua-400 bg-teal-800 hover:bg-teal-900 border-gray-700 hover:border-gray-900"
                        on:click=move |_| create_new_thread.dispatch(())
                    >
                        "mew"
                    </button>
                </div>
                <select
                    class="self-start ib text-xs md:text-sm text-gray-800 hover:text-gray-900 p-2 border-2 bg-teal-800 hover:bg-teal-900 border-gray-700 hover:border-gray-900"
                    on:change=handle_model_change
                >
                    <option value="claude-3-haiku-20240307">"claude-3-haiku"</option>
                    <option value="claude-3-sonnet-20240229">"claude-3-sonnet"</option>
                    <option value="claude-3-opus-20240229">"claude-3-opus"</option>
                    <option value="claude-3-5-sonnet-20240620">"claude-3-5-sonnet"</option>
                    <option value="gpt-4o-mini" selected="selected">
                        "gpt-4o-mini"
                    </option>
                    <option value="gpt-4o">"gpt-4o"</option>
                    <option value="gpt-4-turbo">"gpt-4-turbo"</option>
                </select>
            </div>
            <div class="flex flex-row items-start justify-between">
                <div class=move || {
                    let base_class = "transition-all duration-300 ease-in-out overflow-hidden";
                    if show_threads.get() {
                        format!("{} max-w-xs w-full opacity-100", base_class)
                    } else {
                        format!("{} max-w-0 w-0 opacity-0", base_class)
                    }
                }>
                    <Suspense fallback = move || view! { <p>"loading threads..."</p> }>
                    {move || {
                        threads.get().map(|thread_list| {
                            match thread_list {
                                Ok(_threads) => view! {
                                    <div>
                                        <ThreadList 
                                            current_thread_id=thread_id 
                                            set_current_thread_id=set_thread_id
                                            _lab=lab // will use for filtering later
                                        />
                                    </div>
                                },
                                Err(_) => view! { <div>"error loading threads: {e}"</div>},
                            }
                        })
                    }}

                    </Suspense>
                </div>
                <div class="w-full flex flex-col content-end justify-between h-[calc(90vh-10px)]">
                    <MessageList current_thread_id=thread_id/>
                    <div class="relative text-mint-700">
                        <Toast
                            message=toast_message
                            visible=toast_visible
                            on_close=move |_| set_toast_visible(false)
                        />
                        <Chat thread_id=thread_id model=model lab=lab/>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[server(CreateThread, "/api")]
pub async fn create_thread() -> Result<String, ServerFnError> {
    use diesel::prelude::*;
    use crate::schema::threads;
    use chrono::Utc;
    use std::fmt;
    use crate::state::AppState;
    use crate::models::conversations::Thread;

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
                ThreadError::Database(e) => write!(f, "database error: {}", e),
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

    let new_thread_id = conn
        .interact(|conn| {
            let new_thread = Thread {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            };

            diesel::insert_into(threads::table)
                .values(&new_thread)
                .execute(conn)
                .map(|_| new_thread.id)
        })
        .await
        .map_err(|e| ThreadError::Interaction(e.to_string()))
        .map_err(to_server_error)?
        .map_err(ThreadError::Database)
        .map_err(to_server_error)?;

    Ok(new_thread_id)
}
