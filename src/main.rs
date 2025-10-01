use axum::Router;
use fesnuk::listener;
use fesnuk::{db, routes};

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::create_db_pool();

    let app = Router::new()
        .nest("/api/v1", routes::all_routes())
        .with_state(pool);

    let (listener, _) = listener::get_listener().await;
    axum::serve(listener, app).await.unwrap();
}
