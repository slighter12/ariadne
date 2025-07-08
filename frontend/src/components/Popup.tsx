import React, { useState, useEffect } from 'react';

const Popup: React.FC = () => {
  const [stats, setStats] = useState({
    totalRelations: 0,
    pendingRelations: 0,
    approvedRelations: 0
  });
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadStats();
  }, []);

  const loadStats = async () => {
    try {
      // 這裡可以載入統計資料
      // 目前使用模擬資料
      setStats({
        totalRelations: 42,
        pendingRelations: 5,
        approvedRelations: 37
      });
    } catch (error) {
      console.error('Failed to load stats:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const openOptions = () => {
    if (typeof chrome !== 'undefined' && chrome.runtime) {
      chrome.runtime.openOptionsPage();
    }
  };

  const openDashboard = () => {
    if (typeof chrome !== 'undefined' && chrome.tabs) {
      chrome.tabs.create({ url: 'http://localhost:3000/dashboard' });
    }
  };

  if (isLoading) {
    return (
      <div className="w-80 p-4">
        <div className="flex items-center justify-center py-8">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="w-80 bg-white">
      {/* 標題 */}
      <div className="bg-blue-600 text-white p-4">
        <h1 className="text-lg font-semibold">Ariadne</h1>
        <p className="text-sm text-blue-100">影片內容溯源系統</p>
      </div>

      {/* 統計資訊 */}
      <div className="p-4 space-y-4">
        <div className="grid grid-cols-3 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{stats.totalRelations}</div>
            <div className="text-xs text-gray-600">總關聯數</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-yellow-600">{stats.pendingRelations}</div>
            <div className="text-xs text-gray-600">待審核</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">{stats.approvedRelations}</div>
            <div className="text-xs text-gray-600">已通過</div>
          </div>
        </div>

        {/* 快速操作 */}
        <div className="space-y-2">
          <button
            onClick={openDashboard}
            className="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors text-sm"
          >
            開啟管理面板
          </button>
          
          <button
            onClick={openOptions}
            className="w-full px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-sm"
          >
            設定
          </button>
        </div>

        {/* 使用說明 */}
        <div className="bg-gray-50 p-3 rounded-lg">
          <h3 className="text-sm font-medium text-gray-900 mb-2">使用說明</h3>
          <ul className="text-xs text-gray-600 space-y-1">
            <li>• 在 YouTube 影片頁面會自動顯示關聯面板</li>
            <li>• 點擊「新增關聯」來標記影片片段關係</li>
            <li>• 支援時間範圍選擇和來源影片輸入</li>
          </ul>
        </div>

        {/* 版本資訊 */}
        <div className="text-center text-xs text-gray-500 pt-2 border-t border-gray-200">
          Version 1.0.0
        </div>
      </div>
    </div>
  );
};

export default Popup; 