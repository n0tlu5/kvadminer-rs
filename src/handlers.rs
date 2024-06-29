use actix_files::NamedFile;
use actix_web::{web, HttpResponse, HttpRequest, Result};
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use log::{info, error};
use crate::errors::KVAdminerError;
use crate::redis_ops::{RedisInfo, get_redis_value, get_redis_hash, set_redis_value, set_redis_hash, RedisValueType, create_redis_client};
use crate::session::{AppState, SessionData, get_or_create_session_id};

#[derive(Deserialize)]
pub struct SetKeyRequest {
    pub key: String,
    pub value: String,
    pub value_type: RedisValueType,
}

#[derive(Deserialize)]
pub struct SetHashFieldRequest {
    pub key: String,
    pub field: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: usize,
    pub page_size: usize,
    pub search: Option<String>,
}

#[derive(Serialize)]
struct PaginatedKeys {
    keys: Vec<(String, String, RedisValueType)>,
    current_page: usize,
    total_pages: usize,
    total_keys: usize,
}

async fn get_redis_client(
    state: web::Data<AppState>,
    info: &RedisInfo,
) -> Result<redis::Client, KVAdminerError> {
    let session_id = info.session_id.clone().unwrap();
    let mut connections = state.connections.lock().await;
    if let Some(session_data) = connections.get_mut(&session_id) {
        // Update last active time for session timeout
        session_data.last_active = std::time::Instant::now();
        info!("Using existing Redis client for session: {}", session_id);
        Ok(session_data.client.clone())
    } else {
        let client = create_redis_client(info)?;
        connections.insert(session_id.clone(), SessionData {
            client: client.clone(),
            last_active: std::time::Instant::now(),
        });
        info!("Created new Redis client for session: {}", session_id);
        Ok(client)
    }
}

pub async fn get_key(
    state: web::Data<AppState>,
    req: HttpRequest,
    info: web::Query<RedisInfo>,
    key: web::Path<String>,
) -> Result<HttpResponse, KVAdminerError> {
    let session_id = get_or_create_session_id(&req);
    let client_info = RedisInfo { session_id: Some(session_id.clone()), ..info.into_inner() };
    let client = get_redis_client(state, &client_info).await?;
    let mut con = client.get_connection()?;
    match get_redis_value(&mut con, &key).map_err(|e| {
        error!("Error getting key from Redis: {}", e);
        e
    }) {
        Ok((val, val_type)) => {
            info!("Key retrieved successfully: {}", key);
            Ok(HttpResponse::Ok()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .json((val, val_type)))
        },
        Err(err) => Err(err),
    }
}

pub async fn set_key(
    state: web::Data<AppState>,
    req: HttpRequest,
    info: web::Query<RedisInfo>,
    item: web::Json<SetKeyRequest>,
) -> Result<HttpResponse, KVAdminerError> {
    let session_id = get_or_create_session_id(&req);
    let client_info = RedisInfo { session_id: Some(session_id.clone()), ..info.into_inner() };
    let client = get_redis_client(state, &client_info).await?;
    let mut con = client.get_connection()?;

    let result = set_redis_value(&mut con, &item.key, &item.value, &item.value_type);
    match result {
        Ok(_) => {
            info!("Key set successfully: {}", item.key);
            Ok(HttpResponse::Ok()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .body("Key set successfully"))
        },
        Err(_) => {
            error!("Failed to set key: {}", item.key);
            Ok(HttpResponse::InternalServerError()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .body("Failed to set key"))
        },
    }
}

pub async fn get_hash(
    state: web::Data<AppState>,
    req: HttpRequest,
    info: web::Query<RedisInfo>,
    key: web::Path<String>,
) -> Result<HttpResponse, KVAdminerError> {
    let session_id = get_or_create_session_id(&req);
    let client_info = RedisInfo { session_id: Some(session_id.clone()), ..info.into_inner() };
    let client = get_redis_client(state, &client_info).await?;
    let mut con = client.get_connection()?;
    match get_redis_hash(&mut con, &key).map_err(|e| {
        error!("Error getting hash from Redis: {}", e);
        e
    }) {
        Ok(hash) => {
            info!("Hash retrieved successfully: {}", key);
            Ok(HttpResponse::Ok()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .json(hash))
        },
        Err(err) => Err(err),
    }
}

pub async fn set_hash(
    state: web::Data<AppState>,
    req: HttpRequest,
    info: web::Query<RedisInfo>,
    item: web::Json<SetHashFieldRequest>,
) -> Result<HttpResponse, KVAdminerError> {
    let session_id = get_or_create_session_id(&req);
    let client_info = RedisInfo { session_id: Some(session_id.clone()), ..info.into_inner() };
    let client = get_redis_client(state, &client_info).await?;
    let mut con = client.get_connection()?;
    let result = set_redis_hash(&mut con, &item.key, &item.field, &item.value);
    match result {
        Ok(_) => {
            info!("Hash field set successfully: {}:{}", item.key, item.field);
            Ok(HttpResponse::Ok()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .body("Hash field set successfully"))
        },
        Err(_) => {
            error!("Failed to set hash field: {}:{}", item.key, item.field);
            Ok(HttpResponse::InternalServerError()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .body("Failed to set hash field"))
        },
    }
}

pub async fn delete_key(
    state: web::Data<AppState>,
    req: HttpRequest,
    info: web::Query<RedisInfo>,
    key: web::Path<String>,
) -> Result<HttpResponse, KVAdminerError> {
    let session_id = get_or_create_session_id(&req);
    let client_info = RedisInfo { session_id: Some(session_id.clone()), ..info.into_inner() };
    let client = get_redis_client(state, &client_info).await?;
    let mut con = client.get_connection()?;
    let result: Result<(), _> = con.del(&*key);
    match result {
        Ok(_) => {
            info!("Key deleted successfully: {}", key);
            Ok(HttpResponse::Ok()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .body("Key deleted successfully"))
        },
        Err(_) => {
            error!("Failed to delete key: {}", key);
            Ok(HttpResponse::InternalServerError()
                .append_header(("X-Session-ID", session_id.clone()))
                .cookie(
                    actix_web::cookie::Cookie::build("session_id", session_id.clone())
                        .secure(true)
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .finish()
                )
                .body("Failed to delete key"))
        },
    }
}

pub async fn list_keys(
    state: web::Data<AppState>,
    req: HttpRequest,
    info: web::Query<RedisInfo>,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, KVAdminerError> {
    let session_id = get_or_create_session_id(&req);
    let client_info = RedisInfo { session_id: Some(session_id.clone()), ..info.into_inner() };
    let client = get_redis_client(state, &client_info).await?;
    let mut con = client.get_connection()?;

    let pattern = match &params.search {
        Some(query) => format!("*{}*", query),
        None => "*".to_string(),
    };

    let mut keys = vec![];
    let mut cursor = 0;
    loop {
        let (new_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
            .cursor_arg(cursor)
            .arg("MATCH")
            .arg(&pattern)
            .arg("COUNT")
            .arg(1000)
            .query(&mut con)?;

        keys.extend(batch);
        if new_cursor == 0 {
            break;
        }
        cursor = new_cursor;
    }

    let total_keys = keys.len();
    let total_pages = (total_keys + params.page_size - 1) / params.page_size;

    let start_index = params.page * params.page_size;
    let end_index = std::cmp::min(start_index + params.page_size, total_keys);

    let paginated_keys: Vec<(String, String, RedisValueType)> = keys[start_index..end_index]
        .iter()
        .map(|key| {
            let (value, val_type) = get_redis_value(&mut con, key).unwrap_or_else(|_| ("N/A".to_string(), RedisValueType::Unknown));
            (key.clone(), value, val_type)
        })
        .collect();

    info!("Listed keys for session: {}", session_id);
    Ok(HttpResponse::Ok()
        .append_header(("X-Session-ID", session_id.clone()))
        .cookie(
            actix_web::cookie::Cookie::build("session_id", session_id.clone())
                .secure(true)
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish()
        )
        .json(PaginatedKeys {
            keys: paginated_keys,
            current_page: params.page,
            total_pages,
            total_keys,
        }))
}

// Helper function to serve HTML files
pub async fn serve_html(file_path: &str) -> Result<NamedFile> {
    let path: PathBuf = file_path.parse().unwrap();
    Ok(NamedFile::open(path)?)
}
