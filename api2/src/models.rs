use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Record {
    pub id: Uuid,
    pub content: Option<String>,
    pub file_name: Option<String>,
}
