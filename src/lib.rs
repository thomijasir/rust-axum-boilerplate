pub mod config;
pub mod constant;
pub mod controllers;
pub mod database;
pub mod middlewares;
pub mod models;
pub mod route;
pub mod routes;
pub mod schema;
pub mod server;
pub mod services;
pub mod utils;
pub mod docs;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: config::Config,
    pub cache: utils::cache::Cache,
}
