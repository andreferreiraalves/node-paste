use crate::routes::message::*;
use actix_web::{App, HttpServer};

mod database;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(message).service(create_message)
        // .service(record)
        // .service(create_record)
        // .service()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
