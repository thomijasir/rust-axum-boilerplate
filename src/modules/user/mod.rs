pub mod user_controller;
pub mod user_model;
pub mod user_service;
use crate::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

// Define Routes
pub struct Routes;
impl Routes {
    pub fn index() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", get(user_controller::get_all_users_use_struct))
            .route("/json", get(user_controller::get_all_users_use_json))
    }
}
