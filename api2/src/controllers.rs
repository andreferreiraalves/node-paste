use crate::{database::establish_connection, models::Record};
use actix_web::{get, HttpResponse, Responder, Result};
use diesel::RunQueryDsl;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[get("/records")]
pub async fn records() -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;

    let mut connection = establish_connection();

    let results = records
        .load::<Record>(&mut connection)
        .expect("Error on get all records");

    Ok(HttpResponse::Ok().json(results))
}
