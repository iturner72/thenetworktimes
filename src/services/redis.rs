use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

use redis::aio::Connection;
use std::pin::Pin;
use futures::io::AsyncRead;

use crate::models::farcaster::UserDataResponse;

pub async fn get_user_data_from_cache(
    redis_conn: &mut Connection<Pin<Box<dyn AsyncRead + Send + Sync>>>, 
    fid: u64,
) -> Result<Option<UserDataResponse>, redis::RedisError> {
    let key = format!("user_data:{}", fid);
    let cached_data: Option<String> = redis_conn.get(&key).await?;
    Ok(cached_data.and_then(|data| serde_json::from_str(&data).ok()))
}

pub async fn set_user_data_to_cache(
    redis_conn: &mut Connection<Pin<Box<dyn AsyncRead + Send + Sync>>>, 
    fid: u64,
    user_data: &UserDataResponse,
) -> Result<(), redis::RedisError> {
    let key = format!("user_data:{}", fid);
    let serialized_data = serde_json::to_string(user_data)
        .map_err(e| redis::RedisError::from((redis::ErrorKind::IoError, "serialization error", e.to_string())))?;
    redis::cmd("SETEX")
        .arg(&key)
        .arg(3600) // cache for 1 hour
        .arg(serialized_data)
        .query_async(redis_conn)
        .await
}

}}
