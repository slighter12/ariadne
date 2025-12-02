import type { VideoInfo } from '../types';

// YouTube 頁面類型
export const YouTubePageType = {
  VIDEO: 'video',
  CHANNEL: 'channel',
  SEARCH: 'search',
  HOME: 'home',
  UNKNOWN: 'unknown'
} as const;

export type YouTubePageType = typeof YouTubePageType[keyof typeof YouTubePageType];

// 取得 YouTube 頁面類型
export function getYouTubePageType(): YouTubePageType {
  const pathname = window.location.pathname;

  if (pathname.startsWith('/watch')) {
    return YouTubePageType.VIDEO;
  } else if (pathname.startsWith('/channel/') || pathname.startsWith('/c/') || pathname.startsWith('/@')) {
    return YouTubePageType.CHANNEL;
  } else if (pathname.startsWith('/results')) {
    return YouTubePageType.SEARCH;
  } else if (pathname === '/' || pathname === '/feed/') {
    return YouTubePageType.HOME;
  }

  return YouTubePageType.UNKNOWN;
}

// 檢查是否為影片頁面
export function isVideoPage(): boolean {
  return getYouTubePageType() === YouTubePageType.VIDEO;
}

// 從 URL 解析影片 ID
export function getVideoIdFromUrl(url: string): string | null {
  const urlObj = new URL(url);
  const videoId = urlObj.searchParams.get('v');
  return videoId;
}

// 從當前頁面取得影片 ID
export function getCurrentVideoId(): string | null {
  return getVideoIdFromUrl(window.location.href);
}

// 從頁面元素獲取影片時長（改進版本）
export function getVideoDurationFromPage(): number {
  // 方法1: 從播放器控制欄獲取
  const durationElement = document.querySelector('.ytp-time-duration');
  if (durationElement) {
    const durationText = durationElement.textContent?.trim();
    if (durationText) {
      const duration = parseDuration(durationText);
      if (duration > 0) {
        return duration;
      }
    }
  }

  // 方法2: 從影片資訊面板獲取
  const infoPanel = document.querySelector('ytd-video-primary-info-renderer');
  if (infoPanel) {
    const durationText = infoPanel.textContent?.match(/(\d{1,2}:)?(\d{1,2}):(\d{2})/)?.[0];
    if (durationText) {
      const duration = parseDuration(durationText);
      if (duration > 0) {
        return duration;
      }
    }
  }

  // 方法3: 從 meta 標籤獲取
  const metaDuration = document.querySelector('meta[property="video:duration"]');
  if (metaDuration) {
    const duration = parseInt(metaDuration.getAttribute('content') || '0');
    if (duration > 0) {
      return duration;
    }
  }

  // 方法4: 從 JSON-LD 結構化資料獲取
  const jsonLdScripts = document.querySelectorAll('script[type="application/ld+json"]');
  for (const script of jsonLdScripts) {
    try {
      const data = JSON.parse(script.textContent || '{}');
      if (data.duration) {
        const duration = parseISO8601Duration(data.duration);
        if (duration > 0) {
          return duration;
        }
      }
    } catch (e) {
      // 忽略 JSON 解析錯誤
    }
  }

  // 方法5: 從 YouTube 播放器 API 獲取
  const player = (window as any).yt?.player?.getPlayerByElement?.();
  if (player && typeof player.getDuration === 'function') {
    try {
      const duration = player.getDuration();
      if (duration && duration > 0) {
        return duration;
      }
    } catch (e) {
      // 忽略播放器 API 錯誤
    }
  }

  return 0;
}

// 解析 ISO 8601 時長格式 (PT1H2M3S)
export function parseISO8601Duration(duration: string): number {
  const match = duration.match(/PT(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?/);
  if (!match) return 0;

  const hours = parseInt(match[1] || '0');
  const minutes = parseInt(match[2] || '0');
  const seconds = parseInt(match[3] || '0');

  return hours * 3600 + minutes * 60 + seconds;
}

// 使用 YouTube Data API 獲取影片資訊（直接調用，需要 API 金鑰）
export async function getVideoInfoFromYouTubeAPI(videoId: string): Promise<VideoInfo | null> {
  try {
    // 注意：這需要 YouTube Data API 金鑰
    // 在實際使用中，應該通過後端 API 來調用，避免暴露 API 金鑰
    const response = await fetch(`https://www.googleapis.com/youtube/v3/videos?id=${videoId}&part=snippet,contentDetails&key=YOUR_API_KEY`);
    const data = await response.json();

    if (data.items && data.items.length > 0) {
      const item = data.items[0];
      const duration = parseISO8601Duration(item.contentDetails.duration);

      return {
        id: videoId,
        title: item.snippet.title,
        url: `https://www.youtube.com/watch?v=${videoId}`,
        channel: item.snippet.channelTitle,
        duration,
        currentTime: 0
      };
    }
  } catch (error) {
    console.warn('Failed to fetch video info from YouTube API:', error);
  }

  return null;
}

// 解析影片資訊（改進版本）
export function parseVideoInfo(): VideoInfo | null {
  if (!isVideoPage()) {
    return null;
  }

  const videoId = getCurrentVideoId();
  if (!videoId) {
    return null;
  }

  // 嘗試從頁面元素取得資訊
  const titleElement = document.querySelector('h1.ytd-video-primary-info-renderer');
  const channelElement = document.querySelector('ytd-channel-name a');

  const title = titleElement?.textContent?.trim() || '';
  const channel = channelElement?.textContent?.trim() || '';

  // 使用改進的時長獲取方法
  const duration = getVideoDurationFromPage();

  // 驗證影片時長是否符合 YouTube 限制
  if (duration > 0 && !validateYouTubeVideoDuration(duration)) {
    console.warn(`Video duration ${duration} seconds is outside YouTube limits (${MIN_RELATION_DURATION}-${MAX_RELATION_DURATION} seconds)`);
  }

  // 取得當前播放時間
  const currentTimeElement = document.querySelector('.ytp-time-current');
  const currentTimeText = currentTimeElement?.textContent?.trim() || '0:00';
  const currentTime = parseDuration(currentTimeText);

  return {
    id: videoId,
    title,
    url: window.location.href,
    channel,
    duration,
    currentTime
  };
}

// 使用後端 API 獲取影片資訊（備用方案，目前不使用）
// export async function getVideoInfoFromAPI(url: string): Promise<VideoInfo | null> {
//   // 目前不使用後端 API，直接從頁面解析
//   // 保留此函數作為未來擴展的備用方案
//   console.log('Backend API method not currently used, using page parsing instead');
//   return null;
// }

// 等待影片時長載入
export async function waitForVideoDuration(timeout = 10000): Promise<number> {
  const startTime = Date.now();

  while (Date.now() - startTime < timeout) {
    const duration = getVideoDurationFromPage();
    if (duration > 0) {
      return duration;
    }

    // 等待一小段時間再重試
    await new Promise(resolve => setTimeout(resolve, 500));
  }

  return 0;
}

// 解析時間格式 (MM:SS 或 HH:MM:SS)
export function parseDuration(timeString: string): number {
  const parts = timeString.split(':').map(Number);

  if (parts.length === 2) {
    // MM:SS 格式
    return parts[0] * 60 + parts[1];
  } else if (parts.length === 3) {
    // HH:MM:SS 格式
    return parts[0] * 3600 + parts[1] * 60 + parts[2];
  }

  return 0;
}

// 格式化時間為 MM:SS 格式
export function formatTime(seconds: number): string {
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = Math.floor(seconds % 60);
  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
}

// 格式化時間為 HH:MM:SS 格式
export function formatTimeLong(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`;
  }

  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
}

// 時間相關常數（秒）
export const MIN_RELATION_DURATION = 1;  // 最小關聯時長
export const MAX_RELATION_DURATION = 12 * 3600;  // 最大關聯時長（YouTube 最長影片 12 小時）

// 驗證時間範圍是否有效
export function validateTimeRange(startTime: number, endTime: number, videoDuration: number): boolean {
  return startTime >= 0 &&
         endTime > startTime &&
         endTime <= videoDuration &&
         videoDuration > 0;
}

// 驗證影片時長是否符合 YouTube 限制
export function validateYouTubeVideoDuration(duration: number): boolean {
  return duration >= MIN_RELATION_DURATION && duration <= MAX_RELATION_DURATION;
}

// 監聽 YouTube 頁面變化
export function watchYouTubePageChanges(callback: () => void): () => void {
  let currentUrl = window.location.href;

  const observer = new MutationObserver(() => {
    if (window.location.href !== currentUrl) {
      currentUrl = window.location.href;
      callback();
    }
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true
  });

  return () => observer.disconnect();
}

// 等待元素出現
export function waitForElement(selector: string, timeout = 5000): Promise<Element> {
  return new Promise((resolve, reject) => {
    const element = document.querySelector(selector);
    if (element) {
      resolve(element);
      return;
    }

    const observer = new MutationObserver(() => {
      const element = document.querySelector(selector);
      if (element) {
        observer.disconnect();
        resolve(element);
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true
    });

    setTimeout(() => {
      observer.disconnect();
      reject(new Error(`Element ${selector} not found within ${timeout}ms`));
    }, timeout);
  });
}