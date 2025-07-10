use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use crate::domain::entities::Relation;
use crate::domain::requests::{CreateRelationPayload, GetRelationsParams};
use crate::domain::responses::ApiResponse;
use crate::delivery::middleware::response_handler::{with_status, with_created_status};
use crate::state::ConcreteAppState;
use tracing::{info, debug};

// 建立路由 - 使用泛型約束
pub fn create_router(app_state: ConcreteAppState) -> Router {
    Router::new()
        .route("/api/relations", get(get_relations).post(create_relation))
        .with_state(app_state)
}

// GET /api/relations?video_id=...
async fn get_relations(
    State(app_state): State<ConcreteAppState>,
    Query(params): Query<GetRelationsParams>,
) -> (StatusCode, Json<ApiResponse<Vec<Relation>>>) {
    debug!("Fetching relations for video_id: {}", params.video_id);
    
    let response = app_state.get_relation_service().get_relations(params).await;
    with_status(Json(response))
}

// POST /api/relations
async fn create_relation(
    State(app_state): State<ConcreteAppState>,
    Json(payload): Json<CreateRelationPayload>,
) -> (StatusCode, Json<ApiResponse<Relation>>) {
    debug!("Creating relation: source_video_id={}, target_video_id={}", 
           payload.source_video_id, payload.target_video_id);
    
    let response = app_state.get_relation_service().create_relation(payload).await;
    info!("Relation creation completed with code: {}", response.code);
    with_created_status(Json(response))
} 