import axios from 'axios';
import type { Relation, RelationFormData, ApiResponse, RelationDisplay, VideoInfo, VideoInfoResponse } from '../types';
import { ERROR_CODES } from '../types';
import { transformFormDataToBackend, transformRelationToDisplay } from '../utils/transform';

// API 基礎 URL
const API_BASE_URL = import.meta.env.PROD
  ? 'http://localhost:8000'  // 生產環境使用實際的後端地址
  : 'http://localhost:8000'; // 開發環境使用本地後端

// 建立 axios 實例
const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 請求攔截器
api.interceptors.request.use(
  (config) => {
    // 可以在這裡加入認證 token
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// 回應攔截器
api.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    // 改善錯誤處理，提供更詳細的錯誤資訊
    if (error.response) {
      // 伺服器回應了錯誤狀態碼
      console.error('API Error Response:', {
        status: error.response.status,
        statusText: error.response.statusText,
        data: error.response.data,
        url: error.config?.url
      });
    } else if (error.request) {
      // 請求已發送但沒有收到回應
      console.error('API Error Request:', {
        message: 'No response received from server',
        url: error.config?.url,
        timeout: error.code === 'ECONNABORTED' ? 'Request timeout' : 'Network error'
      });
    } else {
      // 設定請求時發生錯誤
      console.error('API Error Config:', {
        message: error.message,
        url: error.config?.url
      });
    }
    return Promise.reject(error);
  }
);

// 檢查 API 回應是否成功
function isSuccessResponse<T>(response: ApiResponse<T>): boolean {
  return response.code === 200;
}

// 取得錯誤訊息
function getErrorMessage(response: ApiResponse<any>): string {
  if (response.message) {
    return response.message;
  }

  // 根據錯誤代碼提供預設訊息
  switch (response.code) {
    case ERROR_CODES.VALIDATION_ERROR:
      return '輸入資料驗證失敗';
    case ERROR_CODES.INVALID_TIME_RANGE:
      return '時間範圍無效';
    case ERROR_CODES.EMPTY_VIDEO_ID:
      return '影片 ID 不能為空';
    case ERROR_CODES.SAME_VIDEO_IDS:
      return '來源和目標影片不能相同';
    case ERROR_CODES.RELATION_NOT_FOUND:
      return '找不到指定的關聯';
    case ERROR_CODES.USER_NOT_FOUND:
      return '找不到指定的使用者';
    case ERROR_CODES.VIDEO_NOT_FOUND:
      return '找不到指定的影片';
    case ERROR_CODES.DUPLICATE_RELATION:
      return '關聯已存在';
    case ERROR_CODES.DATABASE_ERROR:
      return '資料庫錯誤';
    case ERROR_CODES.INTERNAL_SERVER_ERROR:
      return '伺服器內部錯誤';
    default:
      return '未知錯誤';
  }
}

// API 服務類別
export class ApiService {
  // 取得 API 版本資訊
  static async getVersion(): Promise<ApiResponse<any>> {
    try {
      const response = await api.get('/api/version');
      return response.data;
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '無法取得版本資訊',
        details: error.message
      };
    }
  }

  // 檢查 API 健康狀態
  static async healthCheck(): Promise<ApiResponse<any>> {
    try {
      const response = await api.get('/api/health');
      return response.data;
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '健康檢查失敗',
        details: error.message
      };
    }
  }

  // 提交關聯
  static async submitRelation(data: RelationFormData): Promise<ApiResponse<Relation>> {
    try {
      const backendData = transformFormDataToBackend(data);
      const response = await api.post('/api/relations', backendData);
      return response.data;
    } catch (error: any) {
      // 處理 axios 錯誤
      if (error.response?.data) {
        return error.response.data;
      }
      // 網路錯誤或其他錯誤
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '網路連線錯誤',
        details: error.message
      };
    }
  }

  // 取得影片關聯
  static async getVideoRelations(videoId: string): Promise<ApiResponse<RelationDisplay[]>> {
    try {
      const response = await api.get(`/api/relations?video_id=${videoId}`);
      const apiResponse: ApiResponse<Relation[]> = response.data;

      if (isSuccessResponse(apiResponse)) {
        // 處理 null 或空物件，確保回傳空陣列
        const relations = apiResponse.data || [];
        const transformedData = relations.map(transformRelationToDisplay);
        return {
          code: 200,
          data: transformedData
        };
      }

      // 錯誤回應
      return {
        code: apiResponse.code,
        message: getErrorMessage(apiResponse),
        details: apiResponse.details
      };
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '網路連線錯誤',
        details: error.message
      };
    }
  }

  // 取得影片資訊
  static async getVideoInfo(url: string): Promise<ApiResponse<VideoInfoResponse>> {
    try {
      const response = await api.get(`/api/video/info?url=${encodeURIComponent(url)}`);
      const apiResponse: ApiResponse<VideoInfoResponse> = response.data;

      if (isSuccessResponse(apiResponse)) {
        return apiResponse;
      }

      return {
        code: apiResponse.code,
        message: getErrorMessage(apiResponse),
        details: apiResponse.details
      };
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '無法獲取影片資訊',
        details: error.message
      };
    }
  }

  // 取得來源反向索引
  static async getSourceRelations(videoId: string): Promise<ApiResponse<Relation[]>> {
    try {
      const response = await api.get(`/api/sources/${videoId}`);
      const apiResponse: ApiResponse<Relation[]> = response.data;

      if (isSuccessResponse(apiResponse)) {
        // 處理 null 或空物件，確保回傳空陣列
        const relations = apiResponse.data || [];
        return {
          code: 200,
          data: relations
        };
      }

      return {
        code: apiResponse.code,
        message: getErrorMessage(apiResponse),
        details: apiResponse.details
      };
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '網路連線錯誤',
        details: error.message
      };
    }
  }

  // 審核關聯
  static async approveRelation(relationId: string, approved: boolean): Promise<ApiResponse<void>> {
    try {
      const endpoint = approved ? 'approve' : 'reject';
      const response = await api.post(`/api/relations/${relationId}/${endpoint}`);
      return response.data;
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '網路連線錯誤',
        details: error.message
      };
    }
  }

  // 取得待審核關聯
  static async getPendingRelations(): Promise<ApiResponse<Relation[]>> {
    try {
      const response = await api.get('/api/relations/pending');
      const apiResponse: ApiResponse<Relation[]> = response.data;

      if (isSuccessResponse(apiResponse)) {
        // 處理 null 或空物件，確保回傳空陣列
        const relations = apiResponse.data || [];
        return {
          code: 200,
          data: relations
        };
      }

      return {
        code: apiResponse.code,
        message: getErrorMessage(apiResponse),
        details: apiResponse.details
      };
    } catch (error: any) {
      if (error.response?.data) {
        return error.response.data;
      }
      return {
        code: ERROR_CODES.INTERNAL_SERVER_ERROR,
        message: '網路連線錯誤',
        details: error.message
      };
    }
  }
}

export default api;