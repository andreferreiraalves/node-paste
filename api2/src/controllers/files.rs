use std::path::PathBuf;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, Error, HttpResponse, Result};
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

use crate::schema::UploadForm;

#[post("/upload")]
async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> Result<HttpResponse, Error> {
    for f in form.files {
        let path = format!("../temp/{}", f.file_name.unwrap());
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": "success"
    })))
}

#[get("/download")]
async fn download() -> Result<HttpResponse> {
    let file_path: PathBuf = "./temp/arquivo.txt".into();

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
