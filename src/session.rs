use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

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
    Uuid::new_v4().to_string()
}

