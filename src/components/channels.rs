use cfg_if::cfg_if;
use leptos::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;
use std::collections::HashSet;
use log::info;
use crate::models::farcaster::{Channel, ChannelsResponse};

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use gloo_net::http::Request;
        use crate::models::farcaster::UserDataResponse;

        async fn fetch_channels() -> Result<ChannelsResponse, String> {
            match Request::get("/api/channels").send().await {
                Ok(response) => {
                    match response.json::<ChannelsResponse>().await {
                        Ok(data) => {
                            info!("Channels fetched successfully: {:?}", data);
                            Ok(data)
                        },
                        Err(err) => {
                            info!("Failed to parse channels: {:?}", err);
                            Err(format!("Failed to parse channels: {:?}", err))
                        }
                    }
                },
                Err(err) => {
                    info!("Failed to fetch channels: {:?}", err);
                    Err(format!("Failed to fetch channels: {:?}", err))
                }
            }
        }

        async fn fetch_username(fid: u64, lead_usernames: Signal<HashMap<u64, String>>) -> Result<String, String> {
            if lead_usernames.with_untracked(|usernames| !usernames.contains_key(&fid)) {
                match Request::get(&format!("https://api.thenetworktimes.xyz/userDataByFid?fid={}&user_data_type=USER_DATA_TYPE_USERNAME", fid)).send().await {
                    Ok(response) => {
                        match response.json::<UserDataResponse>().await {
                            Ok(data) => {
                                let username = if data.data.user_data_body.data_type == "USER_DATA_TYPE_USERNAME" {
                                    Some(data.data.user_data_body.value)
                                } else {
                                    None
                                }.ok_or_else(|| "Username not found".to_string())?;
                                info!("Username for fid {}: {}", fid, username);
                                Ok(username)
                            },
                            Err(err) => {
                                info!("Failed to parse user data for fid {}: {:?}", fid, err);
                                Err(format!("Failed to parse user data for fid {}: {:?}", fid, err))
                            }
                        }
                    },
                    Err(err) => {
                        info!("Failed to fetch user data for fid {}: {:?}", fid, err);
                        Err(format!("Failed to fetch user data for fid {}: {:?}", fid, err))
                    }
                }
            } else {
                Ok(lead_usernames.with_untracked(|usernames| usernames.get(&fid).unwrap().clone()))
            }
        }
    } else {
        async fn fetch_channels() -> Result<ChannelsResponse, String> {
            Err("Fetching channels not supported on server side".into())
        }

        async fn fetch_username(_fid: u64, _lead_usernames: Signal<HashMap<u64, String>>) -> Result<String, String> {
            Err("Fetching usernames not supported on server side".into())
        }
    }
}

#[allow(deprecated)]
fn format_date(timestamp: u64) -> String {
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    datetime.format("%B '%y").to_string().to_lowercase()
    
}

#[component]
pub fn Channels(
    set_active_channel: WriteSignal<String>
) -> impl IntoView {
    let (channels, set_channels) = create_signal(Vec::new());
    let (lead_usernames, set_lead_usernames) = create_signal(HashMap::new());
    let (error_message, set_error_message) = create_signal(None);
    let (search_query, set_search_query) = create_signal(String::new());
    let ongoing_requests = std::cell::RefCell::new(HashSet::new());

    let desired_channels = [
        "https://warpcast.com/~/channel/onthebrink".to_string(),
        "https://warpcast.com/~/channel/piratewires".to_string(),
        "https://warpcast.com/~/channel/moz".to_string(),
        "https://warpcast.com/~/channel/networktimes".to_string(),
        "https://warpcast.com/~/channel/gray".to_string(),
        "https://warpcast.com/~/channel/all-in".to_string(),
    ];


    spawn_local(async move {
        match fetch_channels().await {
            Ok(data) => {
                let filtered_channels: Vec<Channel> = data.result.channels.into_iter()
                    .filter(|channel| desired_channels.contains(&channel.url))
                    .collect();
                info!("Filtered channels: {:?}", filtered_channels);
                set_channels(filtered_channels);
            },
            Err(err) => set_error_message(Some(err)),
        }
    });

    let filtered_channels = create_memo(move |_| {
        let query = search_query().to_lowercase();
        channels().into_iter()
            .filter(|channel|
                channel.id.to_lowercase().contains(&query) ||
                channel.description.to_lowercase().contains(&query)
            )
            .collect::<Vec<_>>()
    });

    create_effect({
        let ongoing_requests = ongoing_requests.clone();
        move |_| {
            for channel in channels().iter() {
                let fid = channel.leadFid;
                if !lead_usernames().contains_key(&fid) && !ongoing_requests.borrow().contains(&fid) {
                    ongoing_requests.borrow_mut().insert(fid);
                    let ongoing_requests = ongoing_requests.clone();
                    spawn_local(async move {
                        match fetch_username(fid, lead_usernames.into()).await {
                            Ok(username) => {
                                set_lead_usernames.update(|usernames| {
                                    usernames.insert(fid, username);
                                });
                            },
                            Err(err) => set_error_message(Some(err)),
                        }
                        ongoing_requests.borrow_mut().remove(&fid);
                    });
                }
            }
        }
    });

    view! {
        <div class="channels-component-view w-7/12 md:w-3/12 xl:w-2/12 p-2 mx-auto">
            <h1 class="text-2xl font-bold text-teal-600 dark:text-mint-400 text-center mb-4">"Channels"</h1>
            <input
                type="text"
                placeholder="grep channel, bio"
                on:input=move |ev| set_search_query(event_target_value(&ev))
                class="w-full p-2 mb-4 bg-gray-100 dark:bg-teal-700 text-gray-800 dark:text-gray-200 border border-gray-300 dark:border-teal-600 rounded-md focus:outline-none focus:ring-2 focus:ring-seafoam-500 dark:focus:ring-aqua-400"
            />
            {move || error_message().map(|err| view! { <p class="text-salmon-600 dark:text-salmon-400">{err}</p> })}
    
            <ul class="channels-list flex flex-col space-y-2">
                {move || {
                    filtered_channels()
                        .iter()
                        .map(|channel| {
                            let fid = channel.leadFid;
                            let channel_id = channel.id.clone();
                            view! {
                                <button
                                    class="channel-item bg-gray-200 dark:bg-teal-800 p-3 rounded-md shadow hover:bg-gray-300 dark:hover:bg-teal-700 transition duration-300 group relative"
                                    on:click=move |_| set_active_channel(channel_id.clone())
                                >
                                    <div class="channel-item-info-container flex flex-col items-start">
                                        <div class="channel-avatar-chip flex flex-row items-center justify-between text-center space-x-4">
                                            <img
                                                src=channel.imageUrl.clone()
                                                alt=channel.id.clone()
                                                class="w-10 h-10 rounded-full"
                                            />
                                            <div class="title-v-stack flex flex-col items-start">
                                                <a
                                                    href=channel.url.clone()
                                                    class="text-base text-seafoam-600 dark:text-aqua-400 hover:text-seafoam-700 dark:hover:text-aqua-300 pb-2"
                                                >
                                                    {&channel.id}
                                                </a>
                                                {move || match lead_usernames.get().get(&fid) {
                                                    Some(username) => {
                                                        view! { <p class="text-xs text-gray-600 dark:text-gray-400">{username}</p> }
                                                    }
                                                    None => {
                                                        view! { <p class="text-xs text-gray-600 dark:text-gray-400">"chill"</p> }
                                                    }
                                                }}
                                            </div>
                                        </div>
                                        <div class="description-v-stack hidden group-hover:flex flex-col items-start justify-center text-left mt-4 w-full absolute top-0 left-1/2 ml-2 z-50 bg-white dark:bg-teal-900 p-4 rounded-md shadow-lg">
                                            <p class="text-gray-700 dark:text-gray-300 text-xs w-full">
                                                {&channel.description}
                                            </p>
                                            {channel
                                                .moderatorFid
                                                .map(|fid| {
                                                    view! {
                                                        <p class="text-sm text-gray-600 dark:text-gray-400">
                                                            {"moderator fid: "} {fid}
                                                        </p>
                                                    }
                                                })}
                                            <p class="text-xs text-teal-600 dark:text-teal-400">
                                                {"created: "} {format_date(channel.createdAt)}
                                            </p>
                                            <p class="text-xs text-seafoam-600 dark:text-seafoam-400">
                                                {"followers: "} {channel.followerCount}
                                            </p>
                                        </div>
                                    </div>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </ul>
        </div>
    }
}
