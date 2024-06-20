use actix_web::{App, HttpServer};
use controllers::*;

mod controllers;
mod database;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(records)
        // .service()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
