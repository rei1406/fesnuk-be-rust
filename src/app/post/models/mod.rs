use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub attachments: Value,
    pub reactions: Value,
    pub nook_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub attachments: Value,
    pub nook_id: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostChanges {
    pub title: Option<String>,
    pub content: Option<String>,
    pub attachments: Option<Value>,
}

pub struct Reaction {
    pub post_id: i32,
    // Unicode of the reaction emoji - e.g. "U+1F44D" for "thumbs up" (👍)
    pub unicode: String,
    // 1 for up, -1 for down
    pub action: i8,
}
