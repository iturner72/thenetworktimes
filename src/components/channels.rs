use cfg_if::cfg_if;
use leptos::*;
use leptos_router::A;
use chrono::{DateTime, Utc};
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
    let naive_datetime = chrono::NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    datetime.format("%B %Y").to_string()
}

#[component]
pub fn Channels() -> impl IntoView {
    let (channels, set_channels) = create_signal(Vec::new());
    let (lead_usernames, set_lead_usernames) = create_signal(HashMap::new());
    let (error_message, set_error_message) = create_signal(None);

    let desired_channels = vec![
        "https://warpcast.com/~/channel/onthebrink".to_string(),
        "https://warpcast.com/~/channel/piratewires".to_string(),
        "https://warpcast.com/~/channel/moz".to_string(),
        "https://warpcast.com/~/channel/networktimes".to_string(),
        "https://warpcast.com/~/channel/gray".to_string(),
        "https://warpcast.com/~/channel/all-in".to_string(),
    ];

    let ongoing_requests = std::cell::RefCell::new(HashSet::new());

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

    create_effect({
        let channels = channels.clone();
        let lead_usernames = lead_usernames.clone();
        let set_error_message = set_error_message.clone();
        let ongoing_requests = ongoing_requests.clone();
        move |_| {
            for channel in channels().iter() {
                let fid = channel.leadFid;
                if !lead_usernames().contains_key(&fid) && !ongoing_requests.borrow().contains(&fid) {
                    ongoing_requests.borrow_mut().insert(fid);
                    let set_lead_usernames = set_lead_usernames.clone();
                    let lead_usernames = lead_usernames.clone();
                    let set_error_message = set_error_message.clone();
                    let ongoing_requests = ongoing_requests.clone();
                    spawn_local(async move {
                        match fetch_username(fid, lead_usernames.clone().into()).await {
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
        <div class="w-11/12 lg:w-8/12 xl:w-5/12 p-6 mx-auto">
            <h1 class="text-2xl ib text-salmon-300 text-center mb-4">"Channels"</h1>
            {move || error_message().map(|err| view! {
                <p class="text-salmon-800">{err}</p>
            })}
            <ul class="space-y-2">
                {move || channels().iter().map(|channel| {
                    let fid = channel.leadFid;
                    view! {
                        <li class="bg-teal-800 p-4 shadow hover:bg-teal-900 transition duration-0">
                            <div class="flex flex-col md:flex-row items-center md:items-center space-y-4 md:space-y-0 md:space-x-12">
                                <div class="flex-shrink-0 flex flex-col items-center md:items-start justify-center text-center md:text-left w-40">
                                    <img src={channel.imageUrl.clone()} alt={channel.id.clone()} class="w-16 h-16 rounded-full mb-2"/>
                                    <p class="ir text-sm text-mint-500">{"followers: "}{channel.followerCount}</p>
                                    {move || match lead_usernames.get().get(&fid) {
                                        Some(username) => view! {
                                            <p class="ib text-sm text-mint-700">{username}</p>
                                        },
                                        None => view! {
                                            <p class="ib text-sm text-mint-700">"chill"</p>
                                        },
                                    }}
                                    {channel.moderatorFid.map(|fid| view! {
                                        <p class="ib text-sm text-mint-700">{"moderator fid: "}{fid}</p>
                                    })}
                                    <p class="ir text-xs text-salmon-400">{"created: "}{format_date(channel.createdAt)}</p>
                                </div>
                                <div class="flex-grow flex flex-col items-center md:items-start justify-center text-center md:text-left">
                                    <a href={channel.url.clone()} class="ib text-pistachio-500 hover:text-pistachio-500 text-xl pb-2">{&channel.id}</a>
                                    <p class="ir text-pistachio-200 text-base w-full">{&channel.description}</p>
                                    <A href=format!("/casts/{}", channel.url.replace("https://warpcast.com/~/channel/", "")) class="ib text-salmon-600 hover:text-salmon-700 text-sm mt-2">
                                        "view casts"
                                    </A>
                                </div>
                            </div>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
