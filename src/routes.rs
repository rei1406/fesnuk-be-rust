use axum::{Router};
use axum::routing::get;
use crate::db::DBPool;
use crate::app::nook::controllers::nook_routes;

pub fn all_routes() -> Router<DBPool> {
    Router::new()
	.route("/", get(|| async { "Hello this is root of API endpoint" }))
	.nest("/nooks", nook_routes())
}