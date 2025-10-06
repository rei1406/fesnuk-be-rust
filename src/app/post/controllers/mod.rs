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
    match PostService::get_all_posts(&pool).await {
        Ok(posts) => ApiResponse::success(
            "Posts retrieved successfully".to_string(),
            Some(posts.into_iter().map(|p| p.into()).collect()),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve posts".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_posts_by_nook(
    State(pool): State<DBPool>,
    Path(nook_id): Path<String>,
) -> ApiResponse<Vec<PostResponse>> {
    match PostService::get_posts_by_nook_id(&pool, &nook_id).await {
        Ok(posts) => ApiResponse::success(
            "Posts retrieved successfully".to_string(),
            Some(posts.into_iter().map(|p| p.into()).collect()),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve posts".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_post(
    State(pool): State<DBPool>,
    Path(id): Path<i32>,
) -> ApiResponse<PostResponse> {
    match PostService::get_post_by_id(&pool, id).await {
        Ok(post) => {
            ApiResponse::success("Post retrieved successfully".to_string(), Some(post.into()), Some(StatusCode::OK))
        }
        Err(_) => ApiResponse::error("Post not found".to_string(), Some(StatusCode::NOT_FOUND)),
    }
}

async fn create_post(
    State(pool): State<DBPool>,
    Json(create_dto): Json<CreatePostDto>,
) -> ApiResponse<PostResponse> {
    match PostService::create_post(&pool, create_dto.to_new_post()).await {
        Ok(post) => ApiResponse::success("Post created successfully".to_string(), Some(post.into()), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create post".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn react_post(
    State(pool): State<DBPool>,
    Json(react_dto): Json<ReactPostDto>,
) -> ApiResponse<PostResponse> {
    let post_id = react_dto.post_id;
    match PostService::react_to_post(&pool, post_id, react_dto.to_reaction()).await {
        Ok(_) => {
            // After reacting, fetch the updated post
            match PostService::get_post_by_id(&pool, post_id).await {
                Ok(post) => ApiResponse::success("Post reaction added successfully".to_string(), Some(post.into()), Some(StatusCode::OK)),
                Err(_) => ApiResponse::error("Failed to retrieve updated post".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
            }
        }
        Err(_) => ApiResponse::error("Failed to react to post".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}