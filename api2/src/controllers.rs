use crate::{
    database::establish_connection,
    models::{NewRecord, Record},
};
use actix_web::{get, post, web, HttpResponse, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[get("/record")]
pub async fn records() -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;

    let mut connection = establish_connection();

    let results = records
        .load::<Record>(&mut connection)
        .expect("Error on get all records");

    Ok(HttpResponse::Ok().json(results))
}

#[get("/record/{idd}")]
pub async fn record(idd: web::Path<String>) -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();
    let generated_id = Uuid::parse_str(idd.as_str()).expect("");

    let result = records
        .filter(id.eq(generated_id))
        .load::<Record>(&mut connection)
        .expect("Error on get record");

    Ok(HttpResponse::Ok().json(result))
}

#[post("/record")]
pub async fn new_record(new_record: web::Json<NewRecord>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("resutl"))
}
