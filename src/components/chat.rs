use cfg_if::cfg_if;
use leptos::*;
use log::error;
use urlencoding;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{EventSource, MessageEvent, ErrorEvent, HtmlElement};

use crate::models::conversations::NewMessageView;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::response::sse::Event;
        use anyhow::{anyhow, Error};
        use reqwest::Client;
        use regex::Regex;
        use std::env;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use tokio::sync::mpsc;
        use futures::stream::{Stream, StreamExt};
        use log::info;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;

        use crate::database::db::establish_connection;
        use crate::models::conversations::Message;
        use crate::schema::messages;

        pub struct SseStream {
            pub receiver: mpsc::Receiver<Result<Event, anyhow::Error>>,
        }

        impl Stream for SseStream {
            type Item = Result<Event, anyhow::Error>;

            fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                self.receiver.poll_recv(cx)
            }
        }

        #[derive(Clone)]
        pub struct OpenAIService {
            client: Client,
            api_key: String,
            model: String,
        }

        #[derive(Clone)]
        pub struct AnthropicService {
            client: Client,
            api_key: String,
            model: String,
        }

        impl AnthropicService {
            pub fn new(model: String) -> Self {
                let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set.");
                let client = Client::new();
                AnthropicService { client, api_key, model }
            }

            pub async fn send_message(&self, thread_id: &str, tx: mpsc::Sender<Result<Event, anyhow::Error>>) -> Result<(), Error> {
                info!("Sending message to OpenAI API");
                info!("Current thread id: {}", thread_id.to_string());

                let history = fetch_message_history(thread_id).await?;
        
                let api_messages = history.into_iter()
                    .map(|msg| serde_json::json!({
                        "role": msg.role,
                        "content": msg.content.unwrap_or_default(),
                    }))
                    .collect::<Vec<_>>();
        
//                info!("history: {:?}", api_messages.clone());

                let response = self.client.post("https://api.anthropic.com/v1/messages")
                    .header("x-api-key", self.api_key.to_string())
                    .header("anthropic-version", "2023-06-01")
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({
                        "model": self.model,
                        "messages": api_messages,
                        "max_tokens": 1360,
                        "stream": true,
                    }))
                    .send()
                    .await
                    .map_err(|e| anyhow!("Failed to send message: {}", e))?;

                let mut stream = response.bytes_stream();

                // todo (this is copy pasta from open ai below, will need to fix!!
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(bytes) => {
                            let event = String::from_utf8(bytes.to_vec()).map_err(|e| anyhow!("Failed to convert bytes to string: {}", e))?;
                            info!("Trimmed event: {}", event.trim());

                            for line in event.trim().lines() {
                                if line.trim() == "event: message_stop" {
                                    info!("Received message_stop event");
                                    tx.send(Ok(Event::default().data("[DONE]"))).await.ok();
                                    break;
                                } else if line.trim().starts_with("data: ") {
                                    let json_str = &line.trim()[6..];
                                    let re = Regex::new(r#""text":"([^"]*)""#).unwrap();
                                    for cap in re.captures_iter(json_str) {
                                        let content = cap[1].to_string();
                                        info!("Extracted content: {}", content);
                                        tx.send(Ok(Event::default().data(content))).await.ok();
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to process stream: {}", e);
                            tx.send(Err(anyhow!("Failed to process stream: {}", e))).await.ok();
                            break;
                        }
                    }
                }

                info!("Stream closed");
                Ok(())
            }

        }

        impl OpenAIService {
            pub fn new(model: String) -> Self {
                let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
                let client = Client::new();
                OpenAIService { client, api_key, model }
            }

            pub async fn send_message(&self, thread_id: &str, tx: mpsc::Sender<Result<Event, anyhow::Error>>) -> Result<(), Error> {
                info!("Sending message to OpenAI API");
                info!("Current thread id: {}", thread_id.to_string());

                let history = fetch_message_history(thread_id).await?;

                let api_messages = history.into_iter()
                    .map(|msg| serde_json::json!({
                        "role": msg.role,
                        "content": msg.content.unwrap_or_default(),
                    }))
                    .collect::<Vec<_>>();

//                info!("history: {:?}", api_messages.clone());

                let response = self.client.post("https://api.openai.com/v1/chat/completions")
                    .header("Authorization", format!("Bearer {}", self.api_key))
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({
                        "model": self.model,
                        "messages": api_messages,
                        "max_tokens": 1360,
                        "stream": true,
                    }))
                    .send()
                    .await
                    .map_err(|e| anyhow!("Failed to send message: {}", e))?;

                let mut stream = response.bytes_stream();

                while let Some(item) = stream.next().await {
                    match item {
                        Ok(bytes) => {
                            let event = String::from_utf8(bytes.to_vec()).map_err(|e| anyhow!("Failed to convert bytes to string: {}", e))?;
                            info!("Trimmed event: {}", event.trim());

                            for line in event.trim().lines() {
                                if line.trim() == "data: [DONE]" {
                                    info!("Received [DONE] event");
                                    tx.send(Ok(Event::default().data("[DONE]"))).await.ok();
                                    break;
                                } else if line.trim().starts_with("data: ") {
                                    let json_str = &line.trim()[6..];
                                    let re = Regex::new(r#""content":"([^"]*)""#).unwrap();
                                    for cap in re.captures_iter(json_str) {
                                        let content = cap[1].to_string();
                                        info!("Extracted content: {}", content);
                                        tx.send(Ok(Event::default().data(content))).await.ok();
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to process stream: {}", e);
                            tx.send(Err(anyhow!("Failed to process stream: {}", e))).await.ok();
                            break;
                        }
                    }
                }

                info!("Stream closed");
                Ok(())
            }
        }

        pub async fn fetch_message_history(thread_id: &str) -> Result<Vec<Message>, Error> {
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = establish_connection(&database_url);
            let conn = pool
                .get()
                .await
                .map_err(|e| Error::msg(format!("Failed to get database connection: {:?}", e)))?;

            let thread_id = thread_id.to_string();
            let messages_result = conn
                .interact(move |conn| {
                    messages::table
                        .filter(messages::thread_id.eq(thread_id))
                        .load::<Message>(conn)
                })
                .await
                .map_err(|e| Error::msg(format!("Database interaction error: {:?}", e)))?;

            match messages_result {
                Ok(msgs) => Ok(msgs),
                Err(e) => Err(Error::msg(format!("Failed to fetch messages: {:?}", e))),
            }
        }

        pub async fn send_message_stream(thread_id: String, model: String, active_lab: String, tx: mpsc::Sender<Result<Event, anyhow::Error>>) {
            let decoded_thread_id = urlencoding::decode(&thread_id).expect("Failed to decode thread_id");
            let decoded_model = urlencoding::decode(&model).expect("Failed to decode model");
            let decoded_lab = urlencoding::decode(&active_lab).expect("failed to decode lab");

            let result = match decoded_lab.as_ref() {
                "anthropic" => {
                    let anthropic_service = AnthropicService::new(decoded_model.into_owned());
                    anthropic_service.send_message(&decoded_thread_id, tx.clone()).await
                },
                "openai" => {
                let openai_service = OpenAIService::new(decoded_model.into_owned());
                openai_service.send_message(&decoded_thread_id, tx.clone()).await
                },
                _ => Err(anyhow::anyhow!("unsupported lab: {}", decoded_lab)),
            };

            if let Err(e) = result {
                error!("Error in send_message_stream: {}", e);
            }
        }
    }
}

#[component]
pub fn Chat(
    thread_id: ReadSignal<String>,
    model: ReadSignal<String>,
    lab: ReadSignal<String>
) -> impl IntoView {
    let (message, set_message) = create_signal(String::new());
    let (response, set_response) = create_signal(String::new());
    let (is_sending, set_is_sending) = create_signal(false);
    let (llm_content, set_llm_content) = create_signal(String::new());

    let send_message_action = move |_| {
        let message_value = message.get();
        let current_thread_id = thread_id.get_untracked();
        let selected_model = model.get_untracked();
        let active_lab = lab.get_untracked();
        let role = "user";

        spawn_local(async move {
            set_is_sending(true);
            set_response.set("".to_string());
            set_llm_content.set("".to_string());
            let is_llm = false;

            let new_message_view = NewMessageView {
                thread_id: Some(current_thread_id.clone()),
                content: Some(message_value.clone()),
                role: role.to_string(),
                active_model: selected_model.clone(),
                active_lab: active_lab.clone(),
            };

            match create_message(new_message_view, is_llm).await {
                Ok(_) => {

                    let thread_id_value = thread_id().to_string();
                    let active_model_value = model().to_string();
                    let active_lab_value = lab().to_string();
                    let event_source = Rc::new(EventSource::new(
                            &format!("/api/send_message_stream?thread_id={}&model={}&lab={}",
                            urlencoding::encode(&thread_id_value),
                            urlencoding::encode(&active_model_value),
                            urlencoding::encode(&active_lab_value))
                        ).expect("Failed to connect to SSE endpoint"));
        
        			let on_message = {
        				let event_source = Rc::clone(&event_source);
        				Closure::wrap(Box::new(move |event: MessageEvent| {
        					let data = event.data().as_string().unwrap();
        					if data == "[DONE]" {
                                let llm_content_value = llm_content.get();
                                let is_llm = true;
                                let new_message_view = NewMessageView {
                                    thread_id: Some(thread_id().clone()),
                                    content: Some(llm_content_value),
                                    role: "assistant".to_string(),
                                    active_model: model().clone(),
                                    active_lab: lab().clone(),
                                };

                                spawn_local(async move {
                                    if let Err(e) = create_message(new_message_view, is_llm).await {
                                        error!("Failed to create LLM message: {:?}", e);
                                    }
                                });

        						set_is_sending.set(false);
        						event_source.close();
        					} else {
                                let processed_data = data.replace("\\n", "\n");
        						set_response.update(|resp| {
        							resp.push_str(&processed_data);
        							resp.to_string();
        						});
                                set_llm_content.update(|content| {
                                    content.push_str(&processed_data);
                                    content.to_string();
                                });
        					}
        				}) as Box<dyn FnMut(_)>)
        			};
        
        			event_source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        			on_message.forget();
        
        			let on_error = {
        				let event_source = Rc::clone(&event_source);
        				Closure::wrap(Box::new(move |event: ErrorEvent| {
                            let error_message = format!(
        						"Error receiving message: type = {:?}, message = {:?}, filename = {:?}, lineno = {:?}, colno = {:?}, error = {:?}",
                                event.type_(),
                                event.message(),
                                event.filename(),
                                event.lineno(),
                                event.colno(),
                                event.error()
                            );
        					error!("{}", error_message);
        					set_is_sending.set(false);
        					set_response(error_message);
        					event_source.close();
        				}) as Box<dyn FnMut(_)>)
        			};
        
        			event_source.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        			on_error.forget();
                }
                Err(e) => {
                    error!("Failed to create message: {:?}", e);
                    set_is_sending(false);
                }
            }
        });
    };

    view! {
        <div class="flex flex-col items-center justify-between pb-2 md:pb-4">
            <div class="w-10/12 md:w-7/12 h-[calc(0vh-10px)] overflow-y-auto flex flex-col-reverse pb-0 md:pb-12">
                <Suspense fallback=|| view! { <p class="ir text-base text-seafoam-100">"loading..."</p> }>
                    {move || {
                        view! {
                            <p class="ir text-gray-700 whitespace-pre-wrap">{response.get()}</p>
                        }
                    }}
                </Suspense>
            </div>
            <div class="flex flex-row justify-center space-x-4 w-6/12 md:w-7/12">
                <textarea
                    class="ir text-sm text-gray-600 bg-teal-800 w-full h-8 md:h-12 p-2 text-wrap"
                    value=message
                    on:input={move |event| {
                        set_message(event_target_value(&event));
                        let target = event.target().unwrap();
                        let style = target.unchecked_ref::<HtmlElement>().style();
                        style.set_property("height", "auto").unwrap();
                        style.set_property("height", &format!("{}px", target.unchecked_ref::<HtmlElement>().scroll_height())).unwrap();
                    }}
                ></textarea>
                <button
                    class="ib text-gray-700 hover:text-teal-400 text-xs md:text-lg w-1/6"
                    on:click=send_message_action
                    disabled={move || is_sending.get()}
                >
                    {move || if is_sending.get() { "yapping..." } else { "yap" }}
                </button>
            </div>
        </div>
    }
}

#[server(CreateMessage, "/api")]
pub async fn create_message(new_message_view: NewMessageView, is_llm: bool) -> Result<(), ServerFnError> {
    use diesel::prelude::*;
    use std::fmt;

    use crate::state::AppState;
    use crate::models::conversations::{NewMessage, Thread};
    use crate::schema::{messages, threads};

    #[derive(Debug)]
    enum CreateMessageError {
        PoolError(String),
        InteractionError(String),
    }

    impl fmt::Display for CreateMessageError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CreateMessageError::PoolError(e) => write!(f, "Pool error: {}", e),
                CreateMessageError::InteractionError(e) => write!(f, "Interaction error: {}", e),
            }
        }
    }

    impl From<CreateMessageError> for ServerFnError {
        fn from(error: CreateMessageError) -> Self {
            ServerFnError::ServerError(error.to_string())
        }
    }

    let app_state = use_context::<AppState>()
        .expect("Failed to get AppState from context");

    let pool = app_state.pool;

    let conn = pool
        .get()
        .await
        .map_err(|e| CreateMessageError::PoolError(e.to_string()))?;

    conn.interact(move |conn| {
        let new_message: NewMessage = new_message_view.into();

        if !is_llm {
            if let Some(thread_id) = &new_message.thread_id {
                if threads::table.find(thread_id).first::<Thread>(conn).optional()?.is_none() {
                    let new_thread = Thread {
                        id: thread_id.clone(),
                        created_at: None,
                        updated_at: None,
                    };
                    diesel::insert_into(threads::table)
                        .values(&new_thread)
                        .execute(conn)?;
                }
            }
        }

        diesel::insert_into(messages::table)
            .values(&new_message)
            .execute(conn)?;

        if !is_llm {
            info!("Message successfully inserted into the database: {:?}", new_message);
        }

        Ok::<(), diesel::result::Error>(())
    }).await.map_err(|e| CreateMessageError::InteractionError(e.to_string()))??;

    Ok(())
}
