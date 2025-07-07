use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use crate::models::{Relation, CreateRelationPayload, GetRelationsParams, ApiResponse};
use crate::state::ConcreteAppState;
use tracing::{error, info, debug, warn};

// 建立路由 - 使用泛型約束
pub fn create_router(app_state: ConcreteAppState) -> Router {
    Router::new()
        .route("/api/relations", get(get_relations).post(create_relation))
        .with_state(app_state)
}

// GET /api/relations?video_id=...
pub async fn get_relations(
    State(app_state): State<ConcreteAppState>,
    Query(params): Query<GetRelationsParams>,
) -> Result<Json<ApiResponse<Vec<Relation>>>, StatusCode> {
    debug!("Fetching relations for video_id: {}", params.video_id);
    
    let result = app_state.get_relation_service().get_relations(params).await;
    match result {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to fetch relations: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// POST /api/relations
pub async fn create_relation(
    State(app_state): State<ConcreteAppState>,
    Json(payload): Json<CreateRelationPayload>,
) -> Result<(StatusCode, Json<ApiResponse<Relation>>), StatusCode> {
    debug!("Creating relation: source_video_id={}, target_video_id={}", 
           payload.source_video_id, payload.target_video_id);
    
    let result = app_state.get_relation_service().create_relation(payload).await;
    match result {
        Ok(response) => {
            if response.success {
                info!("Successfully created relation");
                Ok((StatusCode::CREATED, Json(response)))
            } else {
                warn!("Failed to create relation: {:?}", response.message);
                Ok((StatusCode::BAD_REQUEST, Json(response)))
            }
        }
        Err(e) => {
            error!("Failed to create relation: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
} 