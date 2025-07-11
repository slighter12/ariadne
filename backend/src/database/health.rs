use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::time::timeout;
use tracing::{info, debug, warn, error};

/// 驗證資料庫連接（帶有 timeout）
async fn check_database_connection(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // 使用 PostgreSQL 內建的連接檢查，設定 10 秒超時
    let query_result = timeout(
        Duration::from_secs(10),
        sqlx::query("SELECT pg_is_in_recovery()").fetch_one(pool)
    ).await;
    
    match query_result {
        Ok(Ok(_)) => {
            debug!("Database connection successful");
            Ok(())
        }
        Ok(Err(e)) => {
            error!("Database connection failed: {}", e);
            Err(e)
        }
        Err(_) => {
            error!("Database connection timeout after 10 seconds");
            Err(sqlx::Error::Configuration("Database connection timeout".into()))
        }
    }
}

/// 檢查資料庫健康狀態（帶有 retry 機制）
pub async fn check_database_health(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    check_database_health_with_retry(pool, 3, Duration::from_secs(5)).await
}

/// 檢查資料庫健康狀態（帶有 retry 機制）
pub async fn check_database_health_with_retry(
    pool: &Pool<Postgres>, 
    max_retries: u32, 
    retry_delay: Duration
) -> Result<(), sqlx::Error> {
    info!("🔍 Checking database health...");
    
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        info!("🔍 Database health check attempt {}/{}", attempts, max_retries);
        
        // 檢查連接池狀態
        let pool_size = pool.size();
        debug!("Connection pool size: {}", pool_size);
        
        // 檢查資料庫連接
        match check_database_connection(pool).await {
            Ok(()) => {
                // 檢查資料庫版本和狀態（帶有 timeout）
                let version_result = timeout(
                    Duration::from_secs(10),
                    sqlx::query_scalar::<_, String>("SELECT version()").fetch_one(pool)
                ).await;
                
                match version_result {
                    Ok(Ok(version)) => {
                        debug!("Database version: {}", version);
                        info!("✅ Database health check completed successfully");
                        info!("💡 Run 'make db-setup' to initialize database schema");
                        return Ok(());
                    }
                    Ok(Err(e)) => {
                        error!("Failed to get database version: {}", e);
                        if attempts >= max_retries {
                            return Err(e);
                        }
                    }
                    Err(_) => {
                        error!("Database version query timeout");
                        if attempts >= max_retries {
                            return Err(sqlx::Error::Configuration("Database version query timeout".into()));
                        }
                    }
                }
            }
            Err(e) => {
                if attempts >= max_retries {
                    error!("❌ Database health check failed after {} attempts: {}", max_retries, e);
                    return Err(e);
                }
                
                warn!("⚠️ Database health check attempt {} failed: {}. Retrying in {:?}...", attempts, e, retry_delay);
                tokio::time::sleep(retry_delay).await;
            }
        }
    }
} 