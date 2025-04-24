use crate::utils::errors::HttpError;
use axum::{
    body::Body,
    http::{header, Request},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct BasicAuth {
    pub token: String,
}
pub async fn auth_header(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, HttpError> {
    let get_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| HttpError::unauthorized("AUTHORIZATION_REQUIRED"))
        .and_then(|header| {
            header
                .to_str()
                .map_err(|_| HttpError::forbidden("INVALID_AUTHORIZATION"))
        })
        .map(|val| val.strip_prefix("Bearer ").unwrap_or(val).to_string())?;

    // Store token in request extensions
    req.extensions_mut().insert(get_token);

    // Run the next middleware/handler
    Ok(next.run(req).await)
}
