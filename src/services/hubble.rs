use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            extract::{Path, Query},
            http::StatusCode,
            response::Json,
        };
        use reqwest::Client;
        use serde::Deserialize;
        use serde_json::Value;
        use std::collections::HashMap;
        use std::env;
        use tracing::log::info;
        
        #[derive(Deserialize)]
        pub struct UserDataParams {
            pub fid: u64,
            pub user_data_type: Option<String>,
        }
        
        #[derive(Deserialize)]
        pub struct ReactionsByCastParams {
            pub target_fid: u64,
            pub target_hash: String,
            pub reaction_type: Option<String>,
        }
        
        pub async fn get_username_proofs_by_fid(Path(fid): Path<u64>) -> Result<Json<Value>, StatusCode> {
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let url = format!("{}:2281/v1/userNameProofsByFid?fid={}", hubble_url, fid);
            fetch_and_respond(url).await
        }
        
        pub async fn get_user_data_by_fid(Query(params): Query<UserDataParams>) -> Result<Json<Value>, StatusCode> {
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut url = format!("{}/userDataByFid?fid={}", hubble_url, params.fid);
            if let Some(ref data_type) = params.user_data_type {
                url.push_str(&format!("&user_data_type={}", data_type));
            }

            info!("Final user data URL: {}", url);
            fetch_and_respond(url).await
        }
        
        pub async fn get_cast_by_id(Path((fid, hash)): Path<(u64, String)>) -> Result<Json<Value>, StatusCode> {
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let url = format!("{}:2281/v1/castById?fid={}&hash={}", hubble_url, fid, hash);
            fetch_and_respond(url).await
        }
        
        pub async fn get_casts_by_fid(Path(fid): Path<u64>) -> Result<Json<Value>, StatusCode> {
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let url = format!("{}:2281/v1/castsByFid?fid={}", hubble_url, fid);
            fetch_and_respond(url).await
        }
        
        pub async fn get_channels() -> Result<Json<Value>, StatusCode> {
            let warpcast_url = env::var("WARPCAST_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let url = format!("{}all-channels", warpcast_url);
            fetch_and_respond(url).await
        }
        
        pub async fn get_casts_by_parent(
            Path(encoded_url): Path<String>,
            Query(query): Query<HashMap<String, u64>>,
        ) -> Result<Json<Value>, StatusCode> {
            info!("Fetching Casts by Channel");
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let page = query.get("page").cloned().unwrap_or(1);
            let limit = query.get("limit").cloned().unwrap_or(40);
            let _offset = (page - 1) * limit;

            let url = format!(
                "{}/castsByChannel/{}",
                hubble_url, encoded_url 
            );

            info!("Final URL: {}", url);
        
            fetch_and_respond(url).await
        }
        
        pub async fn get_casts_by_mention(Path(fid): Path<u64>) -> Result<Json<Value>, StatusCode> {
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let url = format!("{}:2281/v1/castsByMention?fid={}", hubble_url, fid);
            fetch_and_respond(url).await
        }
        
        pub async fn get_reactions_by_cast(Query(params): Query<ReactionsByCastParams>) -> Result<Json<Value>, StatusCode> {
            let hubble_url = env::var("HUBBLE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut url = format!("{}:2281/v1/reactionsByCast?target_fid={}&target_hash={}",
                                  hubble_url, params.target_fid, params.target_hash);
        
            if let Some(ref reaction_type) = params.reaction_type {
                url.push_str(&format!("&reaction_type={}", reaction_type));
            }
        
            fetch_and_respond(url).await
        }
        
        async fn fetch_and_respond(url: String) -> Result<Json<Value>, StatusCode> {
            let client = Client::new();
            match client.get(&url).send().await {
                Ok(response) if response.status().is_success() => {
                    match response.json::<Value>().await {
                        Ok(json) => Ok(Json(json)),
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
}}
