use crate::app::nook::{
    dto::{CreateNookDto, NookResponse},
    repositories::NookRepository,
};
use sqlx::PgPool;

pub struct NookService;

impl NookService {
    pub async fn get_all_nooks(
        pool: &PgPool,
    ) -> Result<Vec<NookResponse>, sqlx::Error> {
        let nooks = NookRepository::find_all(pool).await?;
        Ok(nooks.into_iter().map(NookResponse::from).collect())
    }

    pub async fn get_nook_by_id(
        pool: &PgPool,
        id: &str,
    ) -> Result<NookResponse, sqlx::Error> {
        let nook = NookRepository::find_by_id(pool, id).await?;
        Ok(NookResponse::from(nook))
    }

    pub async fn create_nook(
        pool: &PgPool,
        dto: CreateNookDto,
    ) -> Result<NookResponse, sqlx::Error> {
        let nook = NookRepository::create(pool, dto.to_new_nook()).await?;
        Ok(NookResponse::from(nook))
    }

    pub async fn delete_nook(
        pool: &PgPool,
        id: &str,
    ) -> Result<NookResponse, sqlx::Error> {
        let nook = NookRepository::delete(pool, id).await?;
        Ok(NookResponse::from(nook))
    }
}
