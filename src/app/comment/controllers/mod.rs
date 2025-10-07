use crate::{
    app::comment::{
        dto::{CommentResponse, CreateCommentDto, ReplyCommentDto},
        services::CommentService,
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

pub fn comment_routes() -> Router<DBPool> {
    Router::new()
        .route("/", get(get_all_comments))
        .route("/", post(create_comment))
        .route("/{id}", get(get_comment))
        .route("/{id}", delete(delete_comment))
        .route("/post/{post_id}", get(get_comments_by_post))
        .route("/{parent_id}/replies", get(get_replies_by_comment))
        .route("/reply", post(reply_to_comment))
}

async fn get_all_comments(State(pool): State<DBPool>) -> ApiResponse<Vec<CommentResponse>> {
    match CommentService::get_all_comments(&pool).await {
        Ok(comments) => ApiResponse::success(
            "Comments retrieved successfully".to_string(),
            Some(comments),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve comments".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_comments_by_post(
    State(pool): State<DBPool>,
    Path(post_id): Path<i32>,
) -> ApiResponse<Vec<CommentResponse>> {
    match CommentService::get_comments_by_post_id(&pool, post_id).await {
        Ok(comments) => ApiResponse::success(
            "Comments retrieved successfully".to_string(),
            Some(comments),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve comments".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_replies_by_comment(
    State(pool): State<DBPool>,
    Path(parent_id): Path<i32>,
) -> ApiResponse<Vec<CommentResponse>> {
    match CommentService::get_replies_by_comment_id(&pool, parent_id).await {
        Ok(comments) => ApiResponse::success(
            "Comments retrieved successfully".to_string(),
            Some(comments),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve comments".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_comment(
    State(pool): State<DBPool>,
    Path(id): Path<i32>,
) -> ApiResponse<CommentResponse> {
    match CommentService::get_comment_by_id(&pool, id).await {
        Ok(comment) => {
            ApiResponse::success("Comment retrieved successfully".to_string(), Some(comment), Some(StatusCode::OK))
        }
        Err(_) => ApiResponse::error("Comment not found".to_string(), Some(StatusCode::NOT_FOUND)),
    }
}

async fn create_comment(
    State(pool): State<DBPool>,
    Json(create_dto): Json<CreateCommentDto>,
) -> ApiResponse<CommentResponse> {
    match CommentService::create_comment(&pool, create_dto).await {
        Ok(comment) => ApiResponse::success("Comment created successfully".to_string(), Some(comment), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create comment".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn reply_to_comment(
    State(pool): State<DBPool>,
    Json(reply_dto): Json<ReplyCommentDto>,
) -> ApiResponse<CommentResponse> {
    match CommentService::reply_to_comment(&pool, reply_dto).await {
        Ok(comment) => ApiResponse::success("Reply created successfully".to_string(), Some(comment), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create reply".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn delete_comment(
    State(pool): State<DBPool>,
    Path(id): Path<i32>,
) -> ApiResponse<CommentResponse> {
    match CommentService::delete_comment(&pool, id).await {
        Ok(comment) => ApiResponse::success("Comment deleted successfully".to_string(), Some(comment), Some(StatusCode::OK)),
        Err(_) => ApiResponse::error("Failed to delete comment".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}