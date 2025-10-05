use crate::{
    app::comment::{
        dto::{CommentResponse, CreateCommentDto, ReactCommentDto, ReplyCommentDto},
        services::CommentService,
    },
    utils::response::ApiResponse,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
    http::StatusCode,
};

use crate::db::DBPool;

pub fn comment_routes() -> Router<DBPool> {
    Router::new()
        .route("/post/{post_id}", get(get_root_comments))
        .route("/replies/{parent_id}", get(get_direct_children))
        .route("/", post(create_root_comment))
        .route("/reply", post(reply_comment))
        .route("/react", post(react_comment))
}

async fn get_root_comments(
    State(pool): State<DBPool>,
    Path(post_id): Path<i32>,
) -> ApiResponse<Vec<CommentResponse>> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match CommentService::get_root_comments_by_post_id(&mut conn, post_id) {
        Ok(comments) => ApiResponse::success(
            "Root comments retrieved successfully".to_string(),
            Some(comments),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve root comments".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_direct_children(
    State(pool): State<DBPool>,
    Path(parent_id): Path<i32>,
) -> ApiResponse<Vec<CommentResponse>> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match CommentService::get_direct_children_by_parent_id(&mut conn, parent_id) {
        Ok(comments) => ApiResponse::success(
            "Direct children comments retrieved successfully".to_string(),
            Some(comments),
            Some(StatusCode::OK),
        ),
        Err(_) => ApiResponse::error("Failed to retrieve direct children comments".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn create_root_comment(
    State(pool): State<DBPool>,
    Json(create_dto): Json<CreateCommentDto>,
) -> ApiResponse<CommentResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match CommentService::create_root_comment(&mut conn, create_dto) {
        Ok(comment) => ApiResponse::success("Root comment created successfully".to_string(), Some(comment), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create root comment".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn reply_comment(
    State(pool): State<DBPool>,
    Json(reply_dto): Json<ReplyCommentDto>,
) -> ApiResponse<CommentResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match CommentService::reply_to_comment(&mut conn, reply_dto) {
        Ok(comment) => ApiResponse::success("Reply comment created successfully".to_string(), Some(comment), Some(StatusCode::CREATED)),
        Err(_) => ApiResponse::error("Failed to create reply comment".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn react_comment(
    State(pool): State<DBPool>,
    Json(react_dto): Json<ReactCommentDto>,
) -> ApiResponse<CommentResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Database connection error".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    match CommentService::react_to_comment(&mut conn, react_dto) {
        Ok(comment) => ApiResponse::success("Comment reacted successfully".to_string(), Some(comment), Some(StatusCode::OK)),
        Err(_) => ApiResponse::error("Failed to react to comment".to_string(), Some(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}