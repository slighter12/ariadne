use sqlx::{Pool, Postgres};
use tracing::{info, debug};

/// 驗證資料庫連接
pub async fn check_database_connection(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // 使用 PostgreSQL 內建的連接檢查
    sqlx::query("SELECT pg_is_in_recovery()")
        .fetch_one(pool)
        .await?;
    
    debug!("Database connection successful");
    Ok(())
}

/// 檢查資料庫健康狀態
pub async fn check_database_health(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("🔍 Checking database health...");
    
    // 檢查連接池狀態
    let pool_size = pool.size();
    debug!("Connection pool size: {}", pool_size);
    
    // 檢查資料庫連接
    check_database_connection(pool).await?;
    
    // 檢查資料庫版本和狀態
    let version: String = sqlx::query_scalar("SELECT version()")
        .fetch_one(pool)
        .await?;
    debug!("Database version: {}", version);
    
    info!("✅ Database health check completed");
    info!("💡 Run 'make db-setup' to initialize database schema");
    
    Ok(())
} 