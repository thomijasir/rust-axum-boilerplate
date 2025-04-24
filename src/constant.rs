use axum::http::{header, HeaderName, Method};

pub const CACHE_TIMEOUT: u64 = 3600; // 1 hour default cache
pub const METHOD_ALLOW: [Method; 4] = [Method::GET, Method::POST, Method::PUT, Method::DELETE];
pub const HEADER_ALLOW: [HeaderName; 2] = [header::CONTENT_TYPE, header::ACCEPT];
pub const CORS_WHITELIST: [&str; 2] = ["http://localhost:5000", "http://localhost:8080"];
