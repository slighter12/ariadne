// 共用常數
pub const API_VERSION: &str = "1.0.0";
pub const DEFAULT_PAGE_SIZE: i32 = 20;
pub const MAX_PAGE_SIZE: i32 = 100;

// 時間相關常數（秒）
pub const MIN_RELATION_DURATION: i32 = 1;  // 最小關聯時長
pub const MAX_RELATION_DURATION: i32 = 3600;  // 最大關聯時長（1小時）

// 使用者聲望相關常數
pub const DEFAULT_USER_REPUTATION: i32 = 0;
pub const MIN_REPUTATION_FOR_APPROVAL: i32 = 10;

// 錯誤訊息常數 (保留向後相容性)
pub const ERROR_VIDEO_ID_EMPTY: &str = "Video ID cannot be empty";
pub const ERROR_INVALID_TIME_RANGE: &str = "Invalid time range";
pub const ERROR_SAME_VIDEO_IDS: &str = "Source and target video IDs cannot be the same";
pub const ERROR_RELATION_NOT_FOUND: &str = "Relation not found";
pub const ERROR_USER_NOT_FOUND: &str = "User not found";