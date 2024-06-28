use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use actix_web::http::header;
use actix_web::{HttpRequest, HttpMessage};
use log::info;

#[derive(Clone)]
pub struct AppState {
    pub connections: Arc<Mutex<HashMap<String, SessionData>>>,
    pub session_timeout: std::time::Duration,
}

pub struct SessionData {
    pub client: redis::Client,
    pub last_active: std::time::Instant,
}

pub fn generate_session_id() -> String {
    let session_id = Uuid::new_v4().to_string();
    info!("Generated new session ID: {}", session_id);
    session_id
}

pub fn get_or_create_session_id(req: &HttpRequest) -> String {
    if let Some(cookie) = req.cookie("session_id") {
        info!("Found existing session ID: {}", cookie.value());
        cookie.value().to_string()
    } else {
        let session_id = generate_session_id();
        info!("Creating new session ID: {}", session_id);
        session_id
    }
}
