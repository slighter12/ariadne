use tower_http::cors::{Any, CorsLayer};

// 建立 CORS 配置
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // 在生產環境中應指定前端來源
        .allow_methods(Any)
        .allow_headers(Any)
}