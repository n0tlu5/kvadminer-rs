use crate::errors::KVAdminerError;
use serde::Deserialize;
use log::info;

#[derive(Deserialize)]
pub struct RedisInfo {
    pub host: String,
    pub port: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub session_id: Option<String>, // Optional session identifier
}

pub fn create_redis_client(info: &RedisInfo) -> Result<redis::Client, KVAdminerError> {
    let redis_url = if let Some(username) = &info.username {
        if let Some(password) = &info.password {
            format!(
                "redis://{}:{}@{}:{}/",
                username, password, info.host, info.port
            )
        } else {
            format!("redis://{}@{}:{}/", username, info.host, info.port)
        }
    } else {
        format!("redis://{}:{}/", info.host, info.port)
    };
    info!("Creating Redis client for URL: {}", redis_url);
    redis::Client::open(redis_url).map_err(|_| KVAdminerError::InvalidRedisUrl)
}
