use std::path::PathBuf;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{model::RecordModel, schema::UploadForm, AppState};

#[post("/upload")]
async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    let new_guid = uuid::Uuid::new_v4();

    for f in form.files {
        let file_name = f.file_name.unwrap();
        let path = format!("./temp/{}", new_guid.to_string());

        let query_result = sqlx::query!(
            r"insert into records (ID, FILE_NAME, file_path) values ($1, $2, $3)",
            &new_guid,
            &file_name,
            "./temp/"
        )
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

        if let Err(err) = query_result {
            return HttpResponse::InternalServerError().json(serde_json::json!({"error": err}));
        }

        let file_result = f.file.persist(path);

        if let Err(e) = file_result {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "success",
        "guid": new_guid.to_string(),
    }))
}

#[get("/download/{guid}")]
async fn download(guid: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let id = guid.into_inner();

    let file_record_result = sqlx::query_as!(
        RecordModel,
        r"select * from records where id = $1 and FILE_NAME is not null",
        id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    let result = match file_record_result {
        Ok(record) => record,
        Err(err) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"message": err}));
        }
    };

    let file_path: PathBuf = format!("./temp/{}", result.id.to_string()).into();

    let file_result = File::open(&file_path)
        .await
        .map_err(actix_web::error::ErrorInternalServerError);

    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"message":"file not found"}));
        }
    };

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    let file_name = format!("attachment; filename={}", result.file_name.unwrap());

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header(("Content-Disposition", file_name))
        .body(buffer)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/files").service(upload).service(download);

    conf.service(scope);
}
