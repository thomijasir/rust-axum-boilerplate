use std::sync::Arc;

use axum::{routing::get, Router};

use crate::controllers::user_controller;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(user_controller::get_all_users))
}
