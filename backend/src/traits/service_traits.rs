#![allow(async_fn_in_trait)]

use uuid::Uuid;
use crate::domain::entities::{Relation, User, RelationStatus};
use crate::domain::requests::{CreateRelationPayload, GetRelationsParams, UserAuthPayload};
use crate::domain::responses::ApiResponse;

// Service Traits (Clean Architecture - Use Case Layer)
#[allow(dead_code)]
pub trait RelationServiceTrait: Send + Sync {
    async fn get_relations(&self, params: GetRelationsParams) -> ApiResponse<Vec<Relation>>;
    async fn create_relation(&self, payload: CreateRelationPayload) -> ApiResponse<Relation>;
    async fn update_relation_status(&self, id: Uuid, status: RelationStatus) -> ApiResponse<Relation>;
}

#[allow(dead_code)]
pub trait UserServiceTrait: Send + Sync {
    async fn get_user(&self, id: Uuid) -> ApiResponse<User>;
    async fn create_user(&self, payload: UserAuthPayload) -> ApiResponse<User>;
    async fn update_user(&self, id: Uuid, payload: UserAuthPayload) -> ApiResponse<User>;
} 