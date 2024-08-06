use std::path::PathBuf;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, Error, HttpResponse, Result};
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{schema::UploadForm, AppState};

#[post("/upload")]
async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
    // ) -> Result<HttpResponse, Error> {
) -> HttpResponse {
    let new_guid = uuid::Uuid::new_v4();

    for f in form.files {
        let file_name = f.file_name.unwrap();
        let path = format!("./temp/{}", new_guid.to_string());

        let query_result = sqlx::query!(
            r"insert into records (ID, FILE_NAME, file_path) values ($1, $2, $3)",
            new_guid,
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
        "status": "success"
    }))
}

#[get("/download/{filename}")]
async fn download(filename: web::Path<String>) -> Result<HttpResponse> {
    let file_path: PathBuf = format!("./temp/{}", filename).into();

    let mut file = File::open(&file_path)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header(("Content-Disposition", "attachment; filename=file.txt"))
        .body(buffer))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/files").service(upload).service(download);

    conf.service(scope);
}
