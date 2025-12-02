use uuid::Uuid;
use crate::domain::entities::User;
use crate::domain::requests::UserAuthPayload;
use crate::domain::responses::ApiResponse;
use crate::domain::errors::ErrorCode;
use crate::traits::UserRepositoryTrait;

// Service Layer (Clean Architecture - Use Case Layer)
#[derive(Clone)]
#[allow(dead_code)]
pub struct UserService<R>
where
    R: UserRepositoryTrait,
{
    repository: R,
}

impl<R> UserService<R>
where
    R: UserRepositoryTrait,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // 業務邏輯：根據 ID 獲取使用者
    #[allow(dead_code)]
    pub async fn get_user_by_id(&self, id: Uuid) -> ApiResponse<User> {
        match self.repository.find_by_id(id).await {
            Ok(Some(user)) => ApiResponse::success(user),
            Ok(None) => ApiResponse::error(
                ErrorCode::UserNotFound.as_u16(),
                "User not found"
            ),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：根據 email 獲取使用者
    #[allow(dead_code)]
    pub async fn get_user_by_email(&self, email: &str) -> ApiResponse<User> {
        match self.repository.find_by_email(email).await {
            Ok(Some(user)) => ApiResponse::success(user),
            Ok(None) => ApiResponse::error(
                ErrorCode::UserNotFound.as_u16(),
                "User not found"
            ),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：根據 Google ID 獲取使用者
    #[allow(dead_code)]
    pub async fn get_user_by_google_id(&self, google_id: &str) -> ApiResponse<User> {
        match self.repository.find_by_google_id(google_id).await {
            Ok(Some(user)) => ApiResponse::success(user),
            Ok(None) => ApiResponse::error(
                ErrorCode::UserNotFound.as_u16(),
                "User not found"
            ),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：創建使用者
    #[allow(dead_code)]
    pub async fn create_user(&self, payload: UserAuthPayload) -> ApiResponse<User> {
        match self.repository.create(&payload).await {
            Ok(user) => ApiResponse::success(user),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：更新使用者
    #[allow(dead_code)]
    pub async fn update_user(&self, id: Uuid, payload: UserAuthPayload) -> ApiResponse<User> {
        match self.repository.update(id, &payload).await {
            Ok(user) => ApiResponse::success(user),
            Err(code) => ApiResponse::error(
                code,
                "Database error occurred"
            ),
        }
    }

    // 業務邏輯：更新使用者聲望
    #[allow(dead_code)]
    pub async fn update_reputation(&self, id: Uuid, reputation: i32) -> ApiResponse<User> {
        match self.repository.update_reputation(id, reputation).await {
            Ok(user) => ApiResponse::success(user),
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
    use crate::repositories::mock_repository::MockUserRepository;

    #[tokio::test]
    async fn test_create_user_success() {
        // Arrange
        let mock_repo = MockUserRepository::new();
        let service = UserService::new(mock_repo);

        let payload = UserAuthPayload {
            google_id: Some("test_google_id".to_string()),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };

        // Act
        let result = service.create_user(payload).await;

        // Assert
        assert_eq!(result.code, 200); // 成功碼
        let user = result.data.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.reputation, 0); // 新使用者預設聲望為 0
    }

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        // Arrange
        let mock_repo = MockUserRepository::new();
        let service = UserService::new(mock_repo);
        let user_id = Uuid::new_v4();

        // Act
        let result = service.get_user_by_id(user_id).await;

        // Assert
        assert!(result.code >= 400); // 錯誤碼
        assert!(result.data.is_none());
        assert_eq!(result.code, 40402); // UserNotFound error code
        assert_eq!(result.message.unwrap(), "User not found");
    }

    #[tokio::test]
    async fn test_get_user_by_email() {
        // Arrange
        let mock_repo = MockUserRepository::new();
        let service = UserService::new(mock_repo);

        // Act
        let result = service.get_user_by_email("nonexistent@example.com").await;

        // Assert
        assert!(result.code >= 400); // 錯誤碼
        assert!(result.data.is_none());
    }
}