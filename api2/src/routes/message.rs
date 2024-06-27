use crate::{
    database::establish_connection,
    models::{NewRecord, Record},
    schema::records,
};
use actix_web::{get, post, web, HttpResponse, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[post("/message")]
pub async fn create_message() -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();
}

#[get("/message/{guid}")]
pub async fn message(guid: web::Path<String>) -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;

    let mut connection = establish_connection();
    let id_path = Uuid::parse_str(guid.as_str()).expect("Guid passed invalid");

    let record_database = records
        .filter(id.eq(id_path))
        .load::<Record>(&mut connection)
        .expect("Error on get all records");

    Ok(HttpResponse::Ok().json(record_database))
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
pub async fn create_record(new_record: web::Json<NewRecord>) -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();

    let record_saved = diesel::insert_into(records)
        .values(&new_record.into_inner())
        .execute(&mut connection)
        .expect("Error on create a record");

    println!("{:?}", record_saved);

    Ok(HttpResponse::Ok().json("resutl"))
}
