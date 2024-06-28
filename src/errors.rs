use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum KVAdminerError {
    RedisError(String),
    InvalidRedisUrl,
}

impl fmt::Display for KVAdminerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KVAdminerError::RedisError(err) => write!(f, "Redis Error: {}", err),
            KVAdminerError::InvalidRedisUrl => write!(f, "Invalid Redis URL"),
        }
    }
}

impl ResponseError for KVAdminerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            KVAdminerError::RedisError(err) => {
                HttpResponse::InternalServerError().body(err.clone())
            }
            KVAdminerError::InvalidRedisUrl => {
                HttpResponse::InternalServerError().body("Invalid Redis URL")
            }
        }
    }
}

impl From<redis::RedisError> for KVAdminerError {
    fn from(err: redis::RedisError) -> KVAdminerError {
        KVAdminerError::RedisError(err.to_string())
    }
}

