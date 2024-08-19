use leptos::*;
use leptos_router::A;
use crate::models::farcaster::{Cast, UserDataResponse};
use crate::components::cache_provider::ClientCache;
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

#[component]
pub fn CastEntry(
    cast: Cast,
    #[prop(into)] lazy_load_index: Signal<bool>,
) -> impl IntoView {
    let client_cache = use_context::<RwSignal<ClientCache>>().expect("ClientCache should be provided");
    let (user_data, set_user_data) = create_signal(None::<(String, String)>);
    let (is_visible, set_is_visible) = create_signal(false);
    let (cast_add_body, _set_cast_add_body) = create_signal(cast.data.castAddBody.clone());
    let (show_modal, set_show_modal) = create_signal(false);
    let (modal_image_url, set_modal_image_url) = create_signal(None::<String>);

    let load_user_data = create_action(move |_: &()| {
        let fid = cast.data.fid;
        let client_cache = client_cache.get();
        async move {
            if let Some(cached_data) = client_cache.get(fid) {
                log::debug!("using client cached data for fid: {}", fid);
                set_user_data(Some(cached_data));
            } else {
                log::debug!("fetching user data from server for fid: {}", fid);
                let username = get_user_data(fid, 6).await.ok()
                    .and_then(|response| Some(response.data.user_data_body.value));
                let pfp = get_user_data(fid, 1).await.ok()
                    .and_then(|response| Some(response.data.user_data_body.value));
    
                log::debug!("fetched data for fid {}: username: {:?}, pfp: {:?}", fid, username, pfp);
    
                if let (Some(username), Some(pfp)) = (username, pfp) {
                    log::info!("updating client cache and user data for fid: {}", fid);
                    client_cache.set(fid, username.clone(), pfp.clone());
                    set_user_data(Some((username, pfp)));
                } else {
                    log::warn!("failed to fetch user data for fid: {}", fid);
                }
            }
        }
    });

    create_effect(move |_| {
        if (lazy_load_index.get() || is_visible.get()) && user_data.get().is_none() {
            load_user_data.dispatch(());
        }
    });

    let element_ref = create_node_ref::<html::Div>();

    create_effect(move |_| {
        let element = element_ref.get().expect("div to be available");

        let observer_callback = Closure::wrap(Box::new(move |entries: Vec<IntersectionObserverEntry>, _: IntersectionObserver| {
            if let Some(entry) = entries.get(0) {
                if entry.is_intersecting() {
                    set_is_visible.set(true);
                }
            }
        }) as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);

        let mut options = IntersectionObserverInit::new();
        options.threshold(&JsValue::from(0.1));

        let observer = IntersectionObserver::new_with_options(
            observer_callback.as_ref().unchecked_ref(),
            &options,
        ).expect("failed to create IntersectionObserver");

        observer.observe(&element);

        on_cleanup(move || {
            observer.disconnect();
            drop(observer_callback);
        });
    });

    let processed_content = create_resource(
        move || cast_add_body.get().and_then(|body| body.text.clone()),
        |text| async move {
            match text {
                Some(content) => process_cast_content(content).await.unwrap_or_default(),
                None => vec!["no text".to_string()],
            }
        }
    );

    let open_modal = move |url: String| {
        set_modal_image_url(Some(url));
        set_show_modal(true);
    };

    let close_modal = move |_| {
        set_show_modal(false);
    };

    view! {
        <div class="cast-entry" node_ref=element_ref>
            {move || {
                match user_data.get() {
                    Some((username, pfp)) => view! {
                        <div class="user-info flex flex-row items-center justify-start space-x-2">

                            <A href=format!("/profile/{}", cast.data.fid)>
                                <img src={pfp} alt="profile" class="w-12 h-12 rounded-full cursor-pointer" />
                            </A>
                            <A href=format!("/profile/{}", cast.data.fid)>
                                <span class="username ib text-mint-700">{username}</span>
                            </A>
                        </div>
                    },
                    None => view! {
                        <div class="user-info-placeholder">
                            <div class="w-12 h-12 bg-aqua-800 rounded-full"></div>
                            <span class="username-placeholder bg-aqua-800 w-20 h-4"></span>
                        </div>
                    }
                }
            }}
            <div class="cast-content flex flex-col items-start pl-12">
                <Suspense fallback=move || view! { <p class="pt-2 ib text-mint-700">"loading..."</p> }>
                    {move || {
                        processed_content.get().map(|parts| {
                            view! {
                                <p class="ir text-md text-pistachio-200">
                                    {parts.into_iter().map(|part| {
                                        if part.starts_with("http") {
                                            view! {
                                                <a href={part.clone()} target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:underline">
                                                    {part}
                                                </a>
                                            }.into_view()
                                        } else {
                                            view! { <span>{part}</span> }.into_view()
                                        }
                                    }).collect::<Vec<_>>()}
                                </p>
                            }
                        })
                    }}
                </Suspense>

                {move || {
                    cast_add_body.get().and_then(|body| {
                        Some(body.embeds.iter().filter_map(|embed| {
                            embed.url.as_ref().map(|url| {
                                let url_clone = url.clone();
                                view! {
                                    <img
                                        src={url.clone()}
                                        alt="cast image"
                                        class="mt-2 max-w-sm h-auto rounded-lg cursor-pointer"
                                        on:click=move |_| open_modal(url_clone.clone())
                                    />
                                }
                            })
                        }).collect::<Vec<_>>())
                    })
                }}
            </div>

            {move || {
                if show_modal.get() {
                    view! {
                        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                            <div class="bg-gray-800 p-4 rounded-lg max-w-auto max-h-screen overflow-auto">
                                <ImageView url={modal_image_url.get().unwrap_or_default()} />
                                <button
                                    class="mt-4 px-4 py-2 bg-aqua-800 text-gray-500 hover:bg-gray-300"
                                    on:click=close_modal
                                >
                                    "Close"
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}

#[component]
fn ImageView(#[prop(into)] url: String) -> impl IntoView {
    view! {
        <img src={url} alt="Cast image" class="mt-2 max-w-lg max-h-screen object-contian rounded-lg" />
    }
}

#[server(ProcessCastContent, "/api")]
pub async fn process_cast_content(content:String) -> Result<Vec<String>, ServerFnError> {
    use regex::Regex;
    /*
     * apparently intersperse_with is feature gated and there's an open issue (i do not care), so i
     * needed to add the feature flag thing to my lib file to parse the content on the server. i
     * refuse to use the regex crate on the client! my wasm binary will still be large af i reckon,
     * oh well.
    */
    let url_regex = Regex::new(r"https?://\S+").unwrap();
    let parts: Vec<String> = url_regex.split(&content)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .intersperse_with(|| {
            url_regex.find(&content)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default()
        })
        .filter(|s| !s.is_empty())
        .collect();

    Ok(parts)
}

#[server(GetUserData, "/api")]
pub async fn get_user_data(fid: u64, user_data_type: u8) -> Result<UserDataResponse, ServerFnError> {
    use crate::services::hubble::{UserDataParams, get_user_data_by_fid};
    use crate::services::redis::{get_user_data_from_cache, set_user_data_to_cache};
    use crate::state::AppState;
    use axum::extract::Query;
    use std::fmt;
    use log::{debug, info, warn};

    #[derive(Debug)]
    enum UserDataError {
        CacheReadError(String),
        CacheWriteError(String),
        FetchError(String),
        ParseError(String),
    }
    
    impl fmt::Display for UserDataError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                UserDataError::CacheReadError(e) => write!(f, "cache read error: {}", e),
                UserDataError::CacheWriteError(e) => write!(f, "cache write error: {}", e),
                UserDataError::FetchError(e) => write!(f, "fetch error: {}", e),
                UserDataError::ParseError(e) => write!(f, "parse error: {}", e),
            }
        }
    }
    
    fn to_server_error(e: UserDataError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    debug!("get_user_data called with fid: {}, user_data_type: {}", fid, user_data_type);

    let app_state = use_context::<AppState>().expect("Failed to get AppState from context");
    let mut redis_conn = app_state.redis_pool.clone();

    let cache_key = format!("user_data:{}:{}", fid, user_data_type);

    // Check cache
    match get_user_data_from_cache(&mut redis_conn, &cache_key).await {
        Ok(Some(cached_data)) => {
            info!("cache hit for fid: {}, type: {}", fid, user_data_type);
            return Ok(cached_data);
        }
        Ok(None) => {
            debug!("cache miss for fid: {}, type: {}", fid, user_data_type);
        }
        Err(e) => {
            warn!("error reading from cache for fid {}, type {}: {}", fid, user_data_type, e);
            return Err(UserDataError::CacheReadError(e.to_string())).map_err(to_server_error);
        }
    }

    let params = UserDataParams {
        fid,
        user_data_type: Some(user_data_type.to_string()),
    };

    match get_user_data_by_fid(Query(params)).await {
        Ok(json) => {
            debug!("successfully fetched user data for fid: {}, type: {}", fid, user_data_type);
            match serde_json::from_value::<UserDataResponse>(json.0) {
                Ok(user_data) => {
                    // Update cache
                    if let Err(e) = set_user_data_to_cache(&mut redis_conn, &cache_key, &user_data).await {
                        warn!("failed to update cache for fid {}, type {}: {}", fid, user_data_type, e);
                        return Err(UserDataError::CacheWriteError(e.to_string())).map_err(to_server_error);
                    }
                    debug!("successfully updated cache for fid: {}, type: {}", fid, user_data_type);
                    Ok(user_data)
                }
                Err(e) => {
                    warn!("failed to parse user data for fid {}, type {}: {}", fid, user_data_type, e);
                    Err(UserDataError::ParseError(e.to_string())).map_err(to_server_error)
                }
            }
        }
        Err(e) => {
            warn!("failed to fetch user data for fid {}, type {}: {}", fid, user_data_type, e);
            Err(UserDataError::FetchError(e.to_string())).map_err(to_server_error)
        }
    }

}
