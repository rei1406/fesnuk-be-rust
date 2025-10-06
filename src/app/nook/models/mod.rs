use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Nook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

pub struct NewNook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}

pub struct NookChanges {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}
