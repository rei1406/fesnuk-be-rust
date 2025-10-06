use super::models::{NewNook, Nook};
use sqlx::PgPool;

pub struct NookRepository;

impl NookRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Nook>, sqlx::Error> {
        sqlx::query_as!(
            Nook,
            "SELECT id, name, description, image, created_at, updated_at, deleted_at 
             FROM nooks 
             WHERE deleted_at IS NULL"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, nook_id: &str) -> Result<Nook, sqlx::Error> {
        sqlx::query_as!(
            Nook,
            "SELECT id, name, description, image, created_at, updated_at, deleted_at 
             FROM nooks 
             WHERE id = $1 AND deleted_at IS NULL",
            nook_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id_with_deleted(
        pool: &PgPool,
        nook_id: &str,
    ) -> Result<Option<Nook>, sqlx::Error> {
        sqlx::query_as!(
            Nook,
            "SELECT id, name, description, image, created_at, updated_at, deleted_at 
             FROM nooks 
             WHERE id = $1",
            nook_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, new_nook: NewNook) -> Result<Nook, sqlx::Error> {
        // Check if there's a soft-deleted nook with the same ID
        match Self::find_by_id_with_deleted(pool, &new_nook.id).await? {
            Some(existing_nook) if existing_nook.deleted_at.is_some() => {
                // If exists and is soft-deleted, hard delete it first
                sqlx::query!("DELETE FROM nooks WHERE id = $1", &new_nook.id)
                    .execute(pool)
                    .await?;
            }
            Some(_) => {
                // If exists and not soft-deleted, return error
                return Err(sqlx::Error::RowNotFound);
            }
            None => {}
        }

        // Create new nook
        sqlx::query_as!(
            Nook,
            "INSERT INTO nooks (id, name, description) 
             VALUES ($1, $2, $3) 
             RETURNING id, name, description, image, created_at, updated_at, deleted_at",
            new_nook.id,
            new_nook.name,
            new_nook.description
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, nook_id: &str) -> Result<Nook, sqlx::Error> {
        sqlx::query_as!(
            Nook,
            "UPDATE nooks 
             SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP 
             WHERE id = $1 AND deleted_at IS NULL 
             RETURNING id, name, description, image, created_at, updated_at, deleted_at",
            nook_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn hard_delete(pool: &PgPool, nook_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM nooks WHERE id = $1", nook_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
