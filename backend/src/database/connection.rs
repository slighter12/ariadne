use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tracing::info;

/// 建立資料庫連接池
pub async fn create_connection_pool(database_url: &str, max_connections: u32) -> Result<Arc<Pool<Postgres>>, sqlx::Error> {
    info!("🔌 Creating database connection pool...");
    
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    info!("✅ Database connection pool created successfully");
    Ok(Arc::new(pool))
}