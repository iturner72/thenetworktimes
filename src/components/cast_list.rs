use leptos::*;
use crate::models::farcaster::Cast;
use crate::components::cast_entry::CastEntry;

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
    let limit = 4u64;

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
        <div class="channel-casts-container w-11/12 lg:w-8/12 xl:w-3/12 mx-auto">
            <h2 class="text-2xl ib text-gray-700 hover:text-gray-900 pb-6">
                <a href={move || format!("https://warpcast.com/~/channel/{}", active_channel.get())} target="_blank" rel="noopener noreferrer">
                    {move || format!("/{}", active_channel.get())}
                </a>
            </h2>
            {move || error.get().map(|err| view! { <p class="text-red-500">{err}</p> })}
            <div class="cast-list">
                <For
                    each=move || cast_list.get()
                    key=|cast| cast.hash.clone()
                    children=move |cast| {
                        let index = cast_list.with(|list| list.iter().position(|c| c.hash == cast.hash).unwrap_or(0));
                        view! {
                            <div class=move || format!("border-l border-r border-b last:border-b-0 border-seafoam-800 {} p-4 transition-colors duration-0 ease-in-out hover:bg-teal-800",
                                if index == 0 { "border-t" } else { "" }
                            )>
                                <CastEntry 
                                    cast=cast 
                                    lazy_load_index=Signal::derive(move || index < 8)
                                />
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
