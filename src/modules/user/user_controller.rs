use crate::{
    utils::{errors::HttpError, responses::HttpResponse},
    AppState,
};
use axum::{
    extract::State,
    // Extension,
};
use serde_json::json;
use std::sync::Arc;

use super::{user_model::UserData, user_service};

#[utoipa::path(
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "List users", body = [UserData])
    )
)]
pub async fn get_all_users_use_struct(
    // Extension(_token): Extension<String>,
    State(_state): State<Arc<AppState>>,
    // BodyJson(_body): BodyJson<serde_json::Value>,
    // Query(_params): Query<HashMap<String, String>>,
) -> Result<HttpResponse<UserData>, HttpError> {
    let user_data = UserData {
        user_id: "123456789".to_string(),
        full_name: "Thomi".to_string(),
        email: "thomi@example.com".to_string(),
        phone_number: "1234567890".to_string(),
    };

    // Call the service function
    user_service::get_print_user();

    // Return the response
    Ok(HttpResponse::ok(
        Some(user_data),
        Some("CREDENTIAL_PIN_ADDED".to_string()),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/json",
    responses(
        (status = 200, description = "List users as JSON", body = serde_json::Value)
    )
)]
pub async fn get_all_users_use_json(
    // Extension(_token): Extension<String>,
    State(_state): State<Arc<AppState>>,
    // Json(_body): Json<serde_json::Value>,
    // Query(_params): Query<HashMap<String, String>>,
) -> Result<HttpResponse<serde_json::Value>, HttpError> {
    let user_data = json!({
        "user_id": "123456789",
        "full_name": "Thomi",
        "email": "thomi@example.com",
        "phone_number": "1234567890"
    });

    // Call the service function
    user_service::get_print_user();

    // Return the response
    Ok(HttpResponse::ok(
        Some(user_data),
        Some("CREDENTIAL_PIN_ADDED".to_string()),
    ))
}
