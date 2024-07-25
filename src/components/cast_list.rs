use leptos::*;
use crate::models::farcaster::Cast;

#[server(GetCastsByChannel, "/api")]
pub async fn get_casts_by_channel(channel: String, page: u64, limit: u64) -> Result<Vec<Cast>, ServerFnError> {
    use axum::extract::{Query, Path};
    use serde_json::Value;
    use std::collections::HashMap;
    use crate::services::hubble::get_casts_by_parent;
    use std::fmt;

    #[derive(Debug)]
    enum CastError {
        FetchError(String),
        ParseError(String),
    }

    impl fmt::Display for CastError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CastError::FetchError(e) => write!(f, "fetch error: {}", e),
                CastError::ParseError(e) => write!(f, "parse error: {}", e),
            }
        }
    }
    
    fn to_server_error(e: CastError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    let mut query_params = HashMap::new();
    query_params.insert("page".to_string(), page);
    query_params.insert("limit".to_string(), limit);

    let channel_url = format!("https://warpcast.com/~/channel/{}", channel);
    let encoded_channel_url = urlencoding::encode(&channel_url);

    let casts_response = get_casts_by_parent(
        Path(encoded_channel_url.to_string()),
        Query(query_params)
    )
    .await
    .map_err(|e| CastError::FetchError(format!("failed to fetch casts: {:?}", e)))
    .map_err(to_server_error)?;

    let cast_response: Value = serde_json::from_value(casts_response.0)
        .map_err(|e| CastError::ParseError(format!("failed to parse cast response: {:?}", e)))
        .map_err(to_server_error)?;

    let casts: Vec<Cast> = serde_json::from_value(cast_response["messages"].clone())
        .map_err(|e| CastError::ParseError(format!("failed to parse casts: {:?}", e)))
        .map_err(to_server_error)?;

    Ok(casts)
}

#[component]
pub fn CastList(
    active_channel: ReadSignal<String>
) -> impl IntoView {
    let (cast_list, set_cast_list) = create_signal(Vec::new());
    let (page, set_page) = create_signal(1u64);
    let (error, set_error) = create_signal(None::<String>);
    let (is_loading, set_is_loading) = create_signal(false);
    let (has_more, set_has_more) = create_signal(true);
    let limit = 40u64;

    let fetch_casts = create_action(move |_: &()| {
        let current_page = page.get();
        let current_channel = active_channel.get();
        async move {
            set_is_loading.set(true);
            match get_casts_by_channel(current_channel, current_page, limit).await {
                Ok(fetched_casts) => {
                    if fetched_casts.is_empty() {
                        set_has_more.set(false);
                    } else {
                        set_cast_list.update(|list| list.extend(fetched_casts));
                    }
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("failed to fetch casts: {}", e)));
                }
            }
            set_is_loading.set(false);
        }
    });

    create_effect(move |_| {
        active_channel.track();
        set_cast_list.set(Vec::new());
        set_page.set(1);
        set_has_more.set(true);
        set_error.set(None);
        fetch_casts.dispatch(());
    });

    let load_more = move |_| {
        if !is_loading.get() && has_more.get() {
            set_page.update(|p| *p += 1);
            fetch_casts.dispatch(());
        }
    };

    view! {
        <div class="cast-list w-11/12 lg:w-8/12 xl:w-3/12 mx-auto">
            <h2 class="text-2xl ib text-gray-700 hover:text-gray-900 pb-8">
                <a href={move || format!("https://warpcast.com/~/channel/{}", active_channel.get())} target="_blank" rel="noopener noreferrer">
                    {move || format!("/{}", active_channel.get())}
                </a>
            </h2>
            {move || error.get().map(|err| view! { <p class="text-red-500">{err}</p> })}
            <div class="space-y-4">
                <For
                    each=move || cast_list.get()
                    key=|cast| cast.hash.clone()
                    children=move |cast| {
                        view! {
                            <div class="cast-item flex flex-col items-center justify-between bg-teal-800 p-4 shadow hover:bg-teal-900 border-2 border-teal-900 hover:border-teal-800 transition duration-0">
                                <p class="ib text-md text-pistachio-500">"author fid: "{cast.data.fid}</p>
                                <p class="ir text-md text-pistachio-200">
                                    {cast.data.castAddBody.as_ref().and_then(|body| body.text.as_ref()).unwrap_or(&String::from("No text"))}
                                </p>
                                {cast.data.castAddBody.as_ref().and_then(|body| {
                                    body.embeds.first().and_then(|embed| embed.url.as_ref().map(|url| {
                                        view! {
                                            <div class="h-64 w-64 overflow-hidden">
                                                <img src={url.clone()} alt="embedded content" class="w-full h-full object-cover" />
                                            </div>
                                        }
                                    }))
                                })}
                            </div>
                        }
                    }
                />
            </div>
            <div>
                {move || {
                    if is_loading.get() {
                        view! { <div><p class="text-mint-700">"loading..."</p></div> }
                    } else if has_more.get() {
                        view! {
                            <div>
                                <button
                                    on:click=load_more
                                    class="alumni-sans-regular mt-4 px-4 py-2 bg-stone-700 text-white hover:bg-stone-600"
                                >
                                    "load more"
                                </button>
                            </div>
                        }
                    } else {
                        view! { <div><p class="text-indigo-300">"no more casts to load."</p></div> }
                    }
                }}
            </div>
        </div>
    }
}
