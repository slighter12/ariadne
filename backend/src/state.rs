use sqlx::{Pool, Postgres};
use std::sync::Arc;
use crate::traits::{RelationRepositoryTrait, UserRepositoryTrait};
use crate::services::{RelationService, UserService};
use crate::database::{create_connection_pool, DatabaseConfig};

// 應用程式狀態 - 使用泛型約束
#[derive(Clone)]
pub struct AppState<R, U>
where
    R: RelationRepositoryTrait + Clone,
    U: UserRepositoryTrait + Clone,
{
    #[allow(dead_code)]
    pub db_pool: Arc<Pool<Postgres>>,
    pub relation_service: RelationService<R>,
    #[allow(dead_code)]
    pub user_service: UserService<U>,
}

impl<R, U> AppState<R, U>
where
    R: RelationRepositoryTrait + Clone,
    U: UserRepositoryTrait + Clone,
{
    // 建立新的應用程式狀態
    pub async fn new(database_url: &str, relation_repo: R, user_repo: U) -> Result<Self, sqlx::Error> {
        let db_pool = create_connection_pool(database_url).await?;

        let relation_service = RelationService::new(relation_repo);
        let user_service = UserService::new(user_repo);

        Ok(Self {
            db_pool,
            relation_service,
            user_service,
        })
    }

    // 取得資料庫連接池的引用
    #[allow(dead_code)]
    pub fn get_db_pool(&self) -> &Pool<Postgres> {
        &self.db_pool
    }

    // 取得 relation service
    pub fn get_relation_service(&self) -> &RelationService<R> {
        &self.relation_service
    }

    // 取得 user service
    #[allow(dead_code)]
    pub fn get_user_service(&self) -> &UserService<U> {
        &self.user_service
    }
}

// 配置結構
#[derive(Clone)]
pub struct Config {
    pub database_config: DatabaseConfig,
    pub server_port: u16,
    pub server_host: String,
}

impl Config {
    // 從環境變數載入配置
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database_config: DatabaseConfig::from_env()?,
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()?,
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
        })
    }
}

// 具體的 AppState 類型別名，使用實際的 repository 實作
pub type ConcreteAppState = AppState<
    crate::repositories::relation_repository::RelationRepository,
    crate::repositories::user_repository::UserRepository
>; 