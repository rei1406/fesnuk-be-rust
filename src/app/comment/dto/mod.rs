use crate::app::comment::models::{CommentReaction, NewComment, Comment};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct CreateCommentDto {
    pub post_id: i32,
    pub content: String,
    pub attachments: Option<Value>,
}

#[derive(Deserialize)]
pub struct ReplyCommentDto {
    pub post_id: i32,
    pub parent_id: i32,
    pub content: String,
    pub attachments: Option<Value>,
}

#[derive(Deserialize)]
pub struct ReactCommentDto {
    pub comment_id: i32,
    pub unicode: String,
    pub action: i8,
}

impl CreateCommentDto {
    pub fn to_new_comment(self) -> NewComment {
        NewComment {
            post_id: self.post_id,
            parent_id: None,
            content: self.content,
            attachments: self.attachments.unwrap_or(serde_json::json!([])),
        }
    }
}

impl ReplyCommentDto {
    pub fn to_new_comment(self) -> NewComment {
        NewComment {
            post_id: self.post_id,
            parent_id: Some(self.parent_id),
            content: self.content,
            attachments: self.attachments.unwrap_or(serde_json::json!([])),
        }
    }
}

impl ReactCommentDto {
    pub fn to_comment_reaction(self) -> CommentReaction {
        CommentReaction {
            comment_id: self.comment_id,
            unicode: self.unicode,
            action: self.action,
        }
    }
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub attachments: Value,
    pub reactions: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub reply_count: i64,
}

impl From<Comment> for CommentResponse {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id,
            post_id: comment.post_id,
            parent_id: comment.parent_id,
            content: comment.content,
            attachments: comment.attachments,
            reactions: comment.reactions,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            reply_count: 0, // Default to 0, can be updated separately if needed
        }
    }
}