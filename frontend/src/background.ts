// 背景腳本 - 處理擴充功能的生命週期和事件

// 安裝時的初始化
if (typeof chrome !== 'undefined') {
  chrome.runtime.onInstalled.addListener((details: { reason: string }) => {
    console.log('Ariadne extension installed:', details.reason);

    // 設定預設值
    chrome.storage.local.set({
      enabled: true,
      apiUrl: 'http://localhost:3000',
      notifications: true
    });
  });

  // 處理來自內容腳本和彈出視窗的訊息
  chrome.runtime.onMessage.addListener((request: any, _sender: any, sendResponse: (response: any) => void) => {
    console.log('Received message:', request);

    switch (request.type) {
      case 'GET_VIDEO_INFO':
        // 取得當前分頁的影片資訊
        chrome.tabs.query({ active: true, currentWindow: true }, (tabs: Array<{ url?: string }>) => {
          const activeTab = tabs[0];
          if (activeTab && activeTab.url?.includes('youtube.com/watch')) {
            sendResponse({ success: true, url: activeTab.url });
          } else {
            sendResponse({ success: false, error: 'Not a YouTube video page' });
          }
        });
        return true; // 保持訊息通道開啟

      case 'OPEN_TAB':
        // 開啟新分頁
        chrome.tabs.create({ url: request.url });
        sendResponse({ success: true });
        break;

      default:
        sendResponse({ success: false, error: 'Unknown message type' });
    }
  });

  // 處理分頁更新
  chrome.tabs.onUpdated.addListener((_tabId: number, changeInfo: { status: string }, tab: { url?: string }) => {
    if (changeInfo.status === 'complete' && tab.url?.includes('youtube.com/watch')) {
      // 當 YouTube 影片頁面載入完成時，可以執行一些初始化操作
      console.log('YouTube video page loaded:', tab.url);
    }
  });

  // 處理擴充功能圖示點擊
  chrome.action.onClicked.addListener((tab: { url?: string }) => {
    if (tab.url?.includes('youtube.com/watch')) {
      // 在 YouTube 影片頁面點擊圖示時，可以觸發一些操作
      console.log('Extension icon clicked on YouTube video page');
    }
  });
}