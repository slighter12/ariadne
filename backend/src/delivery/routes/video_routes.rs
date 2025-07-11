use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::domain::responses::ApiResponse;
use crate::services::youtube::{get_video_info_from_url};
use crate::config::app_config::AppConfig;

#[derive(Debug, Deserialize)]
pub struct VideoInfoQuery {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct VideoInfoResponse {
    id: String,
    title: String,
    channel_title: String,
    duration: i32,
    published_at: Option<String>,
    description: Option<String>,
}

pub fn video_routes() -> Router {
    Router::new()
        .route("/info", get(get_video_info))
}

async fn get_video_info(
    Query(query): Query<VideoInfoQuery>,
) -> Result<Json<ApiResponse<VideoInfoResponse>>, StatusCode> {
    // 從配置中獲取 YouTube API 金鑰
    let config = AppConfig::load().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let api_key = config.youtube_api_key.as_deref()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    match get_video_info_from_url(&query.url, api_key).await {
        Ok(video_info) => {
            let response = VideoInfoResponse {
                id: video_info.id,
                title: video_info.title,
                channel_title: video_info.channel_title,
                duration: video_info.duration,
                published_at: video_info.published_at,
                description: video_info.description,
            };

            Ok(Json(ApiResponse {
                code: 200,
                data: Some(response),
                message: None,
                details: None,
            }))
        }
        Err(e) => {
            let error_message = format!("Failed to get video info: {}", e);
            Ok(Json(ApiResponse {
                code: 40001,
                data: None,
                message: Some(error_message),
                details: None,
            }))
        }
    }
} 