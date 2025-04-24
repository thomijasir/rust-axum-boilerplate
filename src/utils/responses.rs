use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ResponsesMessage {
    OK,
    CREATED,
}

impl ResponsesMessage {
    pub fn to_str(&self) -> String {
        match self {
            ResponsesMessage::OK => "OK".to_string(),
            ResponsesMessage::CREATED => "Resource successfully created".to_string(),
        }
    }
}

impl Display for ResponsesMessage {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub message: String,
    pub status: StatusCode,
    pub data: Option<serde_json::Value>,
}
impl HttpResponse {
    pub fn new(
        message: impl Into<String>,
        status: StatusCode,
        data: Option<serde_json::Value>,
    ) -> Self {
        HttpResponse {
            message: message.into(),
            status,
            data,
        }
    }
    fn get_message(
        message: Option<String>,
        default: ResponsesMessage,
    ) -> String {
        message.unwrap_or_else(|| default.to_string())
    }
    // Use for get method
    pub fn ok(
        data: Option<serde_json::Value>,
        msg: Option<String>,
    ) -> Self {
        HttpResponse {
            status: StatusCode::OK,
            message: Self::get_message(msg, ResponsesMessage::OK),
            data,
        }
    }
    // Use for post, put, and patch
    pub fn created(
        data: Option<serde_json::Value>,
        msg: Option<String>,
    ) -> Self {
        HttpResponse {
            status: StatusCode::CREATED,
            message: Self::get_message(msg, ResponsesMessage::CREATED),
            data,
        }
    }
    // use for delete
    pub fn delete(id: String) -> Self {
        HttpResponse {
            status: StatusCode::OK,
            message: format!("DELETED:{id}"),
            data: None,
        }
    }
    pub fn into_http_response(self) -> Response {
        let body = json!({
            "success": true,
            "message": self.message,
            "data": self.data,
        });
        (self.status, Json(body)).into_response()
    }
}
impl fmt::Display for HttpResponse {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "HttpResponse: message: {}, status: {}",
            self.message, self.status
        )
    }
}
impl IntoResponse for HttpResponse {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
