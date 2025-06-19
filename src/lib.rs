pub mod config;
pub mod constant;
pub mod database;
pub mod docs;
pub mod dto;
pub mod middlewares;
pub mod modules;
pub mod schema;
pub mod server;
pub mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: config::Config,
    pub cache: utils::cache::Cache,
}
