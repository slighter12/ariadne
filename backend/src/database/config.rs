/// 資料庫配置結構
#[derive(Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// 從環境變數載入資料庫配置
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
        })
    }

    /// 使用預設值建立配置
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
            max_connections: 5,
        }
    }
} 