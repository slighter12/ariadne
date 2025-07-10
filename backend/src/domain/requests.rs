use serde::Deserialize;
use uuid::Uuid;
use crate::domain::entities::{RelationType, RelationStatus};

// 建立關聯時，前端傳來的 JSON payload
#[derive(Debug, Deserialize)]
pub struct CreateRelationPayload {
    pub source_video_id: String,
    pub source_start_time: i32,
    pub source_end_time: i32,
    pub target_video_id: String,
    pub target_start_time: i32,
    pub target_end_time: i32,
    pub relation_type: RelationType,
    pub user_id: Option<Uuid>,
}

// 查詢關聯時，前端傳來的 Query 參數
#[derive(Debug, Deserialize)]
pub struct GetRelationsParams {
    pub video_id: String,
    pub status: Option<RelationStatus>,
}

// 審核關聯時，前端傳來的 JSON payload
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApproveRelationPayload {
    pub status: RelationStatus, // "approved" or "rejected"
    pub reason: Option<String>,
    pub approver_id: Option<Uuid>,
}

// 使用者註冊/登入 payload
#[derive(Debug, Deserialize)]
pub struct UserAuthPayload {
    pub google_id: Option<String>,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
} 