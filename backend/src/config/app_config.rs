use serde::Deserialize;
use config::{Config, Environment, File};

// 配置結構 - 使用 serde 自動 deserialize
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: Database,
    pub server: Server,
    pub logging: Logging,
    pub youtube_api_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logging {
    pub level: String,
    pub color: bool,
}

impl AppConfig {
    // 從檔案和環境變數載入配置
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let settings = Config::builder()
            // 載入預設設定檔案
            .add_source(File::with_name("src/config/config"))
            // 環境變數覆蓋（使用 __ 作為分隔符）
            .add_source(Environment::default().separator("__"))
            .build()?;

        let config: AppConfig = settings.try_deserialize()?;
        Ok(config)
    }

    // 載入配置（別名方法）
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_env()
    }
}