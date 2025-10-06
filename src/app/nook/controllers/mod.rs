use crate::{
    app::nook::{
        dto::{CreateNookDto, NookResponse}, services::NookService
    },
    utils::response::ApiResponse,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
    http::{StatusCode}
};

use crate::db::DBPool;


pub fn nook_routes() -> Router<DBPool> {
    Router::new()
        .route("/", get(get_nooks))
        .route("/", post(create_nook))
        .route("/{id}", get(get_nook))
        .route("/{id}", delete(delete_nook))
}

async fn get_nooks(State(pool): State<DBPool>) -> ApiResponse<Vec<NookResponse>> {
    match NookService::get_all_nooks(&pool).await {
        Ok(nooks) => ApiResponse::success(
            "Nooks retrieved successfully".to_string(),
            Some(nooks),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve nooks".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_nook(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
) -> ApiResponse<NookResponse> {
    match NookService::get_nook_by_id(&pool, &id).await {
        Ok(nook) => {
            ApiResponse::success("Nook retrieved successfully".to_string(), Some(nook), Some(StatusCode::OK))
        }
        Err(_) => ApiResponse::error("Nook not found".to_string(), Some(StatusCode::NOT_FOUND)),
    }
}

async fn create_nook(
    State(pool): State<DBPool>,
    Json(create_dto): Json<CreateNookDto>,
) -> ApiResponse<NookResponse> {
    match NookService::create_nook(&pool, create_dto).await {
        Ok(nook) => ApiResponse::success("Nook created successfully".to_string(), Some(nook), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create nook".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn delete_nook(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
) -> ApiResponse<NookResponse> {
    match NookService::delete_nook(&pool, &id).await {
        Ok(nook) => ApiResponse::success("Nook deleted successfully".to_string(), Some(nook), Some(StatusCode::OK)),
        Err(_) => ApiResponse::error("Failed to delete nook".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
