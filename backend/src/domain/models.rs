use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// 關聯狀態 ENUM
#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[sqlx(type_name = "relation_status")]
#[sqlx(rename_all = "lowercase")]
pub enum RelationStatus {
    Pending,
    Approved,
    Rejected,
    Deleted,  // 新增軟刪除狀態
}

// 關聯類型 ENUM
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "relation_type_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum RelationType {
    Reference,
    Remix,
    Annotation,
    Translation,
    Reaction,
}

// 使用者資料結構
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub google_id: Option<String>,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub reputation: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 關聯資料結構
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Relation {
    pub id: Uuid,
    pub source_video_id: String,
    pub source_start_time: i32,
    pub source_end_time: i32,
    pub target_video_id: String,
    pub target_start_time: i32,
    pub target_end_time: i32,
    pub relation_type: RelationType,
    pub user_id: Option<Uuid>,
    pub status: RelationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 審核記錄資料結構
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ApprovalRecord {
    pub id: Uuid,
    pub relation_id: Uuid,
    pub approver_id: Option<Uuid>,
    pub status: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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

// API 回應結構
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

// 關聯統計資料
#[derive(Debug, Serialize)]
pub struct RelationStats {
    pub total_relations: i64,
    pub pending_relations: i64,
    pub approved_relations: i64,
    pub rejected_relations: i64,
} 