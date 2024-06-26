use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use log::error;

#[derive(Debug)]
pub enum KVAdminerError {
    RedisError(String),
    InvalidRedisUrl,
    TypeError,
}

impl fmt::Display for KVAdminerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KVAdminerError::RedisError(err) => write!(f, "Redis Error: {}", err),
            KVAdminerError::InvalidRedisUrl => write!(f, "Invalid Redis URL"),
            KVAdminerError::TypeError => write!(f, "Type conversion error occurred"),
        }
    }
}

impl ResponseError for KVAdminerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            KVAdminerError::RedisError(err) => {
                error!("Redis Error: {}", err);
                HttpResponse::InternalServerError().body(err.clone())
            }
            KVAdminerError::InvalidRedisUrl => {
                error!("Invalid Redis URL");
                HttpResponse::InternalServerError().body("Invalid Redis URL")
            }
            KVAdminerError::TypeError => {
                error!("Type conversion error occurred");
                HttpResponse::InternalServerError().body("Type conversion error occurred")
            }
        }
    }
}

impl From<redis::RedisError> for KVAdminerError {
    fn from(err: redis::RedisError) -> KVAdminerError {
        error!("Redis Error: {}", err);
        KVAdminerError::RedisError(err.to_string())
    }
}
