use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub fn create_db_pool() -> DBPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
	Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
