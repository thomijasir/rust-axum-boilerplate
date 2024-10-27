// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

use crate::{constant::AppConfig, routes};
use axum::response::IntoResponse;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Json, Router};
use serde_json::json;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{
  cors::{Any, CorsLayer},
  trace::TraceLayer,
};
pub struct ApplicationServer;

impl ApplicationServer {
  pub async fn serve(config: AppConfig) -> anyhow::Result<()> {
    // Define tracing middleware
    let trace = TraceLayer::new_for_http();

    // Define cors with specific origins
    let whitelist: [&str; 2] = ["http://localhost:5000", "http://localhost:8080"];
    let cors = CorsLayer::new()
      .allow_origin(whitelist.map(|origin| origin.parse().unwrap()))
      .allow_methods(Any)
      .allow_headers(Any);

    // Define layered services
    let timeout_secs = config.timeout;
    let route_layer = ServiceBuilder::new()
      .layer(trace)
      .layer(HandleErrorLayer::new(move |err| {
        Self::handle_timeout_error(err, timeout_secs)
      }))
      .timeout(Duration::from_secs(timeout_secs))
      .layer(cors)
      .layer(BufferLayer::new(1024))
      .layer(RateLimitLayer::new(5, Duration::from_secs(1)));

    // build our application with a single route
    let app: Router = routes::route_app()
      .fallback(Self::handle_404)
      .layer(route_layer);

    let port: u16 = config.port;
    let addr: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    println!("Server running on http://{}", addr);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
  }

  async fn handle_timeout_error(
    err: BoxError,
    timeout: u64,
  ) -> (StatusCode, Json<serde_json::Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
      (
        StatusCode::REQUEST_TIMEOUT,
        Json(json!({
            "error":
                format!(
                    "request took longer than the configured {} second timeout",
                    timeout
                )
        })),
      )
    } else {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "error": format!("unhandled internal error: {}", err)
        })),
      )
    }
  }

  async fn handle_404() -> impl IntoResponse {
    (
      StatusCode::NOT_FOUND,
      Json(json!({
        "error": format!("NOT FOUND!")
      })),
    )
  }
}
