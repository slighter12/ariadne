// 影片資訊類型
export interface VideoInfo {
  id: string;
  title: string;
  url: string;
  channel: string;
  duration: number;
  currentTime: number;
}

// 關聯類型 - 匹配後端 snake_case 格式
export interface Relation {
  id: string;
  source_video_id: string;
  source_start_time: number;
  source_end_time: number;
  target_video_id: string;
  target_start_time: number;
  target_end_time: number;
  relation_type: 'reference' | 'remix' | 'annotation' | 'translation' | 'reaction';
  user_id?: string;
  status: 'pending' | 'approved' | 'rejected';
  created_at: string;
  updated_at: string;
}

// 前端顯示用的關聯類型（包含 URL）
export interface RelationDisplay {
  id: string;
  sourceVideoId: string;
  sourceVideoUrl: string;
  sourceStartTime: number;
  sourceEndTime: number;
  targetVideoId: string;
  targetVideoUrl: string;
  targetStartTime: number;
  targetEndTime: number;
  status: 'pending' | 'approved' | 'rejected';
  createdAt: string;
  createdBy?: string;
}

// API 回應類型
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

// 表單類型
export interface RelationFormData {
  sourceVideoUrl: string;
  sourceStartTime: number;
  sourceEndTime: number;
  targetVideoUrl: string;
  targetStartTime: number;
  targetEndTime: number;
}

// YouTube 頁面狀態
export interface YouTubePageState {
  isVideoPage: boolean;
  videoInfo: VideoInfo | null;
  relations: Relation[];
  isLoading: boolean;
  error: string | null;
}

// 時間選擇器類型
export interface TimeRange {
  start: number;
  end: number;
}

// 通知類型
export interface Notification {
  id: string;
  type: 'success' | 'error' | 'info' | 'warning';
  message: string;
  duration?: number;
} 