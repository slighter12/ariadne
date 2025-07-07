// 模組宣告
mod models;
mod database;
mod routes;
mod state;
mod utils;
mod traits;
mod repositories;
mod services;

// 引入模組
use routes::create_router;
use state::Config;
use utils::create_cors_layer;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌系統
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 載入 .env 檔案
    dotenvy::dotenv().expect("Failed to read .env file");

    // 載入配置
    let config = Config::from_env()?;
    
    // 建立資料庫連接池
    let pool_arc = database::create_connection_pool(&config.database_config.database_url).await?;
    
    // 檢查資料庫健康狀態
    if let Err(e) = database::check_database_health(&pool_arc).await {
        warn!("Database health check failed: {}", e);
    }

    // 建立 repositories (使用 Arc 中的 Pool)
    let relation_repository = repositories::relation_repository::RelationRepository::new(pool_arc.as_ref().clone());
    let user_repository = repositories::user_repository::UserRepository::new(pool_arc.as_ref().clone());
    
    // 建立應用程式狀態
    let app_state = state::ConcreteAppState::new(&config.database_config.database_url, relation_repository, user_repository).await?;

    // 建立路由
    let app = create_router(app_state)
        .layer(create_cors_layer());

    // 啟動伺服器
    let addr: std::net::SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Failed to parse server address");
    
    info!("🚀 Ariadne backend server starting...");
    info!("📍 Listening on {}", addr);
    info!("🔗 Database connected successfully");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}