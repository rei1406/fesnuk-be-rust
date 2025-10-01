use super::models::{NewNook, Nook, NookChanges};
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateNookDto {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateNookDto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl CreateNookDto {
    pub fn to_new_nook(self) -> NewNook {
        NewNook {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

impl UpdateNookDto {
    pub fn to_nook_changes(self) -> NookChanges {
        NookChanges {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Serialize)]
pub struct NookResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Nook> for NookResponse {
    fn from(nook: Nook) -> Self {
        Self {
            id: nook.id,
            name: nook.name,
            description: nook.description,
            created_at: nook.created_at,
            updated_at: nook.updated_at,
        }
    }
}
