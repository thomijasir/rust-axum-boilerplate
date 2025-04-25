use crate::docs::api_doc::ApiDoc;
use crate::routes::user_route;
use crate::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create() -> Router<Arc<AppState>> {
    // Route Index
    let route_index = Router::new()
        .nest("/users", user_route::router())
        .route("/", get(ping));
    // Docs Route
    let openapi = ApiDoc::openapi();
    Router::new()
        .nest("/api/v1", route_index)
        .route("/", get(ping))
        .merge(SwaggerUi::new("/docs").url("/api/openapi.json", openapi))
}
async fn ping() -> &'static str {
    "ping"
}
