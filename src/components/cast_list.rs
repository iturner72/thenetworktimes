use leptos::*;
use crate::models::farcaster::{Cast, CastResponse};

#[server(GetCastsByChannel, "/api")]
pub async fn get_casts_by_channel(channel: String, page: u64, limit: u64) -> Result<CastResponse, ServerFnError> {
    todo!("fetch casts by channel")
}

#[component]
pub fn CastList() -> impl IntoView {
    let (channel, set_channel) = create_signal("networktimes".to_string());
    let (cast_list, set_cast_list) = create_signal(Vec::new());
    let (page, set_page) = create_signal(1u64);
    let (error, set_error) = create_signal(None::<String>);
    let (is_loading, set_is_loading) = create_signal(false);
    let (has_more, set_has_more) = create_signal(true);
    let limit = 40u64;

    let fetch_casts = create_action(move |_: &()| {
        let current_page = page.get();
        let current_channel = channel.get();
        async move {
            set_is_loading.set(true);
            match get_casts_by_channel(current_channel, current_page, limit).await {
                Ok(response) => {
                    if response.messages.is_empty() {
                        set_has_more.set(false);
                    } else {
                        set_cast_list.update(|list| list.extend(response.messages));
                    }
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to fetch casts: {}", e)));
                }
            }
            set_is_loading.set(false);
        }
    });

    create_effect(move |_| {
        channel.track();
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
        <div class="cast-list">
            <h2 class="text-2xl alumni-sans-bold text-indigo-800 hover:underline">
                <a href={move || format!("https://warpcast.com/~/channel/{}", channel.get())} target="_blank" rel="noopener noreferrer">
                    {move || format!("/{}", channel.get())}
                </a>
            </h2>
            {move || error.get().map(|err| view! { <p class="text-red-500">{err}</p> })}
            <div class="space-y-4">
                <For
                    each=move || cast_list.get()
                    key=|cast| cast.hash.clone()
                    children=move |cast| {
                        view! {
                            <div class="cast-item bg-teal-800 p-4 shadow hover:bg-teal-900 transition duration-0">
                                <p class="ib text-pistachio-500">"Author FID: "{cast.data.fid}</p>
                                <p class="ir text-pistachio-200">
                                    {cast.data.cast_add_body.as_ref().and_then(|body| body.text.as_ref()).unwrap_or(&"No text")}
                                </p>
                                <p class="ir text-xs text-salmon-400">
                                    {"Timestamp: "}{cast.data.timestamp}
                                </p>
                                <p class="ir text-xs text-salmon-400">
                                    {"Network: "}{&cast.data.network}
                                </p>
                                <p class="ir text-xs text-salmon-400">
                                    {"Type: "}{&cast.data.cast_type}
                                </p>
                                {cast.data.cast_add_body.as_ref().map(|body| view! {
                                    <div>
                                        <p class="ir text-xs text-salmon-400">
                                            {"Mentions: "}{body.mentions.len()}
                                        </p>
                                        <p class="ir text-xs text-salmon-400">
                                            {"Embeds: "}{body.embeds.len()}
                                        </p>
                                    </div>
                                })}
                            </div>
                        }
                    }
                />
            </div>
            <div>
                {move || {
                    if is_loading.get() {
                        view! { <p class="text-indigo-300">"Loading..."</p> }
                    } else if has_more.get() {
                        view! {
                            <button
                                on:click=load_more
                                class="alumni-sans-regular mt-4 px-4 py-2 bg-stone-700 text-white hover:bg-stone-600"
                            >
                                "Load More"
                            </button>
                        }
                    } else {
                        view! { <p class="text-indigo-300">"No more casts to load."</p> }
                    }
                }}
            </div>
        </div>
    }
}
