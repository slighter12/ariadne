use uuid::Uuid;
use crate::domain::{User, UserAuthPayload, ApiResponse};
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
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<ApiResponse<User>, String> {
        match self.repository.find_by_id(id).await {
            Ok(Some(user)) => Ok(ApiResponse::success(user)),
            Ok(None) => Ok(ApiResponse::error("User not found".to_string())),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    // 業務邏輯：根據 email 獲取使用者
    #[allow(dead_code)]
    pub async fn get_user_by_email(&self, email: &str) -> Result<ApiResponse<User>, String> {
        match self.repository.find_by_email(email).await {
            Ok(Some(user)) => Ok(ApiResponse::success(user)),
            Ok(None) => Ok(ApiResponse::error("User not found".to_string())),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    // 業務邏輯：根據 Google ID 獲取使用者
    #[allow(dead_code)]
    pub async fn get_user_by_google_id(&self, google_id: &str) -> Result<ApiResponse<User>, String> {
        match self.repository.find_by_google_id(google_id).await {
            Ok(Some(user)) => Ok(ApiResponse::success(user)),
            Ok(None) => Ok(ApiResponse::error("User not found".to_string())),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    // 業務邏輯：創建使用者
    #[allow(dead_code)]
    pub async fn create_user(&self, payload: UserAuthPayload) -> Result<ApiResponse<User>, String> {
        match self.repository.create(&payload).await {
            Ok(user) => Ok(ApiResponse::success(user)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    // 業務邏輯：更新使用者
    #[allow(dead_code)]
    pub async fn update_user(&self, id: Uuid, payload: UserAuthPayload) -> Result<ApiResponse<User>, String> {
        match self.repository.update(id, &payload).await {
            Ok(user) => Ok(ApiResponse::success(user)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    // 業務邏輯：更新使用者聲望
    #[allow(dead_code)]
    pub async fn update_reputation(&self, id: Uuid, reputation: i32) -> Result<ApiResponse<User>, String> {
        match self.repository.update_reputation(id, reputation).await {
            Ok(user) => Ok(ApiResponse::success(user)),
            Err(e) => Err(format!("Database error: {}", e)),
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
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        let user = response.data.unwrap();
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
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.success); // 使用者不存在
        assert!(response.data.is_none());
        assert_eq!(response.message.unwrap(), "User not found");
    }

    #[tokio::test]
    async fn test_get_user_by_email() {
        // Arrange
        let mock_repo = MockUserRepository::new();
        let service = UserService::new(mock_repo);

        // Act
        let result = service.get_user_by_email("nonexistent@example.com").await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.success);
        assert!(response.data.is_none());
    }
} 