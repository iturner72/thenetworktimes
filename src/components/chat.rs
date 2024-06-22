use cfg_if::cfg_if;
use leptos::*;
use log::error;
use urlencoding;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventSource, MessageEvent, ErrorEvent};
use std::rc::Rc;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use axum::response::sse::Event;
		use anyhow::{anyhow, Error};
		use reqwest::Client;
		use serde::{Deserialize, Serialize};
		use regex::Regex;
		use std::env;
		use std::pin::Pin;
		use std::task::{Context, Poll};
		use tokio::sync::mpsc;
		use futures::stream::{Stream, StreamExt};
		use log::info;

		pub struct SseStream {
			pub receiver: mpsc::Receiver<Result<Event, anyhow::Error>>,
		}

		impl Stream for SseStream {
			type Item = Result<Event, anyhow::Error>;

			fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
				self.receiver.poll_recv(cx)
			}
		}

		#[derive(Debug, Deserialize, Serialize)]
		struct ChatCompletionResponse {
			choices: Vec<Choice>,
		}

		#[derive(Debug, Deserialize, Serialize)]
		struct Choice {
			delta: Delta,
		}

		#[derive(Debug, Deserialize, Serialize)]
		struct Delta {
			content: Option<String>,
		}

		#[derive(Clone)]
		pub struct OpenAIService {
			client: Client,
			api_key: String,
			model: String,
		}

		impl OpenAIService {
			pub fn new(model: String) -> Self {
				let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
				let client = Client::new();
				OpenAIService { client, api_key, model }
			}

			pub async fn send_message(&self, message: String, tx: mpsc::Sender<Result<Event, anyhow::Error>>) -> Result<(), Error> {
				info!("Sending message to OpenAI API");
				let response = self.client.post("https://api.openai.com/v1/chat/completions")
					.header("Authorization", format!("Bearer {}", self.api_key))
					.header("Content-Type", "application/json")
					.json(&serde_json::json!({
						"model": self.model,
						"messages": [{"role": "user", "content": message}],
                        "max_tokens": 360,
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
                                    tx.send(Ok(Event::default().data("[DONE]".to_string()))).await.ok();
                                    break;
                                } else if line.trim().starts_with("data: ") {
                                    let json_str = &line.trim()[6..];
                                    let re = Regex::new(r#""content":"([^"]*)""#).unwrap();
                                    for cap in re.captures_iter(json_str) {
                                        let content = cap[1].to_string();
//                                        info!("Extracted content: {}", content);
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

		pub async fn send_message_stream(message: String, tx: mpsc::Sender<Result<Event, anyhow::Error>>) {
			log::info!("send_message_stream function called with message: {}", message);
            let decoded_message = urlencoding::decode(&message).expect("Failed to decode message");
			let openai_service = OpenAIService::new("gpt-3.5-turbo".to_string());
			if let Err(e) = openai_service.send_message(decoded_message.into_owned(), tx).await {
				error!("Error in send_message_stream: {}", e);
			}
		}
	} else {
		#[server(SendMessage, "/api", "Url", "send_message")]
		pub async fn send_message(message: String) -> Result<String, ServerFnError> {
			Err("Function not available in CSR".to_string())
		}
	}
}

#[component]
pub fn Chat() -> impl IntoView {
	let (message, set_message) = create_signal(String::new());
	let (response, set_response) = create_signal(String::new());
	let (is_sending, set_is_sending) = create_signal(false);

	let send_message_action = move |_| {
		let message_value = message.get();
		let set_response = set_response.clone();
		let set_is_sending = set_is_sending.clone();

		spawn_local(async move {
			set_is_sending(true);
			set_response.set("".to_string());

            let event_source = Rc::new(EventSource::new(&format!("/api/send_message_stream?message={}", urlencoding::encode(&message_value))).expect("Failed to connect to SSE endpoint"));

			let on_message = {
				let event_source = Rc::clone(&event_source);
				Closure::wrap(Box::new(move |event: MessageEvent| {
					let data = event.data().as_string().unwrap();
					if data == "[DONE]" {
						set_is_sending.set(false);
						event_source.close();
					} else {
						set_response.update(|resp| {
                            let processed_data = data.replace("\\n", "\n");
							resp.push_str(&processed_data);
							resp.to_string();
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
		});
	};

	view! {
		<div class="flex flex-col items-center justify-center space-y-8">
			<h1 class="ib text-salmon-300 text-lg pt-8 pb-2">"yap yap yap"</h1>
			<div class="text-left border border-gray-900 p-4 w-7/12 h-auto">
				<Suspense fallback=|| view! { <p class="ir text-base text-pistachio-500">"loading..."</p> }>
					{move || {
						view! {
							<p class="ir text-pistachio-500 whitespace-pre-wrap">{response.get()}</p>
						}
					}}
				</Suspense>
			</div>
			<div class="flex flex-row items-center justify-center space-x-4 w-7/12">
				<textarea
					class="ir text-sm text-pistachio-500 bg-teal-800 border border-teal-600 w-full h-10 p-2"
					value={move || message.get()}
					on:input={move |event| set_message(event_target_value(&event))}
				></textarea>
				<button
					class="ib text-salmon-300 text-lg w-1/6"
					on:click=send_message_action
					disabled={move || is_sending.get()}
				>
					{move || if is_sending.get() { "thinking..." } else { "send it" }}
				</button>
			</div>
		</div>
	}
}
