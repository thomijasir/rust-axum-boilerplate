use crate::routes::user_route;
use std::sync::Arc;
// Register Router API
use crate::AppState;
use axum::routing::get;
use axum::Router;

pub fn create() -> Router<Arc<AppState>> {
    // Route Index
    let route_index = Router::new()
        .nest("/users", user_route::router())
        .route("/", get(ping));
    // Home
    Router::new()
        .nest("/api/v1", route_index)
        .route("/", get(ping))
}
async fn ping() -> &'static str {
    "ping"
}
