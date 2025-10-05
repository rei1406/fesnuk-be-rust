use crate::{
    app::post::{
        dto::{CreatePostDto, PostResponse, ReactPostDto}, services::PostService
    },
    utils::response::ApiResponse,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
    http::{StatusCode}
};

use crate::db::DBPool;

pub fn post_routes() -> Router<DBPool> {
    Router::new()
        .route("/", get(get_posts))
        .route("/", post(create_post))
        .route("/{id}", get(get_post))
        .route("/nook/{nook_id}", get(get_posts_by_nook))
        .route("/react", post(react_post))
}

async fn get_posts(State(pool): State<DBPool>) -> ApiResponse<Vec<PostResponse>> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match PostService::get_all_posts(&mut conn) {
        Ok(posts) => ApiResponse::success(
            "Posts retrieved successfully".to_string(),
            Some(posts),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve posts".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_posts_by_nook(
    State(pool): State<DBPool>,
    Path(nook_id): Path<String>,
) -> ApiResponse<Vec<PostResponse>> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match PostService::get_posts_by_nook_id(&mut conn, &nook_id) {
        Ok(posts) => ApiResponse::success(
            "Posts retrieved successfully".to_string(),
            Some(posts),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve posts".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_post(
    State(pool): State<DBPool>,
    Path(id): Path<i32>,
) -> ApiResponse<PostResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match PostService::get_post_by_id(&mut conn, id) {
        Ok(post) => {
            ApiResponse::success("Post retrieved successfully".to_string(), Some(post), Some(StatusCode::OK))
        }
        Err(_) => ApiResponse::error("Post not found".to_string(), Some(StatusCode::NOT_FOUND)),
    }
}

async fn create_post(
    State(pool): State<DBPool>,
    Json(create_dto): Json<CreatePostDto>,
) -> ApiResponse<PostResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match PostService::create_post(&mut conn, create_dto) {
        Ok(post) => ApiResponse::success("Post created successfully".to_string(), Some(post), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create post".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn react_post(
    State(pool): State<DBPool>,
    Json(react_dto): Json<ReactPostDto>,
) -> ApiResponse<PostResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match PostService::react(&mut conn, react_dto) {
        Ok(post) => ApiResponse::success("Post reacted successfully".to_string(), Some(post), Some(StatusCode::OK)),
        Err(_) => ApiResponse::error("Failed to react post".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}