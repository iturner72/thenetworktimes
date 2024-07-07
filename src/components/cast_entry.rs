use leptos::*;
use crate::models::farcaster::{Cast, UserDataResponse};

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
            .map_err(|e| UserDataError::FetchError(format!("Failed to fetch user data: {:?}", e)))
            .and_then(|json| {
                serde_json::from_value::<UserDataResponse>(json.0)
                    .map_err(|e| UserDataError::ParseError(format!("Failed to parse user data: {:?}", e)))
            })
            .map_err(to_server_error)
}

#[component]
pub fn CastEntry(cast: Cast) -> impl IntoView {
    let username_data = create_resource(
        move || (cast.data.fid, 6), // 6 is the user data type for username
        |(fid, user_data_type)| async move { get_user_data(fid, user_data_type).await }
    );

    let pfp_data = create_resource(
        move || (cast.data.fid, 1), // 1 is the user data type for profile picture
        |(fid, user_data_type)| async move { get_user_data(fid, user_data_type).await }
    );

    view! {
        <div class="bg-teal-800 p-4 shadow hover:bg-teal-900 transition duration-0">
            <div class="flex flex-col space-y-2">
                <div class="flex items-center space-x-2">
                    {move || match pfp_data() {
                        None => view! { <div class="w-10 h-10 bg-gray-300 rounded-full"></div> },
                        Some(Ok(pfp)) => view! {
                            <div>
                                <img
                                    src={pfp.data.user_data_body.value}
                                    alt="pfp"
                                    class="w-10 h-10 rounded-full"
                                />
                            </div>
                        },
                        Some(Err(_)) => view! {
                            <div>
                            <img
                                src={" "}
                                alt="pfp not found"
                                class="w-10 h-10 rounded-full"
                            />
                            </div>
                        },
                    }}
                    {move || match username_data.get() {
                        None => view! { <span class="ib text-pistachio-500">"Loading..."</span> },
                        Some(Ok(user)) => view! {
                            <span class="ib text-pistachio-500">{"@"}{user.data.user_data_body.value}</span>
                        },
                        Some(Err(_)) => view! { <span class="ib text-pistachio-500">"Unknown User"</span> },
                    }}
                </div>
                <p class="ir text-pistachio-200">
                    {cast.data.castAddBody.as_ref().and_then(|body| body.text.as_ref()).unwrap_or(&"N/A".to_string())}
                </p>
            </div>
        </div>
    }
}
