// YouTube 相關服務

// 驗證 YouTube URL
fn is_valid_youtube_url(url: &str) -> bool {
    url.contains("youtube.com/watch") || url.contains("youtu.be/")
}

// 從 YouTube URL 提取影片 ID
fn extract_video_id(url: &str) -> Option<String> {
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
fn validate_time_range(start_time: i32, end_time: i32) -> Result<(), String> {
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
fn format_time(seconds: i32) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
} 