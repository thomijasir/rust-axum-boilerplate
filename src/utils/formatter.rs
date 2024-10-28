use super::helper::Helper;
use axum::{
  http::{header, StatusCode},
  response::{IntoResponse, Response},
  Json,
};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub data: Option<T>,
  pub success: bool,
  pub message: String,
}

pub enum ResponseError {
  #[allow(dead_code)]
  BadRequest(String),
  #[allow(dead_code)]
  Unauthorized(String),
  #[allow(dead_code)]
  Forbidden(String),
  #[allow(dead_code)]
  NotFound(String),
  #[allow(dead_code)]
  InternalServerError(String),
  #[allow(dead_code)]
  Other(StatusCode, String),
}

impl IntoResponse for ResponseError {
  fn into_response(self) -> Response {
    let (status, message) = match self {
      ResponseError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
      ResponseError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
      ResponseError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
      ResponseError::NotFound(message) => (StatusCode::NOT_FOUND, message),
      ResponseError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
      ResponseError::Other(status, message) => (status, message),
    };

    let body: ApiResponse<()> = ApiResponse {
      data: None,
      success: false,
      message,
    };

    (status, Json(body)).into_response()
  }
}

pub struct Formatter;

impl Formatter {
  pub fn json<T>(data: T) -> Response
  where
    T: Serialize,
  {
    // Add headers
    let mut headers = header::HeaderMap::new();
    headers.insert(
      "X-Trace-ID",
      header::HeaderValue::from_str(&Helper::generate_id(None)).unwrap(),
    );
    headers.insert(
      "X-Timestamp",
      header::HeaderValue::from_str(&Utc::now().to_string()).unwrap(),
    );
    // Create response
    let response = ApiResponse {
      data: Some(data),
      success: true,
      message: String::from("success"),
    };

    (StatusCode::OK, headers, Json(response)).into_response()
  }
}
