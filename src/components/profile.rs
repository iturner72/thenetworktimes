use leptos::*;
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
    let username_data = create_resource(
        move || (249222, 6),
        |(fid, user_data_type)| async move { get_profile(fid, user_data_type).await }
    );
    
    let pfp_data = create_resource(
        move || (249222, 1),
        |(fid, user_data_type)| async move { get_profile(fid, user_data_type).await }
    );

    view! {
        <Suspense fallback=|| view! { <div class="text-3xl text-mint-700">"loading..."</div> }>
            <div>
                {move || match username_data() {
                    None => view! { <span class="ib text-pistachio-500">"chill"</span> },
                    Some(Ok(user)) => view! {
                        <span class="ib text-7xl text-gray-700">{"@"}{user.data.user_data_body.value}</span>
                    },
                    Some(Err(_)) => view! { <span class="ib text-pistachio-500">"unknown user"</span> },
                }}
            </div>
            <div>
                {move || match pfp_data() {
                    None => view! { <img src="favicon.ico" /> },
                    Some(Ok(user)) => view! {
                        <img src={user.data.user_data_body.value} alt="pfp" class="profile-pic" />
                    },
                    Some(Err(_)) => view! { <img src="favicon.ico" /> },
                }}
            </div>
        </Suspense>
    }
}

