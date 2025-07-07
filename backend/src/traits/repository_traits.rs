use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{Relation, CreateRelationPayload, GetRelationsParams, User, UserAuthPayload, RelationStatus};

// Repository Traits (Clean Architecture - Interface Layer)
// 這些 trait 定義了資料存取層的介面，支援泛型約束

#[async_trait]
pub trait RelationRepositoryTrait: Send + Sync {
    async fn find_by_video_id(&self, params: &GetRelationsParams) -> Result<Vec<Relation>, sqlx::Error>;
    async fn create(&self, payload: &CreateRelationPayload) -> Result<Relation, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Relation>, sqlx::Error>;
    async fn update_status(&self, id: Uuid, status: &RelationStatus) -> Result<Relation, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, payload: &UserAuthPayload) -> Result<User, sqlx::Error>;
    async fn update(&self, id: Uuid, payload: &UserAuthPayload) -> Result<User, sqlx::Error>;
    async fn update_reputation(&self, id: Uuid, reputation: i32) -> Result<User, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
} 