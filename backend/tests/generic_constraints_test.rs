// 泛型約束測試範例
// 這個檔案展示了如何使用泛型約束進行測試

#[cfg(test)]
mod tests {
    use backend::{
        models::{CreateRelationPayload, GetRelationsParams, RelationType, UserAuthPayload},
        repositories::{MockRelationRepository, MockUserRepository},
        services::{RelationService, UserService},
        traits::{RelationRepositoryTrait, UserRepositoryTrait},
    };
    use uuid::Uuid;

    // 測試函數：展示泛型約束的優勢
    #[tokio::test]
    async fn test_relation_service_with_mock() {
        // 建立 mock repository
        let mock_repo = MockRelationRepository::new();
        let relation_service = RelationService::new(mock_repo);

        // 測試 payload
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

        // 測試創建關聯
        let result = relation_service.create_relation(payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_service_with_mock() {
        // 建立 mock repository
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);

        // 測試 payload
        let payload = UserAuthPayload {
            google_id: Some("test_google_id".to_string()),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };

        // 測試創建使用者
        let result = user_service.create_user(payload).await;
        assert!(result.is_ok());
    }

    // 展示泛型函數的優勢
    async fn test_any_relation_service<R>(service: &RelationService<R>) 
    where
        R: RelationRepositoryTrait,
    {
        let params = GetRelationsParams {
            video_id: "test_video".to_string(),
            status: None,
        };

        let result = service.get_relations(params).await;
        // 無論是 mock 還是真實實作，都可以用相同的程式碼測試
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generic_function() {
        // 使用 mock
        let mock_repo = MockRelationRepository::new();
        let mock_service = RelationService::new(mock_repo);
        test_any_relation_service(&mock_service).await;
    }
}