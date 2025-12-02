use uuid::Uuid;
use crate::domain::entities::{Relation, RelationStatus};
use crate::domain::requests::{CreateRelationPayload, GetRelationsParams};
use crate::domain::responses::ApiResponse;
use crate::domain::errors::ErrorCode;
use crate::traits::RelationRepositoryTrait;

// Service Layer (Clean Architecture - Use Case Layer)
#[derive(Clone)]
pub struct RelationService<R>
where
    R: RelationRepositoryTrait,
{
    repository: R,
}

#[allow(dead_code)]
impl<R> RelationService<R>
where
    R: RelationRepositoryTrait,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // 業務邏輯：獲取影片關聯
    pub async fn get_relations(&self, params: GetRelationsParams) -> ApiResponse<Vec<Relation>> {
        // 驗證輸入參數
        if params.video_id.trim().is_empty() {
            return ApiResponse::error(
                ErrorCode::EmptyVideoId.as_u16(),
                "Video ID cannot be empty"
            );
        }

        // 調用 repository
        match self.repository.find_by_video_id(&params).await {
            Ok(relations) => ApiResponse::success(relations),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：創建關聯
    pub async fn create_relation(&self, payload: CreateRelationPayload) -> ApiResponse<Relation> {
        // 驗證時間範圍
        if payload.source_start_time >= payload.source_end_time {
            return ApiResponse::error_with_details(
                ErrorCode::InvalidTimeRange.as_u16(),
                "Invalid time range",
                "Source start time must be before end time"
            );
        }

        if payload.target_start_time >= payload.target_end_time {
            return ApiResponse::error_with_details(
                ErrorCode::InvalidTimeRange.as_u16(),
                "Invalid time range",
                "Target start time must be before end time"
            );
        }

        // 驗證影片 ID 不能相同
        if payload.source_video_id == payload.target_video_id {
            return ApiResponse::error(
                ErrorCode::SameVideoIds.as_u16(),
                "Source and target video IDs cannot be the same"
            );
        }

        // 調用 repository
        match self.repository.create(&payload).await {
            Ok(relation) => ApiResponse::success(relation),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：更新關聯狀態
    pub async fn update_relation_status(&self, id: Uuid, status: RelationStatus) -> ApiResponse<Relation> {
        // 檢查關聯是否存在
        match self.repository.find_by_id(id).await {
            Ok(existing) => {
                if existing.is_none() {
                    return ApiResponse::error(
                        ErrorCode::RelationNotFound.as_u16(),
                        "Relation not found"
                    );
                }
            }
            Err(code) => {
                return ApiResponse::error(
                    code,
                    "Database error occurred"
                );
            }
        }

        // 調用 repository
        match self.repository.update_status(id, &status).await {
            Ok(relation) => ApiResponse::success(relation),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mock_repository::MockRelationRepository;
    use crate::domain::entities::{RelationStatus, RelationType};

    #[tokio::test]
    async fn test_create_relation_success() {
        // Arrange
        let mock_repo = MockRelationRepository::new();
        let service = RelationService::new(mock_repo);

        let payload = CreateRelationPayload {
            source_video_id: "test_video_1".to_string(),
            source_start_time: 0,
            source_end_time: 10,
            target_video_id: "test_video_2".to_string(),
            target_start_time: 0,
            target_end_time: 10,
            relation_type: RelationType::Reference,
            user_id: Some(Uuid::new_v4()),
        };

        // Act
        let result = service.create_relation(payload).await;

        // Assert
        assert_eq!(result.code, 200);
        let relation = result.data.unwrap();
        assert_eq!(relation.source_video_id, "test_video_1");
        assert_eq!(relation.target_video_id, "test_video_2");
        assert_eq!(relation.status, RelationStatus::Pending);
    }

    #[tokio::test]
    async fn test_get_relations_with_mock() {
        // Arrange
        let mock_repo = MockRelationRepository::new();
        let service = RelationService::new(mock_repo);

        let params = GetRelationsParams {
            video_id: "test_video".to_string(),
            status: None,
        };

        // Act
        let result = service.get_relations(params).await;

        // Assert
        assert_eq!(result.code, 200);
        let relations = result.data.unwrap();
        assert!(relations.is_empty()); // Mock 預設為空
    }

    #[tokio::test]
    async fn test_update_relation_status() {
        // Arrange
        let mock_repo = MockRelationRepository::new();
        let service = RelationService::new(mock_repo);

        // 先建立一個關聯
        let payload = CreateRelationPayload {
            source_video_id: "test_video_1".to_string(),
            source_start_time: 0,
            source_end_time: 10,
            target_video_id: "test_video_2".to_string(),
            target_start_time: 0,
            target_end_time: 10,
            relation_type: RelationType::Reference,
            user_id: Some(Uuid::new_v4()),
        };
        let created_relation = service.create_relation(payload).await.data.unwrap();
        let relation_id = created_relation.id;

        // Act
        let result = service.update_relation_status(relation_id, RelationStatus::Approved).await;

        // Assert
        assert_eq!(result.code, 200);
        let relation = result.data.unwrap();
        assert_eq!(relation.status, RelationStatus::Approved);

        // 驗證狀態是否真的被更新了 (再次查詢)
        let updated_relation = service.repository.find_by_id(relation_id).await.unwrap().unwrap();
        assert_eq!(updated_relation.status, RelationStatus::Approved);
    }
}