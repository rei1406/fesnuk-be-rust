use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub attachments: Value,
    pub reactions: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub reply_count: Option<i64>,
}

pub struct NewComment {
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub attachments: Value,
}

pub struct CommentReaction {
    pub comment_id: i32,
    // Unicode of the reaction emoji - e.g. "U+1F44D" for "thumbs up" (👍)
    pub unicode: String,
    // 1 for up, -1 for down
    pub action: i8,
}