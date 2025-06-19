use crate::modules::AppRoute;
use crate::{constant, utils::errors::HttpError, AppState};
use axum::{error_handling::HandleErrorLayer, extract::Request, response::IntoResponse};
use std::net::{
    // Ipv4Addr,
    SocketAddr,
};
use std::sync::Arc;
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

pub struct ApplicationServer;
impl ApplicationServer {
    pub async fn serve(app_state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
        // Define layered services
        let timeout_secs = app_state.env.timeout;
        let route_layer = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(HandleErrorLayer::new(move |err| {
                Self::handle_timeout_error(err, timeout_secs)
            }))
            .timeout(Duration::from_secs(timeout_secs))
            .layer(Self::cors_config())
            .layer(BufferLayer::<Request>::new(1024))
            .layer(RateLimitLayer::new(10240, Duration::from_secs(1)));

        let app = AppRoute::register()
            .with_state(app_state.clone())
            .layer(route_layer)
            .fallback(Self::handle_404);

        let port: u16 = app_state.env.port;
        // let addr: SocketAddr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
        let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await?;
        info!("🚀 Server has launched on http://{addr}");
        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .expect("server error");
        Ok(())
    }
    fn cors_config() -> CorsLayer {
        CorsLayer::new()
            .allow_origin(
                constant::CORS_WHITELIST
                    .iter()
                    .map(|origin| origin.parse().expect("invalid CORS origin"))
                    .collect::<Vec<_>>(),
            )
            .allow_methods(constant::METHOD_ALLOW)
            .allow_headers(constant::HEADER_ALLOW)
    }
    async fn handle_timeout_error(
        err: Box<dyn std::error::Error + Send + Sync>,
        timeout: u64,
    ) -> impl IntoResponse {
        if err.is::<tower::timeout::error::Elapsed>() {
            let msg = format!("Request exceeded {} second timeout", timeout);
            HttpError::timeout(msg)
        } else {
            HttpError::server_error("An unexpected error occurred")
        }
    }
    async fn shutdown_signal() {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        info!("Shutdown signal received, starting graceful shutdown");
    }
    async fn handle_404() -> impl IntoResponse {
        HttpError::not_found("The requested resource was not found")
    }
}
