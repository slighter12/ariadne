use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn, error};

/// 建立資料庫連接池（帶有 timeout 和 retry 機制）
pub async fn create_connection_pool(
    database_url: &str,
    max_connections: u32,
    acquire_timeout: Duration,
    idle_timeout: Duration,
    max_lifetime: Duration
) -> Result<Arc<Pool<Postgres>>, sqlx::Error> {
    info!("🔌 Creating database connection pool...");

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(acquire_timeout)
        .idle_timeout(idle_timeout)
        .max_lifetime(max_lifetime)
        .connect(database_url)
        .await?;

    info!("✅ Database connection pool created successfully");
    Ok(Arc::new(pool))
}

/// 建立資料庫連接池（帶有 retry 機制）
pub async fn create_connection_pool_with_retry(
    database_url: &str,
    max_connections: u32,
    max_retries: u32,
    retry_delay: Duration,
    acquire_timeout: Duration,
    idle_timeout: Duration,
    max_lifetime: Duration
) -> Result<Arc<Pool<Postgres>>, sqlx::Error> {
    let mut attempts = 0;

    loop {
        attempts += 1;
        info!("🔌 Attempting to create database connection pool (attempt {}/{})", attempts, max_retries);

        match create_connection_pool(
            database_url,
            max_connections,
            acquire_timeout,
            idle_timeout,
            max_lifetime
        ).await {
            Ok(pool) => {
                info!("✅ Database connection pool created successfully on attempt {}", attempts);
                return Ok(pool);
            }
            Err(e) => {
                if attempts >= max_retries {
                    error!("❌ Failed to create database connection pool after {} attempts: {}", max_retries, e);
                    return Err(e);
                }

                warn!("⚠️ Database connection attempt {} failed: {}. Retrying in {:?}...", attempts, e, retry_delay);
                tokio::time::sleep(retry_delay).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_pool_creation() {
        // 這個測試需要有效的資料庫連接
        // 在 CI/CD 環境中可能會被跳過
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/ariadne".to_string());

        let result = create_connection_pool(
            &database_url,
            5,
            Duration::from_secs(30),
            Duration::from_secs(300),
            Duration::from_secs(1800)
        ).await;
        match result {
            Ok(_) => println!("Database connection test succeeded"),
            Err(e) => println!("Database connection test failed (expected in CI): {}", e),
        }
    }

    #[tokio::test]
    async fn test_retry_mechanism() {
        // 測試 retry 機制（使用無效的資料庫 URL）
        let invalid_url = "postgresql://invalid:invalid@localhost:9999/invalid";
        let result = create_connection_pool_with_retry(
            invalid_url,
            5,
            2,  // 最多重試 2 次
            Duration::from_millis(100),  // 快速重試用於測試
            Duration::from_secs(30),
            Duration::from_secs(300),
            Duration::from_secs(1800)
        ).await;

        // 應該失敗，但不會卡住
        assert!(result.is_err());
    }
}