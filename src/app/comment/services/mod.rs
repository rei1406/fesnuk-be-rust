use crate::app::comment::{
    dto::{CommentResponse, CreateCommentDto, ReplyCommentDto},
    repositories::CommentRepository,
    models::{NewComment},
};
use sqlx::{PgPool, Error};
use serde_json::{Value};

pub struct CommentService;

impl CommentService {
    pub async fn get_all_comments(pool: &PgPool) -> Result<Vec<CommentResponse>, Error> {
        let comments = CommentRepository::find_all(pool).await?;
        Ok(comments.into_iter().map(CommentResponse::from).collect())
    }

    pub async fn get_comments_by_post_id(
        pool: &PgPool,
        post_id: i32,
    ) -> Result<Vec<CommentResponse>, Error> {
        let comments = CommentRepository::find_by_post_id(pool, post_id).await?;
        Ok(comments.into_iter().map(CommentResponse::from).collect())
    }

    pub async fn get_replies_by_comment_id(
        pool: &PgPool,
        parent_id: i32,
    ) -> Result<Vec<CommentResponse>, Error> {
        let comments = CommentRepository::find_replies(pool, parent_id).await?;
        Ok(comments.into_iter().map(CommentResponse::from).collect())
    }

    pub async fn get_comment_by_id(
        pool: &PgPool,
        comment_id: i32,
    ) -> Result<CommentResponse, Error> {
        let comment = CommentRepository::find_by_id(pool, comment_id).await?;
        Ok(CommentResponse::from(comment))
    }

    pub async fn create_comment(
        pool: &PgPool,
        dto: CreateCommentDto,
    ) -> Result<CommentResponse, Error> {
        let new_comment = NewComment {
            post_id: dto.post_id,
            parent_id: None,
            content: dto.content,
            attachments: dto.attachments.unwrap_or(Value::Array(vec![])),
        };
        
        let comment = CommentRepository::create(pool, new_comment).await?;
        Ok(CommentResponse::from(comment))
    }

    pub async fn reply_to_comment(
        pool: &PgPool,
        dto: ReplyCommentDto,
    ) -> Result<CommentResponse, Error> {
        let new_comment = NewComment {
            post_id: dto.post_id,
            parent_id: Some(dto.parent_id),
            content: dto.content,
            attachments: dto.attachments.unwrap_or(Value::Array(vec![])),
        };
        
        let comment = CommentRepository::create(pool, new_comment).await?;
        Ok(CommentResponse::from(comment))
    }

    pub async fn delete_comment(
        pool: &PgPool,
        comment_id: i32,
    ) -> Result<CommentResponse, Error> {
        let comment = CommentRepository::delete(pool, comment_id).await?;
        Ok(CommentResponse::from(comment))
    }
}