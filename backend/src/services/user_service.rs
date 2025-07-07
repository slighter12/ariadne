use uuid::Uuid;
use crate::models::{User, UserAuthPayload, ApiResponse};
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