use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct RecordModel {
    pub id: Uuid,
    pub message: Option<String>,
    pub file_name: Option<String>,
}
