use crate::{
    app::nook::{
        dto::{CreateNookDto, NookResponse, UpdateNookDto}, services::NookService
    },
    utils::response::ApiResponse,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
    http::{StatusCode}
};

use crate::db::DBPool;


pub fn nook_routes() -> Router<DBPool> {
    Router::new()
        .route("/", get(get_nooks))
        .route("/", post(create_nook))
        .route("/{id}", get(get_nook))
        .route("/{id}", put(update_nook))
        .route("/{id}", delete(delete_nook))
}

async fn get_nooks(State(pool): State<DBPool>) -> ApiResponse<Vec<NookResponse>> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match NookService::get_all_nooks(&mut conn) {
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
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match NookService::get_nook_by_id(&mut conn, &id) {
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
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match NookService::create_nook(&mut conn, create_dto) {
        Ok(nook) => ApiResponse::success("Nook created successfully".to_string(), Some(nook), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create nook".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn update_nook(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Json(update_dto): Json<UpdateNookDto>,
) -> ApiResponse<NookResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match NookService::update_nook(&mut conn, &id, update_dto) {
        Ok(nook) => ApiResponse::success("Nook updated successfully".to_string(), Some(nook), Some(StatusCode::OK)),
        Err(_) => ApiResponse::error("Failed to update nook".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn delete_nook(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
) -> ApiResponse<NookResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match NookService::delete_nook(&mut conn, &id) {
        Ok(nook) => ApiResponse::success("Nook deleted successfully".to_string(), Some(nook), Some(StatusCode::OK)),
        Err(_) => ApiResponse::error("Failed to delete nook".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
