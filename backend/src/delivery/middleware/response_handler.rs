use axum::{
    http::StatusCode,
    Json,
};
use crate::domain::errors::get_http_status_from_code;

// 為 ApiResponse 添加獲取 code 的 trait
pub trait HasCode {
    fn get_code(&self) -> u16;
}

// 簡化的輔助函數，直接返回 (StatusCode, Json<T>)
pub fn with_status<T>(response: Json<T>) -> (StatusCode, Json<T>)
where
    T: HasCode,
{
    let code = response.get_code();
    let http_status = get_http_status_from_code(code);
    (http_status, response)
}

// 為 POST 請求的輔助函數，成功時使用 CREATED 狀態碼
pub fn with_created_status<T>(response: Json<T>) -> (StatusCode, Json<T>)
where
    T: HasCode,
{
    let code = response.get_code();
    let http_status = if code >= 200 && code < 300 {
        StatusCode::CREATED
    } else {
        get_http_status_from_code(code)
    };
    (http_status, response)
}