use leptos::*;
use crate::models::farcaster::Cast;
use log::error;


#[server(GetCastsByChannel, "/api")]
pub async fn get_casts_by_channel(channel: String, page: u64, limit: u64) -> Result<Vec<Cast>, ServerFnError> {
    use crate::services::hubble::get_casts_by_parent;
    use crate::models::farcaster::CastResponse;
    use axum::extract::{Query, Path};
    use std::collections::HashMap;

    #[derive(Debug)]
    enum CastError {
        FetchError(String),
        ParseError(String),
    }
    
    impl std::fmt::Display for CastError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                CastError::FetchError(e) => write!(f, "Fetch error: {}", e),
                CastError::ParseError(e) => write!(f, "Parse error: {}", e),
            }
        }
    }
    
    fn to_server_error(e: CastError) -> ServerFnError {
        ServerFnError::ServerError(e.to_string())
    }

    let _offset = (page - 1) * limit;
    let mut query_params = HashMap::new();
    query_params.insert("page".to_string(), page);
    query_params.insert("limit".to_string(), limit);

    let casts_response = get_casts_by_parent(
        Path(channel),
        Query(query_params)
    )
    .await
    .map_err(|e| CastError::FetchError(e.to_string()))
    .map_err(to_server_error)?;

    let cast_response: CastResponse = serde_json::from_value(casts_response.0)
        .map_err(|e| CastError::ParseError(e.to_string()))
        .map_err(to_server_error)?;

    Ok(cast_response.messages)
}

#[component]
pub fn CastList() -> impl IntoView {
    let (cast_list, set_cast_list) = create_signal(Vec::new());
    let (page, set_page) = create_signal(1u64);
    let limit = 40u64;
    
    let fetch_casts = create_action(move |_: &()| {
        let current_page = page.get();
        async move {
            let channel = "networktimes".to_string(); // Hardcoded channel for testing
            match get_casts_by_channel(channel, current_page, limit).await {
                Ok(fetched_casts) => {
                    set_cast_list.update(|list| list.extend(fetched_casts));
                }
                Err(e) => {
                    error!("Failed to fetch casts: {:?}", e);
                }
            }
        }
    });
    
    // Initial fetch
    fetch_casts.dispatch(());
    
    let load_more = move |_| {
        set_page.update(|p| *p += 1);
        fetch_casts.dispatch(());
    };

    view! {
        <div class="cast-list">
            <h2>"Casts for networktimes"</h2>
            <div>
                {move || 
                    cast_list.get().iter().map(|cast| {
                        view! {
                            <div class="cast-item">
                                <p>"Author FID: "{cast.data.fid}</p>
                                <p>{cast.data.cast_add_body.as_ref().and_then(|body| body.text.as_ref()).unwrap_or(&"No text".to_string())}</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }
            </div>
            <button on:click=move |_| load_more(())>"Load More"</button>
        </div>
    }
}
