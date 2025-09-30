use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use fesnuk::utils::response::ApiResponse;
use serde::Serialize;
use serde_json::{Value, json};
use std::net::Ipv4Addr;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

async fn persons() -> ApiResponse<Person> {
    ApiResponse::success(
        "Hello, World!".to_string(),
        Some(Person {
            name: "John Doe".to_string(),
            age: 30,
        }),
        Some(StatusCode::OK),
    )
}

async fn persons_by_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({
        "message": "Hello, World!",
        "data": {
            "name": name,
            "age": 30
        }
    }))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let person_routes = Router::new()
        .route("/", get(persons))
        .route("/{name}", get(persons_by_name));

    let api_routes = Router::new()
        .route("/", get(|| async { "Hello this is root of API endpoint" }))
        .route("/hello", get(|| async { "Hello, World! (2)" }))
        .nest("/persons", person_routes);

    let app = Router::new().nest("/api/v1", api_routes);

    // run our app with hyper, listening globally on port 3000
    let port: u16 = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("FUNCTIONS_CUSTOMHANDLER_PORT must be a number");
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
