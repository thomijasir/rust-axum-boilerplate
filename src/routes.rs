use axum::{routing::get, Router};

// Router v1
fn route_main() -> Router {
  Router::new().route("/hello", get(hello))
}

// Index Router
pub fn route_app() -> Router {
  Router::new()
    .nest("/api/v1", route_main())
    .route("/ping", get(health))
    .route("/", get(|| async { "Hello, World!" }))
}

async fn hello() -> &'static str {
  "Hello, World!"
}

async fn health() -> &'static str {
  "pong"
}
