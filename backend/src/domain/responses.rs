use serde::Serialize;
use crate::delivery::middleware::response_handler::HasCode;

// API 回應結構
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

// 為 ApiResponse 實現 HasCode trait
impl<T> HasCode for ApiResponse<T> {
    fn get_code(&self) -> u16 {
        self.code
    }
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: None,
            details: None,
        }
    }

    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            data: None,
            message: Some(message.into()),
            details: None,
        }
    }

    pub fn error_with_details(code: u16, message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            code,
            data: None,
            message: Some(message.into()),
            details: Some(details.into()),
        }
    }
}

// 健康檢查回應結構
#[derive(Debug, Serialize)]
pub struct HealthInfo {
    pub status: String,
    pub message: String,
    pub timestamp: String,
}

// 根端點回應結構
#[derive(Debug, Serialize)]
pub struct RootInfo {
    pub message: String,
    pub version: String,
    pub endpoints: std::collections::HashMap<String, String>,
}