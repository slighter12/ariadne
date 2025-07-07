use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// 建立 CORS 配置
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // 在生產環境中應指定前端來源
        .allow_methods(Any)
        .allow_headers(Any)
}

// 解析伺服器地址
#[allow(dead_code)]
pub fn parse_server_addr(host: &str, port: u16) -> SocketAddr {
    format!("{}:{}", host, port)
        .parse()
        .expect("Failed to parse server address")
}

// 驗證 YouTube URL
#[allow(dead_code)]
pub fn is_valid_youtube_url(url: &str) -> bool {
    url.contains("youtube.com/watch") || url.contains("youtu.be/")
}

// 從 YouTube URL 提取影片 ID
#[allow(dead_code)]
pub fn extract_youtube_video_id(url: &str) -> Option<String> {
    if url.contains("youtube.com/watch") {
        // 格式: https://www.youtube.com/watch?v=VIDEO_ID
        url.split("v=")
            .nth(1)
            .and_then(|s| s.split('&').next())
            .map(|s| s.to_string())
    } else if url.contains("youtu.be/") {
        // 格式: https://youtu.be/VIDEO_ID
        url.split("youtu.be/")
            .nth(1)
            .and_then(|s| s.split('?').next())
            .map(|s| s.to_string())
    } else {
        None
    }
}

// 驗證時間範圍
#[allow(dead_code)]
pub fn validate_time_range(start_time: i32, end_time: i32) -> Result<(), String> {
    if start_time < 0 || end_time < 0 {
        return Err("Time values must be non-negative".to_string());
    }
    
    if start_time >= end_time {
        return Err("Start time must be less than end time".to_string());
    }
    
    if end_time - start_time > 3600 { // 1 hour limit
        return Err("Time range cannot exceed 1 hour".to_string());
    }
    
    Ok(())
}

// 格式化時間為 MM:SS 格式
#[allow(dead_code)]
pub fn format_time(seconds: i32) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

// 從 MM:SS 格式解析時間
#[allow(dead_code)]
pub fn parse_time(time_str: &str) -> Result<i32, String> {
    let parts: Vec<&str> = time_str.split(':').collect();
    
    match parts.len() {
        2 => {
            let minutes: i32 = parts[0].parse().map_err(|_| "Invalid minutes")?;
            let seconds: i32 = parts[1].parse().map_err(|_| "Invalid seconds")?;
            
            if seconds >= 60 {
                return Err("Seconds must be less than 60".to_string());
            }
            
            Ok(minutes * 60 + seconds)
        }
        _ => Err("Time format must be MM:SS".to_string()),
    }
} 