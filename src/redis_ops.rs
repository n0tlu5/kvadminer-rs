use crate::errors::KVAdminerError;
use serde::Deserialize;
use log::info;
use redis::Commands;

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

pub fn get_string_value(con: &mut redis::Connection, key: &str) -> Result<String, KVAdminerError> {
    let value: redis::RedisResult<String> = con.get(key);
    match value {
        Ok(val) => Ok(val),
        Err(err) => {
            match err.kind() {
                redis::ErrorKind::TypeError => {
                    log::error!("Redis TypeError: {}", err);
                    Err(KVAdminerError::TypeError)
                },
                _ => {
                    log::error!("Redis Error: {}", err);
                    Err(KVAdminerError::RedisError(err.to_string()))
                }
            }
        }
    }
}
