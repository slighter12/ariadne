import axios from 'axios';
import type { Relation, RelationFormData, ApiResponse, RelationDisplay } from '../types';
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

// API 服務類別
export class ApiService {
  // 提交關聯
  static async submitRelation(data: RelationFormData): Promise<ApiResponse<Relation>> {
    try {
      const backendData = transformFormDataToBackend(data);
      const response = await api.post('/api/relations', backendData);
      return response.data;
    } catch (error) {
      throw error;
    }
  }

  // 取得影片關聯
  static async getVideoRelations(videoId: string): Promise<ApiResponse<RelationDisplay[]>> {
    try {
      const response = await api.get(`/api/relations?video_id=${videoId}`);
      if (response.data.success && response.data.data) {
        const transformedData = response.data.data.map(transformRelationToDisplay);
        return {
          success: true,
          data: transformedData,
          error: response.data.error
        };
      }
      return response.data;
    } catch (error) {
      throw error;
    }
  }

  // 取得來源反向索引
  static async getSourceRelations(videoId: string): Promise<ApiResponse<Relation[]>> {
    try {
      const response = await api.get(`/api/sources/${videoId}`);
      return response.data;
    } catch (error) {
      throw error;
    }
  }

  // 審核關聯
  static async approveRelation(relationId: string, approved: boolean): Promise<ApiResponse<void>> {
    try {
      const endpoint = approved ? 'approve' : 'reject';
      const response = await api.post(`/api/relations/${relationId}/${endpoint}`);
      return response.data;
    } catch (error) {
      throw error;
    }
  }

  // 取得待審核關聯
  static async getPendingRelations(): Promise<ApiResponse<Relation[]>> {
    try {
      const response = await api.get('/api/relations/pending');
      return response.data;
    } catch (error) {
      throw error;
    }
  }
}

export default api; 