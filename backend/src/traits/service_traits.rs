use uuid::Uuid;
use crate::domain::{Relation, CreateRelationPayload, GetRelationsParams, ApiResponse, User, UserAuthPayload, RelationStatus};

// Service Traits (Clean Architecture - Use Case Layer)
#[allow(dead_code)]
pub trait RelationServiceTrait: Send + Sync {
    async fn get_relations(&self, params: GetRelationsParams) -> Result<ApiResponse<Vec<Relation>>, String>;
    async fn create_relation(&self, payload: CreateRelationPayload) -> Result<ApiResponse<Relation>, String>;
    async fn update_relation_status(&self, id: Uuid, status: RelationStatus) -> Result<ApiResponse<Relation>, String>;
}

#[allow(dead_code)]
pub trait UserServiceTrait: Send + Sync {
    async fn get_user(&self, id: Uuid) -> Result<ApiResponse<User>, String>;
    async fn create_user(&self, payload: UserAuthPayload) -> Result<ApiResponse<User>, String>;
    async fn update_user(&self, id: Uuid, payload: UserAuthPayload) -> Result<ApiResponse<User>, String>;
} 