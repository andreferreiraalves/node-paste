use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    model::RecordModel,
    schema::{CreateRecordSchema, FilterOptions},
    AppState,
};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "App is running";
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/records")]
async fn record_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let records: Vec<RecordModel> = sqlx::query_as!(
        RecordModel,
        r#"select * from records order by id limit $1 offset $2"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    // let note_responses = notes
    //     .into_iter()
    //     .map(|note| filter_db_record(&note))
    //     .collect::<Vec<NoteModelResponse>>();
    //
    let json_response = serde_json::json!({
         "status": "success",
         "results":records.len(),
         "records": records,
    });

    HttpResponse::Ok().json(json_response)
}

#[get("/records/{id}")]
async fn get_record_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let record_id = path.into_inner();

    let query_result = sqlx::query_as!(
        RecordModel,
        r#"select * from records where id = $1"#,
        record_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(record) => {
            let record_response = serde_json::json!({
                "status": "success",
                "record": record,
            });

            return HttpResponse::Ok().json(record_response);
        }
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "fail",
                "message": format!("record with id: {} not found", record_id)
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

#[post("/records")]
async fn create_record_handler(
    body: web::Json<CreateRecordSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let record_id = uuid::Uuid::new_v4();

    let query_result =
        sqlx::query(r#"insert into records (id, message, file_name) values ($1, $2, $3)"#)
            .bind(record_id.clone())
            .bind(body.message.to_owned().unwrap_or_default())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "fail",
                    "message": "Record with that title already exists",
                }
            ));
        }

        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success" : "error",
            "message": format!("{:?}", err),
        }));
    }

    let query_result = sqlx::query_as!(
        RecordModel,
        r#"select * from records where id = $1"#,
        record_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(record) => {
            let record_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({"record": record})
            });

            return HttpResponse::Ok().json(record_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status" : "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

#[patch("/records/{id}")]
async fn edit_record_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<CreateRecordSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let record_id = path.into_inner();
    let query_result = sqlx::query_as!(
        RecordModel,
        r#"select * from records where id = $1"#,
        record_id
    )
    .fetch_one(&data.db)
    .await;

    let record = match query_result {
        Ok(record) => record,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "fail",
                "message": format!("Record with ID: {} not found", record_id)
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    };

    let update_result =
        sqlx::query(r#"update records set message = $1, file_name = $2 where id = $3"#)
            .bind(
                body.message
                    .to_owned()
                    .unwrap_or_else(|| record.message.clone().unwrap()),
            )
            .bind(
                body.file_name
                    .to_owned()
                    .unwrap_or_else(|| record.file_name.clone().unwrap()),
            )
            .bind(record_id.to_owned())
            .execute(&data.db)
            .await;

    match update_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Record with ID: {} not found", record_id);
                return HttpResponse::NotFound().json(serde_json::json!({
                    "status": "fail",
                    "message": message,
                }));
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "error",
                "message": message,
            }));
        }
    }

    let updated_record_result = sqlx::query_as!(
        RecordModel,
        r#"select * from records where id = $1"#,
        record_id.to_owned()
    )
    .fetch_one(&data.db)
    .await;

    match updated_record_result {
        Ok(record) => {
            let record_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "record": record,
                })
            });

            HttpResponse::Ok().json(record_response)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message" : format!("{:?}", e)
        })),
    }
}

#[delete("/records/{id}")]
async fn delete_record_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let record_id = path.into_inner();

    let query_result = sqlx::query!(r#"delete from records where id = $1"#, record_id)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Record with id: {} not found", record_id);
                HttpResponse::NotFound().json(json!({"status": "fail",
                "message": message
                }))
            } else {
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => {
            let message = format!("Internal server erro: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": message,
            }))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(record_list_handler)
        .service(get_record_handler)
        .service(create_record_handler)
        .service(edit_record_handler)
        .service(delete_record_handler);

    conf.service(scope);
}
