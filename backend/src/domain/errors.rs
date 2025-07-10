use axum::http::StatusCode;

// Error codes enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Validation errors (400)
    ValidationError,
    InvalidTimeRange,
    EmptyVideoId,
    SameVideoIds,
    
    // Not found errors (404)
    RelationNotFound,
    UserNotFound,
    VideoNotFound,
    
    // Conflict errors (409)
    DuplicateRelation,
    
    // Server errors (500)
    DatabaseError,
    InternalServerError,
    
    // Custom error codes
    Custom(u16),
}

impl ErrorCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            ErrorCode::ValidationError => 40001,
            ErrorCode::InvalidTimeRange => 40002,
            ErrorCode::EmptyVideoId => 40003,
            ErrorCode::SameVideoIds => 40004,
            ErrorCode::RelationNotFound => 40401,
            ErrorCode::UserNotFound => 40402,
            ErrorCode::VideoNotFound => 40403,
            ErrorCode::DuplicateRelation => 40901,
            ErrorCode::DatabaseError => 50001,
            ErrorCode::InternalServerError => 50000,
            ErrorCode::Custom(code) => *code,
        }
    }
}

// 輔助函數：根據錯誤代碼獲取 HTTP 狀態碼
pub fn get_http_status_from_code(code: u16) -> StatusCode {
    if code >= 200 && code < 300 {
        StatusCode::OK
    } else if code >= 400 && code < 500 {
        StatusCode::BAD_REQUEST
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

// 將 sqlx 錯誤轉換為 ApiResponse 的輔助函數
pub fn handle_sqlx_error(error: sqlx::Error) -> u16 {
    match error {
        sqlx::Error::RowNotFound => ErrorCode::RelationNotFound.as_u16(),
        sqlx::Error::Database(db_error) => {
            if let Some(code) = db_error.code() {
                match code.as_ref() {
                    "23505" => ErrorCode::DuplicateRelation.as_u16(), // Unique violation
                    _ => ErrorCode::DatabaseError.as_u16(),
                }
            } else {
                ErrorCode::DatabaseError.as_u16()
            }
        }
        _ => ErrorCode::DatabaseError.as_u16(),
    }
} 