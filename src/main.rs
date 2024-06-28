use actix_files::NamedFile;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
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

    let app_state = web::Data::new(AppState {
        connections: Arc::new(Mutex::new(HashMap::new())),
        session_timeout: std::time::Duration::from_secs(3600), // 1 hour timeout
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
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
            .service(actix_files::Files::new("/public", "./static/public"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
