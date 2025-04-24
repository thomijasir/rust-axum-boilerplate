use axum_boilerplate::config::Config;
use axum_boilerplate::constant;
use axum_boilerplate::server::ApplicationServer;
use axum_boilerplate::utils::cache::Cache;
use axum_boilerplate::utils::logger::Logger;
use axum_boilerplate::AppState;
use dotenv::dotenv;
use std::sync::Arc;
use std::time::Duration;
#[tokio::main]
async fn main() {
    // Load Environment
    dotenv().ok();
    let config = Config::load();
    // Init load
    let _logger = Logger::init(&config.app_env);
    // Create in memory cache
    let cache = Cache::new(Duration::from_secs(constant::CACHE_TIMEOUT));
    // Application state
    let app_state = Arc::new(AppState { env: config, cache });
    // Serve Application
    ApplicationServer::serve(app_state)
        .await
        .expect("Error starting server");
}
