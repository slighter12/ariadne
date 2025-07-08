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

// 解析影片資訊
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
  const durationElement = document.querySelector('.ytp-time-duration');
  
  const title = titleElement?.textContent?.trim() || '';
  const channel = channelElement?.textContent?.trim() || '';
  const durationText = durationElement?.textContent?.trim() || '0:00';
  
  // 解析時長
  const duration = parseDuration(durationText);
  
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