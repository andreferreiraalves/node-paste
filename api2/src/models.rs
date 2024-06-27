use crate::schema::records;
use diesel::prelude::*;
use serde::*;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Record {
    pub id: Uuid,
    pub message: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "records"]
pub struct NewRecord {
    pub message: String,
}
