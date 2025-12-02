use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::{Relation, User, RelationStatus};
use crate::domain::requests::{CreateRelationPayload, GetRelationsParams, UserAuthPayload};

// Repository Traits (Clean Architecture - Interface Layer)
// 這些 trait 定義了資料存取層的介面，支援泛型約束

#[async_trait]
pub trait RelationRepositoryTrait: Send + Sync {
    async fn find_by_video_id(&self, params: &GetRelationsParams) -> Result<Vec<Relation>, u16>;
    async fn create(&self, payload: &CreateRelationPayload) -> Result<Relation, u16>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Relation>, u16>;
    async fn update_status(&self, id: Uuid, status: &RelationStatus) -> Result<Relation, u16>;
}

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, u16>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, u16>;
    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, u16>;
    async fn create(&self, payload: &UserAuthPayload) -> Result<User, u16>;
    async fn update(&self, id: Uuid, payload: &UserAuthPayload) -> Result<User, u16>;
    async fn update_reputation(&self, id: Uuid, reputation: i32) -> Result<User, u16>;
}