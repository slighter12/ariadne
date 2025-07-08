import axios from 'axios';
import type { Relation, RelationFormData, ApiResponse } from '../types';

// API 基礎 URL
const API_BASE_URL = import.meta.env.PROD 
  ? 'http://localhost:8080'  // 生產環境使用實際的後端地址
  : 'http://localhost:8080'; // 開發環境使用本地後端

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
    console.error('API Error:', error);
    return Promise.reject(error);
  }
);

// API 服務類別
export class ApiService {
  // 提交關聯
  static async submitRelation(data: RelationFormData): Promise<ApiResponse<Relation>> {
    try {
      const response = await api.post('/api/relations', data);
      return response.data;
    } catch (error) {
      throw error;
    }
  }

  // 取得影片關聯
  static async getVideoRelations(videoId: string): Promise<ApiResponse<Relation[]>> {
    try {
      const response = await api.get(`/api/relations?video_id=${videoId}`);
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