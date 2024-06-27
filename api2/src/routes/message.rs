use crate::{
    database::establish_connection,
    models::{NewRecord, Record},
};
use actix_web::{get, post, web, HttpResponse, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[post("/message")]
pub async fn create_message(new_record: web::Json<NewRecord>) -> Result<HttpResponse> {
    use crate::schema::records::dsl::*;
    let json = new_record.into_inner();

    let mut connection = establish_connection();

    let record_database: Record = diesel::insert_into(records)
        .values(json)
        .get_result(&mut connection)
        .unwrap();

    Ok(HttpResponse::Ok().json(record_database))
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
