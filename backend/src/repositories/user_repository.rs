use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::entities::User;
use crate::domain::requests::UserAuthPayload;
use crate::domain::errors::handle_sqlx_error;
use crate::traits::UserRepositoryTrait;

// Concrete Implementation (Clean Architecture - Interface Adapter Layer)
#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, u16> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, u16> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, u16> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE google_id = $1",
            google_id
        )
        .fetch_optional(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn create(&self, payload: &UserAuthPayload) -> Result<User, u16> {
        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (google_id, email, name, avatar_url)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            payload.google_id,
            payload.email,
            payload.name,
            payload.avatar_url,
        )
        .fetch_one(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn update(&self, id: Uuid, payload: &UserAuthPayload) -> Result<User, u16> {
        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users SET google_id = $1, email = $2, name = $3, avatar_url = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#,
            payload.google_id,
            payload.email,
            payload.name,
            payload.avatar_url,
            id
        )
        .fetch_one(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn update_reputation(&self, id: Uuid, reputation: i32) -> Result<User, u16> {
        let result = sqlx::query_as!(
            User,
            "UPDATE users SET reputation = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
            reputation,
            id
        )
        .fetch_one(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }
} 