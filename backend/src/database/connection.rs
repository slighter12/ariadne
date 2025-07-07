use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tracing::info;

/// 建立資料庫連接池
pub async fn create_connection_pool(database_url: &str) -> Result<Arc<Pool<Postgres>>, sqlx::Error> {
    info!("🔌 Creating database connection pool...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("✅ Database connection pool created successfully");
    Ok(Arc::new(pool))
}

/// 取得資料庫連接池的引用
pub fn get_db_pool(pool: &Arc<Pool<Postgres>>) -> &Pool<Postgres> {
    pool.as_ref()
} 