use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

use crate::models::farcaster::UserDataResponse;

pub async fn get_user_data_from_cache(
    redis_conn: &mut MultiplexedConnection, 
    cache_key: &str,
) -> Result<Option<UserDataResponse>, redis::RedisError> {
    let cached_data: Option<String> = redis_conn.get(cache_key).await?;
    Ok(cached_data.and_then(|data| serde_json::from_str(&data).ok()))
}

pub async fn set_user_data_to_cache(
    redis_conn: &mut MultiplexedConnection, 
    cache_key: &str,
    user_data: &UserDataResponse,
) -> Result<(), redis::RedisError> {
    let serialized_data = serde_json::to_string(user_data)
        .map_err(|e| redis::RedisError::from((redis::ErrorKind::IoError, "serialization error", e.to_string())))?;
    redis_conn.set_ex(cache_key, serialized_data, 3600).await // cache for 1 hour
}

}}
