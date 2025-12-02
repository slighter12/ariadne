use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use config::{Config, Environment, File};

// 配置結構 - 使用 serde 自動 deserialize
#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: Database,
    pub server: Server,
    pub logging: Logging,
    pub youtube_api_key: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout: std::time::Duration,
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: std::time::Duration,
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime: std::time::Duration,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_retry_delay")]
    pub retry_delay: std::time::Duration,
}

// 預設值函數
fn default_acquire_timeout() -> std::time::Duration { std::time::Duration::from_secs(30) }
fn default_idle_timeout() -> std::time::Duration { std::time::Duration::from_secs(300) }
fn default_max_lifetime() -> std::time::Duration { std::time::Duration::from_secs(1800) }
fn default_max_retries() -> u32 { 5 }
fn default_retry_delay() -> std::time::Duration { std::time::Duration::from_secs(3) }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_parsing() {
        // 測試 TOML 字串解析
        let toml_str = r#"
            [database]
            url = "postgres://test:test@localhost:5432/test"
            max_connections = 10
            acquire_timeout = 45
            idle_timeout = 600
            max_lifetime = 3600
            max_retries = 3
            retry_delay = 5

            [server]
            host = "127.0.0.1"
            port = 8080

            [logging]
            level = "debug"
            color = false
        "#;

        let config: AppConfig = toml::from_str(toml_str).unwrap();

        // 驗證時間解析
        assert_eq!(config.database.acquire_timeout, std::time::Duration::from_secs(45));
        assert_eq!(config.database.idle_timeout, std::time::Duration::from_secs(600));
        assert_eq!(config.database.max_lifetime, std::time::Duration::from_secs(3600));
        assert_eq!(config.database.retry_delay, std::time::Duration::from_secs(5));
        assert_eq!(config.database.max_retries, 3);
    }

    #[test]
    fn test_default_values() {
        // 測試預設值
        let toml_str = r#"
            [database]
            url = "postgres://test:test@localhost:5432/test"
            max_connections = 5

            [server]
            host = "127.0.0.1"
            port = 8080

            [logging]
            level = "info"
            color = true
        "#;

        let config: AppConfig = toml::from_str(toml_str).unwrap();

        // 驗證預設值
        assert_eq!(config.database.acquire_timeout, std::time::Duration::from_secs(30));
        assert_eq!(config.database.idle_timeout, std::time::Duration::from_secs(300));
        assert_eq!(config.database.max_lifetime, std::time::Duration::from_secs(1800));
        assert_eq!(config.database.retry_delay, std::time::Duration::from_secs(3));
        assert_eq!(config.database.max_retries, 5);
    }
}