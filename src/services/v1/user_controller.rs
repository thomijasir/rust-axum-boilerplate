use super::user_model::{CreateUser, User};
use crate::utils::formatter::{Formatter, ResponseError};
use axum::{extract::Path, response::Response, Json};

pub async fn list_users() -> Result<Response, ResponseError> {
  let users = vec![
    User {
      id: 1,
      name: "John Doe".to_string(),
      email: "john@example.com".to_string(),
    },
    User {
      id: 2,
      name: "Jane Smith".to_string(),
      email: "jane@example.com".to_string(),
    },
  ];
  Ok(Formatter::json(users))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
  // Simulated user creation
  Json(User {
    id: 3, // In a real app, this would be generated
    name: payload.name,
    email: payload.email,
  })
}

pub async fn get_user(Path(id): Path<u64>) -> Json<User> {
  // Simulated single user fetch
  Json(User {
    id,
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
  })
}

pub async fn update_user(Path(id): Path<u64>, Json(payload): Json<CreateUser>) -> Json<User> {
  // Simulated user update
  Json(User {
    id,
    name: payload.name,
    email: payload.email,
  })
}

pub async fn delete_user(Path(_id): Path<u64>) -> Json<&'static str> {
  // Simulated user deletion
  Json("User deleted successfully")
}
