use axum::{
    response::Json,
    routing::get,
    Router,
};
use crate::domain::responses::{ApiResponse, HealthInfo, RootInfo};
use crate::domain::common::API_VERSION;
use tracing::info;

// 建立健康檢查路由
pub fn create_health_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
}



// GET /health
async fn health_check() -> Json<ApiResponse<HealthInfo>> {
    info!("🏥 Health check requested");
    Json(ApiResponse::success(HealthInfo {
        status: "healthy".to_string(),
        message: "Ariadne backend is running".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }))
}

// GET /
async fn root() -> Json<ApiResponse<RootInfo>> {
    info!("🏠 Root endpoint accessed");
    let mut endpoints = std::collections::HashMap::new();
    endpoints.insert("health".to_string(), "/health".to_string());
    endpoints.insert("relations".to_string(), "/api/relations".to_string());
    
    Json(ApiResponse::success(RootInfo {
        message: "Welcome to Ariadne API".to_string(),
        version: API_VERSION.to_string(),
        endpoints,
    }))
} 