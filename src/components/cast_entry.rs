use leptos::*;
use leptos_router::A;
use crate::models::farcaster::{Cast, UserDataResponse};
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

#[server(GetUserData, "/api")]
pub async fn get_user_data(fid: u64, user_data_type: u8) -> Result<UserDataResponse, ServerFnError> {
    use crate::services::hubble::{UserDataParams, get_user_data_by_fid};
    use axum::extract::Query;
    use std::fmt;

    #[derive(Debug)]
    enum UserDataError {
        FetchError(String),
        ParseError(String),
    }
    
    impl fmt::Display for UserDataError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                UserDataError::FetchError(e) => write!(f, "Fetch error: {}", e),
                UserDataError::ParseError(e) => write!(f, "Parse error: {}", e),
            }
        }
    }
    
    fn to_server_error(e: UserDataError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }
        let params = UserDataParams {
            fid,
            user_data_type: Some(user_data_type.to_string()),
        };
    
        get_user_data_by_fid(Query(params))
            .await
            .map_err(|e| UserDataError::FetchError(format!("failed to fetch user data: {:?}", e)))
            .and_then(|json| {
                serde_json::from_value::<UserDataResponse>(json.0)
                    .map_err(|e| UserDataError::ParseError(format!("failed to parse user data: {:?}", e)))
            })
            .map_err(to_server_error)
}

#[component]
pub fn CastEntry(
    cast: Cast,
    #[prop(into)] lazy_load_index: Signal<bool>,
) -> impl IntoView {
    let (user_data, set_user_data) = create_signal(None::<(String, String)>);
    let (is_visible, set_is_visible) = create_signal(false);

    let load_user_data = create_action(move |_: &()| {
        let fid = cast.data.fid;
        async move {
            let username = get_user_data(fid, 6).await.ok()
                .and_then(|response| Some(response.data.user_data_body.value));
            let pfp = get_user_data(fid, 1).await.ok()
                .and_then(|response| Some(response.data.user_data_body.value));
            if let (Some(username), Some(pfp)) = (username, pfp) {
                set_user_data(Some((username, pfp)));
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

    view! {
        <div class="cast-entry" node_ref=element_ref>
            {move || {
                match user_data.get() {
                    Some((username, pfp)) => view! {
//                        <div class="user-info flex flex-row items-center justify-start space-x-2">
//                            <img src={pfp} alt="Profile" class="w-12 h-12 rounded-full" />
//                            <span class="username ib text-mint-700">{username}</span>
//                        </div>

                        <div class="user-info flex flex-row items-center justify-start space-x-2">

                            <A href=format!("/profile/{}", cast.data.fid)>
                                <img src={pfp} alt="Profile" class="w-12 h-12 rounded-full cursor-pointer" />
                            </A>
                            <A href=format!("/profile/{}", cast.data.fid)>
                                <span class="username ib text-mint-700">{username}</span>
                            </A>
                        </div>
                    },
                    None => view! {
                        <div class="user-info-placeholder">
                            <div class="w-12 h-12 bg-gray-800 rounded-full"></div>
                            <span class="username-placeholder bg-gray-800 w-20 h-4"></span>
                        </div>
                    }
                }
            }}
            <div class="cast-content flex flex-col items-start pl-12">
                <p class="ir text-md text-pistachio-200">
                    {cast.data.castAddBody.as_ref().and_then(|body| body.text.as_ref()).unwrap_or(&String::from("no text"))}
                </p>
                {move || {
                    cast.data.castAddBody.as_ref().map(|body| {
                        body.embeds.iter().filter_map(|embed| {
                            embed.url.as_ref().map(|url| {
                                view! {
                                    <img src={url.clone()} alt="embedded content" class="mt-2 max-w-md object-contain h-auto rounded-lg" />
                                }
                            })
                        }).collect::<Vec<_>>()
                    })
                }}
            </div>
        </div>
    }
}
