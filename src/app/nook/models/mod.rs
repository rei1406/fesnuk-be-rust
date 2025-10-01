use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::nooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::nooks)]
pub struct NewNook {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::nooks)]
pub struct NookChanges {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}
