mod constant;
mod routes;
mod server;
mod services;
mod utils;

use constant::AppConfig;
use dotenvy::dotenv;
use server::ApplicationServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Load Configuration
  dotenv().ok();
  let config = AppConfig::load();

  // Initialize Database
  // TODO: Initialize Database

  // Start Server
  ApplicationServer::serve(config).await?;

  Ok(())
}
