use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use log::info;
use env_logger::Env;

mod errors;
mod redis_ops;
mod handlers;
mod session;

use handlers::*;
use session::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Starting the server...");

    let app_state = Arc::new(AppState {
        connections: Arc::new(Mutex::new(HashMap::new())),
        session_timeout: Duration::from_secs(3600), // 1 hour timeout
    });

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60)); // Cleanup interval
        loop {
            interval.tick().await;
            session::cleanup_expired_sessions(app_state_clone.clone()).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .route("/", web::get().to(|| serve_html("./static/index.html")))
            .route(
                "/keys-management",
                web::get().to(|| serve_html("./static/db-main.html")),
            )
            .route(
                "/edit-key",
                web::get().to(|| serve_html("./static/key-edit.html")),
            )
            .route("/get/{key}", web::get().to(get_key))
            .route("/set", web::post().to(set_key))
            .route("/delete/{key}", web::delete().to(delete_key))
            .route("/keys", web::get().to(list_keys))
            .route("/get-hash/{key}", web::get().to(get_hash))
            .route("/set-hash", web::post().to(set_hash))
            .service(actix_files::Files::new("/public", "./static/public"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
