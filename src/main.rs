use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, ResponseError};
use redis::Commands;
use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
enum KVAdminerError {
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

#[derive(Deserialize)]
struct RedisInfo {
    host: String,
    port: String,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize)]
struct SetKeyRequest {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct PaginationParams {
    page: usize,
    page_size: usize,
}

#[derive(serde::Serialize)]
struct PaginatedKeys {
    keys: Vec<(String, String)>,
    current_page: usize,
    total_pages: usize,
    total_keys: usize,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the KVAdminer-RS Web App")
}

async fn get_key(
    info: web::Query<RedisInfo>,
    key: web::Path<String>,
) -> Result<HttpResponse, KVAdminerError> {
    let client = create_redis_client(&info)?;
    let mut con = client.get_connection()?;
    let value: Result<String, _> = con.get(&*key);
    match value {
        Ok(val) => Ok(HttpResponse::Ok().body(val)),
        Err(_) => Ok(HttpResponse::NotFound().body("Key not found")),
    }
}

async fn set_key(
    info: web::Query<RedisInfo>,
    item: web::Json<SetKeyRequest>,
) -> Result<HttpResponse, KVAdminerError> {
    let client = create_redis_client(&info)?;
    let mut con = client.get_connection()?;
    let result: Result<(), _> = con.set(&item.key, &item.value);
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Key set successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to set key")),
    }
}

async fn delete_key(
    info: web::Query<RedisInfo>,
    key: web::Path<String>,
) -> Result<HttpResponse, KVAdminerError> {
    let client = create_redis_client(&info)?;
    let mut con = client.get_connection()?;
    let result: Result<(), _> = con.del(&*key);
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Key deleted successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to delete key")),
    }
}

async fn list_keys(
    info: web::Query<RedisInfo>,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, KVAdminerError> {
    let client = create_redis_client(&info)?;
    let mut con = client.get_connection()?;

    let all_keys: Vec<String> = con.keys("*")?;
    let total_keys = all_keys.len();
    let total_pages = (total_keys + params.page_size - 1) / params.page_size;

    let start_index = params.page * params.page_size;
    let end_index = std::cmp::min(start_index + params.page_size, total_keys);

    let paginated_keys: Vec<(String, String)> = all_keys[start_index..end_index]
        .iter()
        .map(|key| {
            let value: String = con.get(key).unwrap_or_else(|_| "N/A".to_string());
            (key.clone(), value)
        })
        .collect();

    Ok(HttpResponse::Ok().json(PaginatedKeys {
        keys: paginated_keys,
        current_page: params.page,
        total_pages,
        total_keys,
    }))
}

fn create_redis_client(info: &RedisInfo) -> Result<redis::Client, KVAdminerError> {
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
    redis::Client::open(redis_url).map_err(|_| KVAdminerError::InvalidRedisUrl)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get/{key}", web::get().to(get_key))
            .route("/set", web::post().to(set_key))
            .route("/delete/{key}", web::delete().to(delete_key))
            .route("/keys", web::get().to(list_keys))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
