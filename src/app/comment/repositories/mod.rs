use super::models::{Comment, NewComment};
use sqlx::PgPool;

pub struct CommentRepository;

impl CommentRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Comment>, sqlx::Error> {
        sqlx::query_as!(
            Comment,
            "SELECT id, post_id, parent_id, content, attachments, reactions, created_at, updated_at, deleted_at 
             FROM comments 
             WHERE deleted_at IS NULL"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, comment_id: i32) -> Result<Comment, sqlx::Error> {
        sqlx::query_as!(
            Comment,
            "SELECT id, post_id, parent_id, content, attachments, reactions, created_at, updated_at, deleted_at 
             FROM comments 
             WHERE id = $1 AND deleted_at IS NULL",
            comment_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_post_id(pool: &PgPool, post_id: i32) -> Result<Vec<Comment>, sqlx::Error> {
        sqlx::query_as!(
            Comment,
            "SELECT id, post_id, parent_id, content, attachments, reactions, created_at, updated_at, deleted_at 
             FROM comments 
             WHERE post_id = $1 AND deleted_at IS NULL",
            post_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(pool: &PgPool, new_comment: NewComment) -> Result<Comment, sqlx::Error> {
        sqlx::query_as!(
            Comment,
            "INSERT INTO comments (post_id, parent_id, content, attachments) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, post_id, parent_id, content, attachments, reactions, created_at, updated_at, deleted_at",
            new_comment.post_id,
            new_comment.parent_id,
            new_comment.content,
            new_comment.attachments
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, comment_id: i32) -> Result<Comment, sqlx::Error> {
        sqlx::query_as!(
            Comment,
            "UPDATE comments 
             SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP 
             WHERE id = $1 AND deleted_at IS NULL 
             RETURNING id, post_id, parent_id, content, attachments, reactions, created_at, updated_at, deleted_at",
            comment_id
        )
        .fetch_one(pool)
        .await
    }
}