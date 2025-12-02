use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use crate::domain::responses::ApiResponse;
use crate::delivery::middleware::response_handler::with_status;
use crate::state::ConcreteAppState;
use serde_json::json;

// 健康檢查路由
pub fn health_router(app_state: ConcreteAppState) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/version", get(get_version))
        .with_state(app_state)
}

// GET /api/health
async fn health_check(
    State(_app_state): State<ConcreteAppState>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let response = ApiResponse::success(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }));
    with_status(Json(response))
}

// GET /api/version
async fn get_version(
    State(_app_state): State<ConcreteAppState>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let response = ApiResponse::success(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": "ariadne",
        "description": "A tool for creating and managing relations between videos",
        "homepage": "https://github.com/slighter12/ariadne",
        "license": "GNU GPLv3",
    }));
    with_status(Json(response))
}