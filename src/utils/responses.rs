use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct HttpResponseFormat<T = serde_json::Value>
where
    T: serde::Serialize,
{
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

use serde::Serialize;

#[derive(Debug, Clone)]
pub struct HttpResponse<T = serde_json::Value>
where
    T: Serialize,
{
    pub message: String,
    pub status: StatusCode,
    pub data: Option<T>,
}

impl<T: Serialize> HttpResponse<T> {
    pub fn new(
        message: impl Into<String>,
        status: StatusCode,
        data: Option<T>,
    ) -> Self {
        HttpResponse {
            message: message.into(),
            status,
            data,
        }
    }

    fn get_message(
        message: Option<String>,
        default: &str,
    ) -> String {
        message.unwrap_or_else(|| default.to_string())
    }

    pub fn ok(
        data: Option<T>,
        msg: Option<String>,
    ) -> Self {
        HttpResponse {
            status: StatusCode::OK,
            message: Self::get_message(msg, "OK"),
            data,
        }
    }

    pub fn created(
        data: Option<T>,
        msg: Option<String>,
    ) -> Self {
        HttpResponse {
            status: StatusCode::CREATED,
            message: Self::get_message(msg, "CREATED"),
            data,
        }
    }

    pub fn delete(id: String) -> HttpResponse<()> {
        HttpResponse {
            status: StatusCode::OK,
            message: format!("DELETED:{id}"),
            data: None,
        }
    }

    pub fn into_http_response(self) -> Response {
        let format = HttpResponseFormat {
            success: self.status.is_success(),
            message: self.message,
            data: self.data,
        };
        (self.status, Json(format)).into_response()
    }
}

impl<T: Serialize> fmt::Display for HttpResponse<T> {
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

impl<T: Serialize> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
