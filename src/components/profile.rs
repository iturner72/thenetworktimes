use leptos::*;
use leptos_router::*;
use crate::models::farcaster::UserDataResponse;

#[server(GetProfile, "/api")]
pub async fn get_profile(fid: u64, user_data_type: u8) -> Result<UserDataResponse, ServerFnError> {
    use crate::services::hubble::{UserDataParams, get_user_data_by_fid};
    use axum::extract::Query;
    use log::{info, error};
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

    info!("Getting profile for fid {} and user data type {}", fid, user_data_type);

    let params = UserDataParams {
        fid,
        user_data_type: Some(user_data_type.to_string()),
    };

    match get_user_data_by_fid(Query(params)).await {
        Ok(json) => {
            info!("Successfully fetched user data");
            match serde_json::from_value::<UserDataResponse>(json.0) {
                Ok(data) => {
                    info!("Successfully parsed user data");
                    Ok(data)
                },
                Err(e) => {
                    error!("Failed to parse user data: {:?}", e);
                    Err(to_server_error(UserDataError::ParseError(format!("Failed to parse user data: {:?}", e))))
                },
            }
        },
        Err(e) => {
            error!("Failed to fetch user data: {:?}", e);
            Err(to_server_error(UserDataError::FetchError(format!("Failed to fetch user data: {:?}", e))))
        },
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let params = use_params_map();
    let fid = create_memo(move |_| {
        params.with(|params| params.get("id").cloned().unwrap_or_default().parse::<u64>().unwrap_or(0))
    });

    let user_data: Resource<u64, Result<(UserDataResponse, UserDataResponse), ServerFnError>> = create_resource(
        move || fid(),
        |fid| async move {
            let username = get_profile(fid, 6).await?;
            let pfp = get_profile(fid, 1).await?;
            Ok((username, pfp))
        }
    );

    view! {


    <Suspense fallback=|| view! { <div class="text-3xl text-mint-700">"loading..."</div> }>
        {move || match user_data.get() {
            None => view! { <div class="text-pistachio-500">"Loading..."</div> },
            Some(Ok((username, pfp))) => view! {
                <div>
                    <div>
                        <span class="ib text-7xl text-gray-700">{"@"}{username.data.user_data_body.value}</span>
                    </div>
                    <div>
                        <img src={pfp.data.user_data_body.value} alt="pfp" class="profile-pic" />
                    </div>
                </div>
            },
            Some(Err(_)) => view! { <div class="text-pistachio-500">"Error loading user data"</div> },
        }}
    </Suspense>

    }
}
