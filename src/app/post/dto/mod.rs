use crate::app::post::models::Reaction;

use super::models::{NewPost, Post, PostChanges};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct CreatePostDto {
    pub title: String,
    pub content: String,
    pub attachments: Option<Value>,
    pub nook_id: String,
}

#[derive(Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub attachments: Option<Value>,
}

impl CreatePostDto {
    pub fn to_new_post(self) -> NewPost {
        NewPost {
            title: self.title,
            content: self.content,
            attachments: self.attachments.unwrap_or(serde_json::json!([])),
            nook_id: self.nook_id,
        }
    }
}

impl UpdatePostDto {
    pub fn to_post_changes(self) -> PostChanges {
        PostChanges {
            title: self.title,
            content: self.content,
            attachments: self.attachments,
        }
    }
}

#[derive(Deserialize)]
pub struct ReactPostDto {
    pub post_id: i32,
    pub unicode: String,
    pub action: i8,
}

impl ReactPostDto {
    pub fn to_reaction(self) -> Reaction {
        Reaction {
            post_id: self.post_id,
            unicode: self.unicode,
            action: self.action,
        }
    }
}



#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub attachments: Value,
    pub reactions: Value,
    pub nook_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            attachments: post.attachments,
            reactions: post.reactions,
            nook_id: post.nook_id,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}