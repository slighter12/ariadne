use reqwest;
use serde::{Deserialize, Serialize};
use crate::domain::common::{MIN_RELATION_DURATION, MAX_RELATION_DURATION};

#[derive(Debug, Serialize, Deserialize)]
pub struct YouTubeVideoInfo {
    pub id: String,
    pub title: String,
    pub channel_title: String,
    pub duration: i32, // 以秒為單位
    pub published_at: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YouTubeApiResponse {
    items: Option<Vec<YouTubeApiItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YouTubeApiItem {
    id: String,
    snippet: YouTubeApiSnippet,
    content_details: YouTubeApiContentDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct YouTubeApiSnippet {
    title: String,
    channel_title: String,
    published_at: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct YouTubeApiContentDetails {
    duration: String, // ISO 8601 格式
}

// 從 YouTube URL 提取影片 ID
pub fn extract_video_id(url: &str) -> Option<String> {
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

// 解析 ISO 8601 時長格式 (PT1H2M3S)
pub fn parse_iso8601_duration(duration: &str) -> i32 {
    let mut total_seconds = 0;
    let mut current_number = 0;

    for ch in duration.chars() {
        match ch {
            'P' | 'T' => continue,
            'H' => {
                total_seconds += current_number * 3600;
                current_number = 0;
            }
            'M' => {
                total_seconds += current_number * 60;
                current_number = 0;
            }
            'S' => {
                total_seconds += current_number;
                current_number = 0;
            }
            '0'..='9' => {
                current_number = current_number * 10 + ch.to_digit(10).unwrap() as i32;
            }
            _ => continue,
        }
    }

    total_seconds
}

// 使用 YouTube Data API 獲取影片資訊
pub async fn get_video_info(video_id: &str, api_key: &str) -> Result<YouTubeVideoInfo, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,contentDetails&key={}",
        video_id, api_key
    );

    let response = reqwest::get(&url).await?;
    let data: YouTubeApiResponse = response.json().await?;

    if let Some(items) = data.items {
        if let Some(item) = items.first() {
            let duration = parse_iso8601_duration(&item.content_details.duration);

            // 驗證影片時長是否符合 YouTube 限制
            if duration < MIN_RELATION_DURATION || duration > MAX_RELATION_DURATION {
                return Err(format!("Video duration {} seconds is outside YouTube limits ({}-{} seconds)",
                    duration, MIN_RELATION_DURATION, MAX_RELATION_DURATION).into());
            }

            return Ok(YouTubeVideoInfo {
                id: item.id.clone(),
                title: item.snippet.title.clone(),
                channel_title: item.snippet.channel_title.clone(),
                duration,
                published_at: Some(item.snippet.published_at.clone()),
                description: Some(item.snippet.description.clone()),
            });
        }
    }

    Err("Video not found".into())
}

// 驗證時間範圍
fn validate_time_range(start_time: i32, end_time: i32, video_duration: Option<i32>) -> Result<(), String> {
    if start_time < 0 || end_time < 0 {
        return Err("Time values must be non-negative".to_string());
    }

    if start_time >= end_time {
        return Err("Start time must be less than end time".to_string());
    }

    // 如果提供了影片時長，檢查是否超出範圍
    if let Some(duration) = video_duration {
        if end_time > duration {
            return Err(format!("End time ({}) exceeds video duration ({})", end_time, duration));
        }
    }

    if end_time - start_time > 3600 { // 1 hour limit
        return Err("Time range cannot exceed 1 hour".to_string());
    }

    Ok(())
}

// 格式化時間為 HH:MM:SS 格式
fn format_time(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let remaining_seconds = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, remaining_seconds)
    } else {
        format!("{:02}:{:02}", minutes, remaining_seconds)
    }
}

// 從 URL 獲取影片資訊（包含時長驗證）
pub async fn get_video_info_from_url(url: &str, api_key: &str) -> Result<YouTubeVideoInfo, Box<dyn std::error::Error>> {
    let video_id = extract_video_id(url)
        .ok_or("Could not extract video ID from URL")?;

    get_video_info(&video_id, api_key).await
}

#[cfg(test)]
mod tests {
    use super::*;

    mod video_id_extraction {
        use super::*;

        #[test]
        fn test_standard_youtube_url() {
            assert_eq!(
                extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
                Some("dQw4w9WgXcQ".to_string())
            );
        }

        #[test]
        fn test_short_youtube_url() {
            assert_eq!(
                extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
                Some("dQw4w9WgXcQ".to_string())
            );
        }

        #[test]
        fn test_url_with_playlist_should_extract_video_id() {
            assert_eq!(
                extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PL1234567890"),
                Some("dQw4w9WgXcQ".to_string())
            );
        }

        #[test]
        fn test_invalid_url() {
            assert_eq!(extract_video_id("https://example.com"), None);
        }
    }

    mod duration_parsing {
        use super::*;

        #[test]
        fn test_parse_iso8601_duration() {
            assert_eq!(parse_iso8601_duration("PT1H2M3S"), 3723);
            assert_eq!(parse_iso8601_duration("PT1M30S"), 90);
            assert_eq!(parse_iso8601_duration("PT1S"), 1);
        }

        #[test]
        fn test_parse_complex_duration() {
            assert_eq!(parse_iso8601_duration("PT2H15M30S"), 8130);
            assert_eq!(parse_iso8601_duration("PT30S"), 30);
            assert_eq!(parse_iso8601_duration("PT1H"), 3600);
            assert_eq!(parse_iso8601_duration("PT1M"), 60);
        }
    }

    mod time_validation {
        use super::*;

        #[test]
        fn test_valid_time_ranges() {
            assert!(validate_time_range(0, 30, Some(60)).is_ok());
            assert!(validate_time_range(10, 50, Some(100)).is_ok());
        }

        #[test]
        fn test_invalid_time_ranges() {
            // 負數時間
            assert!(validate_time_range(-1, 30, Some(60)).is_err());

            // 開始時間大於結束時間
            assert!(validate_time_range(30, 10, Some(60)).is_err());

            // 超出影片時長
            assert!(validate_time_range(0, 70, Some(60)).is_err());

            // 超過1小時限制
            assert!(validate_time_range(0, 3601, None).is_err());
        }
    }

    mod time_formatting {
        use super::*;

        #[test]
        fn test_format_time() {
            assert_eq!(format_time(60), "01:00");
            assert_eq!(format_time(120), "02:00");
            assert_eq!(format_time(121), "02:01");
            assert_eq!(format_time(3600), "01:00:00");
        }

        #[test]
        fn test_format_long_duration() {
            assert_eq!(format_time(3661), "01:01:01"); // 1小時1分1秒
            assert_eq!(format_time(7325), "02:02:05"); // 2小時2分5秒
        }
    }

    mod youtube_limits {
        use super::*;

        #[test]
        fn test_youtube_duration_limits() {
            // 測試 YouTube 影片時長限制
            assert!(MIN_RELATION_DURATION == 1);
            assert!(MAX_RELATION_DURATION == 12 * 3600); // 12 小時

            // 測試有效的時長
            assert!(1 >= MIN_RELATION_DURATION && 1 <= MAX_RELATION_DURATION);
            assert!(3600 >= MIN_RELATION_DURATION && 3600 <= MAX_RELATION_DURATION); // 1 小時
            assert!(12 * 3600 >= MIN_RELATION_DURATION && 12 * 3600 <= MAX_RELATION_DURATION); // 12 小時

            // 測試無效的時長
            assert!(0 < MIN_RELATION_DURATION); // 0 秒無效
            assert!(13 * 3600 > MAX_RELATION_DURATION); // 13 小時無效
        }
    }

    mod api_integration {
        use super::*;

        #[tokio::test]
        async fn test_get_video_info_from_url() {
            let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
            if let Ok(api_key) = std::env::var("YOUTUBE_API_KEY") {
                let result = get_video_info_from_url(url, &api_key).await;
                assert!(result.is_ok());
            } else {
                println!("Skipping test: YOUTUBE_API_KEY not set");
            }
        }

        #[tokio::test]
        async fn test_get_video_info() {
            let video_id = "dQw4w9WgXcQ";
            if let Ok(api_key) = std::env::var("YOUTUBE_API_KEY") {
                let result = get_video_info(video_id, &api_key).await;
                assert!(result.is_ok());
            } else {
                println!("Skipping test: YOUTUBE_API_KEY not set");
            }
        }
    }
}