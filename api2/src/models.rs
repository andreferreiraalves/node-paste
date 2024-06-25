use crate::schema::records;
use diesel::prelude::*;
use serde::*;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Record {
    pub id: Uuid,
    pub content: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "records"]
pub struct NewRecord<'a> {
    pub content: &'a str,
}
