use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::{PgConnection, PgPool};

mod handler;
mod model;
mod schema;

pub struct AppState {
    db: PgPool,
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "App is running";
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    println!("🚀 Server started successfully 8080");

    HttpServer::new(|| {
        App::new().service(health_checker_handler)
        // .service(record)
        // .service(create_record)
        // .service()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
