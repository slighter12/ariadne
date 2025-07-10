use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::{info, warn, error};
use std::time::Instant;

pub async fn request_logger(
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("unknown");
    
    // Log incoming request
    info!("📥 Incoming request: {} {} (User-Agent: {})", method, uri, user_agent);
    
    // Process the request
    let response = next.run(req).await;
    
    // Calculate response time
    let duration = start.elapsed();
    let status = response.status();
    
    // Log response based on status code
    match status {
        StatusCode::OK | StatusCode::CREATED => {
            info!("📤 Response: {} {} - {} ({}ms)", method, uri, status, duration.as_millis());
        }
        StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => {
            warn!("📤 Response: {} {} - {} ({}ms)", method, uri, status, duration.as_millis());
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            error!("📤 Response: {} {} - {} ({}ms)", method, uri, status, duration.as_millis());
        }
        _ => {
            info!("📤 Response: {} {} - {} ({}ms)", method, uri, status, duration.as_millis());
        }
    }
    
    response
} 