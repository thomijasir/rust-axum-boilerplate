#![allow(unused_imports)]
#![allow(unused_variables)]

use axum::{
  routing::{delete, get, post, put},
  Router,
};

use crate::services::v1::user_controller;

// Router v1
fn route_main() -> Router {
  Router::new()
    .route("/users", get(user_controller::list_users))
    .route("/users", post(user_controller::create_user))
    .route(
      "/users/:id",
      get(user_controller::get_user)
        .put(user_controller::update_user)
        .delete(user_controller::delete_user),
    )
    .route("/hello", get(hello))
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
