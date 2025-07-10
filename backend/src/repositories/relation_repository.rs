use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::entities::{Relation, RelationStatus};
use crate::domain::requests::{CreateRelationPayload, GetRelationsParams};
use crate::domain::errors::handle_sqlx_error;
use crate::traits::RelationRepositoryTrait;

// Concrete Implementation (Clean Architecture - Interface Adapter Layer)
#[derive(Clone)]
pub struct RelationRepository {
    pool: Pool<Postgres>,
}

impl RelationRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RelationRepositoryTrait for RelationRepository {
    async fn find_by_video_id(&self, params: &GetRelationsParams) -> Result<Vec<Relation>, u16> {
        let result = if let Some(status) = &params.status {
            sqlx::query_as::<_, Relation>(
                "SELECT * FROM relations WHERE source_video_id = $1 AND status = $2 ORDER BY created_at DESC"
            )
            .bind(&params.video_id)
            .bind(status)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, Relation>(
                "SELECT * FROM relations WHERE source_video_id = $1 ORDER BY created_at DESC"
            )
            .bind(&params.video_id)
            .fetch_all(&self.pool)
            .await
        };

        result.map_err(handle_sqlx_error)
    }

    async fn create(&self, payload: &CreateRelationPayload) -> Result<Relation, u16> {
        let result = sqlx::query_as::<_, Relation>(
            r#"
            INSERT INTO relations (source_video_id, source_start_time, source_end_time, target_video_id, target_start_time, target_end_time, relation_type, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(&payload.source_video_id)
        .bind(payload.source_start_time)
        .bind(payload.source_end_time)
        .bind(&payload.target_video_id)
        .bind(payload.target_start_time)
        .bind(payload.target_end_time)
        .bind(&payload.relation_type)
        .bind(&payload.user_id)
        .fetch_one(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Relation>, u16> {
        let result = sqlx::query_as::<_, Relation>(
            "SELECT * FROM relations WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }

    async fn update_status(&self, id: Uuid, status: &RelationStatus) -> Result<Relation, u16> {
        let result = sqlx::query_as::<_, Relation>(
            "UPDATE relations SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        result.map_err(handle_sqlx_error)
    }
} 