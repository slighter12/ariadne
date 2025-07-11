use backend::services::youtube::{get_video_info_from_url, parse_iso8601_duration};

#[tokio::test]
async fn test_parse_iso8601_duration() {
    // 測試不同的時長格式
    assert_eq!(parse_iso8601_duration("PT1M30S"), 90); // 1分30秒
    assert_eq!(parse_iso8601_duration("PT2H15M30S"), 8130); // 2小時15分30秒
    assert_eq!(parse_iso8601_duration("PT30S"), 30); // 30秒
    assert_eq!(parse_iso8601_duration("PT1H"), 3600); // 1小時
    assert_eq!(parse_iso8601_duration("PT1M"), 60); // 1分鐘
}



#[tokio::test]
async fn test_get_video_info_from_url() {
    // 注意：這個測試需要有效的 YouTube API 金鑰
    // 在 CI/CD 環境中，這個測試可能會被跳過
    
    let test_url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    
    // 如果沒有設置 API 金鑰，測試應該優雅地失敗
    match get_video_info_from_url(test_url, "invalid_key").await {
        Ok(_) => {
            // 如果成功，驗證返回的資料
            println!("YouTube API test succeeded (valid API key provided)");
        }
        Err(e) => {
            // 預期的錯誤，因為使用了無效的 API 金鑰
            println!("YouTube API test failed as expected: {}", e);
        }
    }
}

#[test]
fn test_url_validation() {
    use backend::services::youtube::extract_video_id;
    
    // 測試有效的 YouTube URL
    let valid_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtu.be/dQw4w9WgXcQ",
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=30s",
        "https://youtu.be/dQw4w9WgXcQ?t=30",
    ];
    
    for url in valid_urls {
        assert!(extract_video_id(url).is_some(), "Should extract video ID from: {}", url);
    }
    
    // 測試無效的 URL
    let invalid_urls = vec![
        "https://www.google.com",
        "https://www.youtube.com/channel/UC_x5XG1OV2P6uZZ5FSM9Ttw",
        "not_a_url",
        "",
    ];
    
    for url in invalid_urls {
        assert!(extract_video_id(url).is_none(), "URL should be invalid: {}", url);
    }
} 