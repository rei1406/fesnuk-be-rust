use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
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

#[derive(FromRow, Serialize, Deserialize)]
pub struct PostDetail {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub attachments: Value,
    pub reactions: Value,
    pub nook_id: String,
    pub nook_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub comment_count: Option<i64>,
}

pub struct NewPost {
    pub title: String,
    pub content: String,
    pub attachments: Value,
    pub nook_id: String,
}

pub struct PostChanges {
    pub title: Option<String>,
    pub content: Option<String>,
    pub attachments: Option<Value>,
    pub nook_id: Option<String>,
}

pub enum Reaction {
    Like,
    Dislike,
}

pub struct PostReaction {
    pub post_id: i32,
    // Unicode of the reaction emoji - e.g. "U+1F44D" for "thumbs up" (👍)
    pub unicode: String,
    // 1 for up, -1 for down
    pub action: i8,
}
