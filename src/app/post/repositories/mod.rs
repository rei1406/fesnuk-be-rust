use super::models::{NewPost, Post, PostChanges};
use sqlx::PgPool;

pub struct PostRepository;

impl PostRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as!(
            Post,
            "SELECT id, title, content, attachments, reactions, nook_id, created_at, updated_at, deleted_at 
             FROM posts 
             WHERE deleted_at IS NULL"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, post_id: i32) -> Result<Post, sqlx::Error> {
        sqlx::query_as!(
            Post,
            "SELECT id, title, content, attachments, reactions, nook_id, created_at, updated_at, deleted_at 
             FROM posts 
             WHERE id = $1 AND deleted_at IS NULL",
            post_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_nook_id(pool: &PgPool, nook_id: &str) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as!(
            Post,
            "SELECT id, title, content, attachments, reactions, nook_id, created_at, updated_at, deleted_at 
             FROM posts 
             WHERE nook_id = $1 AND deleted_at IS NULL",
            nook_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(pool: &PgPool, new_post: NewPost) -> Result<Post, sqlx::Error> {
        sqlx::query_as!(
            Post,
            "INSERT INTO posts (title, content, attachments, nook_id) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, title, content, attachments, reactions, nook_id, created_at, updated_at, deleted_at",
            new_post.title,
            new_post.content,
            new_post.attachments,
            new_post.nook_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        post_id: i32,
        update_post: PostChanges,
    ) -> Result<Post, sqlx::Error> {
        // Build dynamic update query
        let mut set_clauses: Vec<String> = vec!["updated_at = CURRENT_TIMESTAMP".to_string()];
        let mut param_index = 1;

        if let Some(_) = &update_post.title {
            set_clauses.push(format!("title = ${}", param_index));
            param_index += 1;
        }
        if let Some(_) = &update_post.content {
            set_clauses.push(format!("content = ${}", param_index));
            param_index += 1;
        }
        if let Some(_) = &update_post.attachments {
            set_clauses.push(format!("attachments = ${}", param_index));
            param_index += 1;
        }
        if let Some(_) = &update_post.nook_id {
            set_clauses.push(format!("nook_id = ${}", param_index));
            param_index += 1;
        }

        let query = format!(
            "UPDATE posts SET {} WHERE id = ${} AND deleted_at IS NULL RETURNING id, title, content, attachments, reactions, nook_id, created_at, updated_at, deleted_at",
            set_clauses.join(", "),
            param_index
        );

        let mut sqlx_query = sqlx::query_as::<_, Post>(&query);
        
        if let Some(title) = &update_post.title {
            sqlx_query = sqlx_query.bind(title);
        }
        if let Some(content) = &update_post.content {
            sqlx_query = sqlx_query.bind(content);
        }
        if let Some(attachments) = &update_post.attachments {
            sqlx_query = sqlx_query.bind(attachments);
        }
        if let Some(nook_id) = &update_post.nook_id {
            sqlx_query = sqlx_query.bind(nook_id);
        }
        
        sqlx_query = sqlx_query.bind(post_id);

        sqlx_query.fetch_one(pool).await
    }

    pub async fn delete(pool: &PgPool, post_id: i32) -> Result<Post, sqlx::Error> {
        sqlx::query_as!(
            Post,
            "UPDATE posts 
             SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP 
             WHERE id = $1 AND deleted_at IS NULL 
             RETURNING id, title, content, attachments, reactions, nook_id, created_at, updated_at, deleted_at",
            post_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn react(
        pool: &PgPool,
        post_id: i32,
        unicode: String,
        action: i8,
    ) -> Result<(), sqlx::Error> {
        // Update the reactions JSONB field in the posts table
        sqlx::query!(
            "UPDATE posts 
             SET reactions = COALESCE(reactions, '{}'::jsonb) || jsonb_build_object($2::text, COALESCE((reactions->>$2::text)::int, 0) + $3::int),
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $1",
            post_id,
            unicode,
            action as i32
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
