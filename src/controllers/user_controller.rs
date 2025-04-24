use crate::{
    services::user_service,
    utils::{errors::HttpError, responses::HttpResponse},
    AppState,
};
use axum::{
    extract::{Json, State},
    // Extension,
};
use serde_json::json;
use std::sync::Arc;

pub async fn get_all_users(
    // Extension(_token): Extension<String>,
    State(_state): State<Arc<AppState>>,
    Json(_body): Json<serde_json::Value>,
    // Query(_params): Query<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let data = json!({
        "user_id": "123456789",
        "full_name": "Thomi",
        "email": "thomi@example.com",
        "phone_number": "1234567890",
    });

    // Call the service function
    user_service::get_print_user();

    // Return the response
    Ok(HttpResponse::ok(
        Some(data),
        Some("CREDENTIAL_PIN_ADDED".to_string()),
    ))
}
