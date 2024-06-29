use crate::errors::KVAdminerError;
use serde::{Deserialize, Serialize};
use log::info;
use redis::{Commands};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct RedisInfo {
    pub host: String,
    pub port: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub session_id: Option<String>, // Optional session identifier
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RedisValueType {
    String,
    List,
    Set,
    ZSet,
    Hash,
    None,
    Unknown,
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

pub fn get_redis_value(con: &mut redis::Connection, key: &str) -> Result<(String, RedisValueType), KVAdminerError> {
    let type_cmd: redis::RedisResult<String> = redis::cmd("TYPE").arg(key).query(con);
    match type_cmd {
        Ok(data_type) => match data_type.as_str() {
            "string" => {
                let value: redis::RedisResult<String> = con.get(key);
                value.map(|v| (v, RedisValueType::String))
                    .map_err(|err| KVAdminerError::RedisError(err.to_string()))
            },
            "list" => {
                let value: redis::RedisResult<Vec<String>> = con.lrange(key, 0, -1);
                value.map(|v| (v.join(", "), RedisValueType::List))
                    .map_err(|err| KVAdminerError::RedisError(err.to_string()))
            },
            "set" => {
                let value: redis::RedisResult<Vec<String>> = con.smembers(key);
                value.map(|v| (v.join(", "), RedisValueType::Set))
                    .map_err(|err| KVAdminerError::RedisError(err.to_string()))
            },
            "zset" => {
                let value: redis::RedisResult<Vec<String>> = con.zrange(key, 0, -1);
                value.map(|v| (v.join(", "), RedisValueType::ZSet))
                    .map_err(|err| KVAdminerError::RedisError(err.to_string()))
            },
            "hash" => {
                let value: redis::RedisResult<Vec<(String, String)>> = con.hgetall(key);
                value.map(|v| {
                    let hash: Vec<String> = v.into_iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                    (hash.join(", "), RedisValueType::Hash)
                }).map_err(|err| KVAdminerError::RedisError(err.to_string()))
            },
            _ => Ok((String::new(), RedisValueType::Unknown)),
        },
        Err(err) => {
            log::error!("Redis Error: {}", err);
            Err(KVAdminerError::RedisError(err.to_string()))
        }
    }
}

pub fn set_redis_value(con: &mut redis::Connection, key: &str, value: &str, value_type: &RedisValueType) -> Result<(), KVAdminerError> {
    match value_type {
        RedisValueType::String => {
            let result: redis::RedisResult<()> = con.set(key, value);
            result.map_err(|err| KVAdminerError::RedisError(err.to_string()))
        },
        RedisValueType::List => {
            let values: Vec<&str> = value.split(',').collect();
            con.del(key as &str).map_err(|err| KVAdminerError::RedisError(err.to_string()))?;
            let result: redis::RedisResult<()> = con.rpush(key, values);
            result.map_err(|err| KVAdminerError::RedisError(err.to_string()))
        },
        RedisValueType::Set => {
            let values: Vec<&str> = value.split(',').collect();
            con.del(key as &str).map_err(|err| KVAdminerError::RedisError(err.to_string()))?;
            let result: redis::RedisResult<()> = con.sadd(key, values);
            result.map_err(|err| KVAdminerError::RedisError(err.to_string()))
        },
        RedisValueType::ZSet => {
            let values: Vec<&str> = value.split(',').collect();
            con.del(key as &str).map_err(|err| KVAdminerError::RedisError(err.to_string()))?;
            let score_value_pairs: Vec<(i64, &str)> = values.iter().enumerate().map(|(i, &v)| (i as i64, v)).collect();
            let result: redis::RedisResult<()> = con.zadd_multiple(key, &score_value_pairs);
            result.map_err(|err| KVAdminerError::RedisError(err.to_string()))
        },
        RedisValueType::Hash => {
            con.del(key as &str).map_err(|err| KVAdminerError::RedisError(err.to_string()))?;
            let kv_pairs: Vec<(&str, &str)> = value.split(',').map(|pair| {
                let mut split = pair.split(':');
                let k = split.next().unwrap().trim();
                let v = split.next().unwrap_or("").trim();
                (k, v)
            }).collect();
            let result: redis::RedisResult<()> = con.hset_multiple(key, &kv_pairs);
            result.map_err(|err| KVAdminerError::RedisError(err.to_string()))
        },
        _ => Err(KVAdminerError::TypeError)
    }
}

pub fn get_redis_hash(con: &mut redis::Connection, key: &str) -> Result<HashMap<String, String>, KVAdminerError> {
    let value: redis::RedisResult<HashMap<String, String>> = con.hgetall(key);
    value.map_err(|err| KVAdminerError::RedisError(err.to_string()))
}

pub fn set_redis_hash(con: &mut redis::Connection, key: &str, field: &str, value: &str) -> Result<(), KVAdminerError> {
    let result: redis::RedisResult<()> = con.hset(key, field, value);
    result.map_err(|err| KVAdminerError::RedisError(err.to_string()))
}
