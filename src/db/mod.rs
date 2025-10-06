use sqlx::{PgPool, Pool, Postgres};

pub type DBPool = PgPool;

pub async fn create_db_pool() -> DBPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create pool")
}
