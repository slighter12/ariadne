import React, { useState, useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import type { VideoInfo, RelationDisplay, RelationFormData } from '../types';
import { ERROR_CODES } from '../types';
import { RelationForm } from './RelationForm';
import { Notification } from './Notification';
import { ApiService } from '../services/api';
import { isVideoPage, parseVideoInfo, watchYouTubePageChanges } from '../utils/youtube';

interface ContentScriptState {
  isVisible: boolean;
  videoInfo: VideoInfo | null;
  relations: RelationDisplay[];
  isLoading: boolean;
  error: string | null;
  notifications: Array<{ id: string; type: 'success' | 'error' | 'info' | 'warning'; message: string; duration?: number }>;
}

const ContentScript: React.FC = () => {
  const [state, setState] = useState<ContentScriptState>({
    isVisible: false,
    videoInfo: null,
    relations: [],
    isLoading: false,
    error: null,
    notifications: []
  });

  // 檢查是否為影片頁面並載入資料
  useEffect(() => {
    const loadVideoData = async () => {
      if (!isVideoPage()) {
        setState(prev => ({ ...prev, isVisible: false }));
        return;
      }

      const videoInfo = parseVideoInfo();
      if (!videoInfo) {
        setState(prev => ({ ...prev, isVisible: false }));
        return;
      }

      setState(prev => ({ 
        ...prev, 
        isVisible: true, 
        videoInfo,
        isLoading: true 
      }));

      try {
        // 載入關聯資料
        const response = await ApiService.getVideoRelations(videoInfo.id);
        if (response.code === ERROR_CODES.SUCCESS) {
          // 處理 null 或空物件，確保使用空陣列
          setState(prev => ({ 
            ...prev, 
            relations: response.data || [],
            isLoading: false,
            error: null
          }));
        } else {
          // API 失敗
          setState(prev => ({ 
            ...prev, 
            relations: [],
            isLoading: false,
            error: response.message || '載入關聯資料失敗'
          }));
        }
      } catch (error) {
        console.error('Failed to load relations:', error);
        // 改善錯誤訊息
        let errorMessage = '載入關聯資料失敗';
        if (error && typeof error === 'object' && 'response' in error) {
          const response = (error as any).response;
          if (response?.status === 404) {
            errorMessage = '此影片尚未有關聯資料';
          } else if (response?.status === 500) {
            errorMessage = '伺服器錯誤，請稍後再試';
          } else if (response?.status === 0) {
            errorMessage = '無法連接到伺服器，請確認後端服務是否運行';
          }
        }
        setState(prev => ({ 
          ...prev, 
          error: errorMessage,
          isLoading: false 
        }));
      }
    };

    loadVideoData();

    // 監聽頁面變化
    const unsubscribe = watchYouTubePageChanges(loadVideoData);
    return unsubscribe;
  }, []);

  // 處理關聯提交
  const handleSubmitRelation = async (formData: RelationFormData) => {
    setState(prev => ({ ...prev, isLoading: true }));

    try {
      const response = await ApiService.submitRelation(formData);
      if (response.code === ERROR_CODES.SUCCESS) {
        addNotification('success', '關聯提交成功！', 3000);
        // 重新載入關聯資料
        if (state.videoInfo) {
          const relationsResponse = await ApiService.getVideoRelations(state.videoInfo.id);
          if (relationsResponse.code === ERROR_CODES.SUCCESS) {
            // 處理 null 或空物件，確保使用空陣列
            setState(prev => ({ 
              ...prev, 
              relations: relationsResponse.data || [],
              isLoading: false,
              error: null
            }));
          } else {
            setState(prev => ({ 
              ...prev, 
              isLoading: false,
              error: relationsResponse.message || '重新載入關聯資料失敗'
            }));
          }
        }
      } else {
        // 根據錯誤代碼提供更具體的錯誤訊息
        let errorMessage = response.message || '提交失敗';
        if (response.code === ERROR_CODES.INVALID_TIME_RANGE) {
          errorMessage = '時間範圍無效，請檢查開始和結束時間';
        } else if (response.code === ERROR_CODES.SAME_VIDEO_IDS) {
          errorMessage = '來源和目標影片不能相同';
        } else if (response.code === ERROR_CODES.DUPLICATE_RELATION) {
          errorMessage = '此關聯已存在';
        }
        addNotification('error', errorMessage, 5000);
        setState(prev => ({ ...prev, isLoading: false }));
      }
    } catch (error) {
      console.error('Failed to submit relation:', error);
      addNotification('error', '提交失敗，請稍後再試', 5000);
      setState(prev => ({ ...prev, isLoading: false }));
    }
  };

  // 新增通知
  const addNotification = (type: 'success' | 'error' | 'info' | 'warning', message: string, duration = 3000) => {
    const id = Date.now().toString();
    setState(prev => ({
      ...prev,
      notifications: [...prev.notifications, { id, type, message, duration }]
    }));
  };

  // 關閉通知
  const closeNotification = (id: string) => {
    setState(prev => ({
      ...prev,
      notifications: prev.notifications.filter(n => n.id !== id)
    }));
  };

  // 切換面板顯示
  const togglePanel = () => {
    setState(prev => ({ ...prev, isVisible: !prev.isVisible }));
  };

  if (!state.isVisible) {
    return null;
  }

  return (
    <>
      {/* 通知 */}
      {state.notifications.map(notification => (
        <Notification
          key={notification.id}
          notification={notification}
          onClose={closeNotification}
        />
      ))}

      {/* 主要面板 */}
      <div className="fixed top-4 right-4 z-40 w-96 bg-white/90 backdrop-blur-sm rounded-lg shadow-xl border border-gray-200">
        {/* 標題欄 */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">
            Ariadne - 影片關聯
          </h2>
          <button
            onClick={togglePanel}
            className="text-gray-400 hover:text-gray-600 transition-colors"
          >
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clipRule="evenodd" />
            </svg>
          </button>
        </div>

        {/* 內容區域 */}
        <div className="p-4 max-h-96 overflow-y-auto">
          {state.isLoading ? (
            <div className="flex items-center justify-center py-8">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
            </div>
          ) : state.error ? (
            <div className="text-red-600 text-center py-4">
              {state.error}
            </div>
          ) : (
            <div className="space-y-4">
              {/* 當前影片資訊 */}
              {state.videoInfo && (
                <div className="bg-gray-50 p-3 rounded-lg">
                  <h3 className="font-medium text-gray-900 mb-2">當前影片</h3>
                  <p className="text-sm text-gray-600 truncate">{state.videoInfo.title}</p>
                  <p className="text-xs text-gray-500">{state.videoInfo.channel}</p>
                </div>
              )}

              {/* 關聯列表 */}
              <div>
                <h3 className="font-medium text-gray-900 mb-2">相關關聯</h3>
                {state.relations.length > 0 ? (
                  <div className="space-y-2">
                    {state.relations.map(relation => (
                      <div key={relation.id} className="bg-blue-50 p-3 rounded-lg">
                        <div className="text-sm text-blue-900">
                          {relation.sourceVideoUrl !== state.videoInfo?.url ? '來源' : '衍生'} 片段
                        </div>
                        <div className="text-xs text-blue-700 mt-1">
                          {relation.sourceStartTime}s - {relation.sourceEndTime}s
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="bg-gray-50 p-4 rounded-lg text-center">
                    <div className="text-gray-500 text-sm">
                      <svg className="w-8 h-8 mx-auto mb-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                      <p>此影片尚未有關聯資料</p>
                      <p className="text-xs mt-1">成為第一個標記關聯的人！</p>
                    </div>
                  </div>
                )}
              </div>

              {/* 新增關聯表單 */}
              <div>
                <h3 className="font-medium text-gray-900 mb-2">新增關聯</h3>
                <RelationForm
                  onSubmit={handleSubmitRelation}
                  isLoading={state.isLoading}
                />
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
};

// 初始化內容腳本
const initContentScript = () => {
  // 檢查是否已經注入
  if (document.getElementById('ariadne-content-script')) {
    return;
  }

  // 建立容器
  const container = document.createElement('div');
  container.id = 'ariadne-content-script';
  document.body.appendChild(container);

  // 建立 React 根
  const root = createRoot(container);
  root.render(<ContentScript />);
};

// 等待 DOM 載入完成
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initContentScript);
} else {
  initContentScript();
}

export default ContentScript; 