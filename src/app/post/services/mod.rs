use super::models::{NewPost, Post, PostChanges, PostReaction, PostWithNook};
use super::repositories::PostRepository;
use sqlx::PgPool;

pub struct PostService;

impl PostService {
    pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<PostWithNook>, sqlx::Error> {
        PostRepository::find_all(pool).await
    }

    pub async fn get_post_by_id(pool: &PgPool, post_id: i32) -> Result<PostWithNook, sqlx::Error> {
        PostRepository::find_by_id(pool, post_id).await
    }

    pub async fn get_posts_by_nook_id(pool: &PgPool, nook_id: &str) -> Result<Vec<PostWithNook>, sqlx::Error> {
        PostRepository::find_by_nook_id(pool, nook_id).await
    }

    pub async fn create_post(pool: &PgPool, new_post: NewPost) -> Result<Post, sqlx::Error> {
        PostRepository::create(pool, new_post).await
    }

    pub async fn update_post(
        pool: &PgPool,
        post_id: i32,
        update_post: PostChanges,
    ) -> Result<Post, sqlx::Error> {
        PostRepository::update(pool, post_id, update_post).await
    }

    pub async fn delete_post(pool: &PgPool, post_id: i32) -> Result<Post, sqlx::Error> {
        PostRepository::delete(pool, post_id).await
    }

    pub async fn react_to_post(
        pool: &PgPool,
        post_id: i32,
        reaction: PostReaction,
    ) -> Result<(), sqlx::Error> {
        PostRepository::react(pool, post_id, reaction.unicode, reaction.action).await
    }
}