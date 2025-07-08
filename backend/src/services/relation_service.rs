use uuid::Uuid;
use crate::domain::{Relation, CreateRelationPayload, GetRelationsParams, ApiResponse, RelationStatus};
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
    pub async fn get_relations(&self, params: GetRelationsParams) -> Result<ApiResponse<Vec<Relation>>, String> {
        // 驗證輸入參數
        if params.video_id.trim().is_empty() {
            return Ok(ApiResponse::error("Video ID cannot be empty".to_string()));
        }

        // 調用 repository
        let relations = self.repository
            .find_by_video_id(&params)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(ApiResponse::success(relations))
    }

    // 業務邏輯：創建關聯
    pub async fn create_relation(&self, payload: CreateRelationPayload) -> Result<ApiResponse<Relation>, String> {
        // 驗證時間範圍
        if payload.source_start_time >= payload.source_end_time {
            return Ok(ApiResponse::error("Source start time must be before end time".to_string()));
        }

        if payload.target_start_time >= payload.target_end_time {
            return Ok(ApiResponse::error("Target start time must be before end time".to_string()));
        }

        // 驗證影片 ID 不能相同
        if payload.source_video_id == payload.target_video_id {
            return Ok(ApiResponse::error("Source and target video IDs cannot be the same".to_string()));
        }

        // 調用 repository
        let relation = self.repository
            .create(&payload)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(ApiResponse::success(relation))
    }

    // 業務邏輯：更新關聯狀態
    pub async fn update_relation_status(&self, id: Uuid, status: RelationStatus) -> Result<ApiResponse<Relation>, String> {
        // 檢查關聯是否存在
        let existing = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing.is_none() {
            return Ok(ApiResponse::error("Relation not found".to_string()));
        }

        // 調用 repository
        let relation = self.repository
            .update_status(id, &status)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(ApiResponse::success(relation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mock_repository::MockRelationRepository;
    use crate::domain::{RelationStatus, RelationType};

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
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        let relation = response.data.unwrap();
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
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        let relations = response.data.unwrap();
        assert!(relations.is_empty()); // Mock 預設為空
    }

    #[tokio::test]
    async fn test_update_relation_status() {
        // Arrange
        let mock_repo = MockRelationRepository::new();
        let service = RelationService::new(mock_repo);
        let relation_id = Uuid::new_v4();

        // Act
        let result = service.update_relation_status(relation_id, RelationStatus::Approved).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        let relation = response.data.unwrap();
        assert_eq!(relation.status, RelationStatus::Approved);
    }
} 