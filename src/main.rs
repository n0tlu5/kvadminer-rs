use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use redis::Commands;
use std::env;

use dotenv::dotenv;


#[derive(serde::Deserialize)]
struct SetKeyRequest {
    key: String,
    value: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the KVAdminer-RS Web App")
}

async fn get_key(data: web::Data<redis::Client>, key: web::Path<String>) -> impl Responder {
    let mut con = data.get_connection().unwrap();
    let value: Result<String, _> = con.get(&*key);
    match value {

        Ok(val) => HttpResponse::Ok().body(val),

        Err(_) => HttpResponse::NotFound().body("Key not found"),
    }
}

async fn set_key(data: web::Data<redis::Client>, item: web::Json<SetKeyRequest>) -> impl Responder {
    let mut con = data.get_connection().unwrap();
    let result: Result<(), _> = con.set(&item.key, &item.value);
    match result {
        Ok(_) => HttpResponse::Ok().body("Key set successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to set key"),
    }
}

async fn list_keys(data: web::Data<redis::Client>) -> impl Responder {
    let mut con = data.get_connection().unwrap();

    let keys: Result<Vec<String>, _> = con.keys("*");
    match keys {
        Ok(keys) => HttpResponse::Ok().json(keys),
        Err(_) => HttpResponse::InternalServerError().body("Failed to list keys"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let redis_host = env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or("6379".to_string());

    let redis_user = env::var("REDIS_USER").unwrap_or("".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or("".to_string());

    let redis_url = if redis_user.is_empty() {
        format!("redis://{}:{}/", redis_host, redis_port)
    } else {
        format!("redis://{}:{}@{}:{}/", redis_user, redis_password, redis_host, redis_port)
    };

    let client = redis::Client::open(redis_url).expect("Invalid Redis URL");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(index))
            .route("/get/{key}", web::get().to(get_key))
            .route("/set", web::post().to(set_key))

            .route("/keys", web::get().to(list_keys))
    })
    .bind("0.0.0.0:8080")?
    .run()

    .await
}
