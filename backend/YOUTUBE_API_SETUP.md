# YouTube Data API 設置指南

## 概述

Ariadne 使用 YouTube Data API 來獲取影片的準確資訊，包括影片時長、標題、頻道等。這對於確保時間範圍驗證的準確性非常重要。

## 獲取 YouTube Data API 金鑰

### 1. 創建 Google Cloud 專案

1. 前往 [Google Cloud Console](https://console.cloud.google.com/)
2. 點擊「選擇專案」或創建新專案
3. 為專案命名（例如：`ariadne-youtube-api`）

### 2. 啟用 YouTube Data API v3

1. 在 Google Cloud Console 中，前往「API 和服務」>「程式庫」
2. 搜尋「YouTube Data API v3」
3. 點擊「啟用」

### 3. 創建憑證

1. 前往「API 和服務」>「憑證」
2. 點擊「建立憑證」>「API 金鑰」
3. 複製生成的 API 金鑰

### 4. 限制 API 金鑰（推薦）

1. 點擊剛創建的 API 金鑰
2. 在「應用程式限制」中選擇「HTTP 參照網址」
3. 添加你的網域（例如：`http://localhost:8000/*`）
4. 在「API 限制」中選擇「限制金鑰」
5. 只選擇「YouTube Data API v3」

## 配置 Ariadne

### 方法 1：環境變數（推薦）

```bash
export YOUTUBE_API_KEY="your_api_key_here"
```

### 方法 2：配置文件

編輯 `backend/src/config/config.toml`：

```toml
# YouTube API 配置
youtube_api_key = "your_api_key_here"
```

### 方法 3：Docker 環境變數

在 `docker-compose.yml` 中添加：

```yaml
services:
  backend:
    environment:
      - YOUTUBE_API_KEY=your_api_key_here
```

## 驗證設置

1. 啟動後端服務
2. 測試 API 端點：

```bash
curl "http://localhost:8000/api/video/info?url=https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

如果設置正確，應該返回影片資訊：

```json
{
  "code": 200,
  "data": {
    "id": "dQw4w9WgXcQ",
    "title": "Rick Astley - Never Gonna Give You Up",
    "channel_title": "Rick Astley",
    "duration": 212,
    "published_at": "2009-10-25T06:57:33Z",
    "description": "..."
  }
}
```

## 配額限制

YouTube Data API 有配額限制：
- 免費帳戶：每天 10,000 個單位
- 每個 `videos.list` 請求消耗 1 個單位

### 監控配額使用

1. 前往 Google Cloud Console
2. 選擇你的專案
3. 前往「API 和服務」>「配額」
4. 查看「YouTube Data API v3」的使用情況

### 優化建議

1. **快取機制**：實現影片資訊快取，避免重複請求
2. **批量請求**：如果需要獲取多個影片資訊，使用批量 API
3. **錯誤處理**：當 API 配額用完時，回退到頁面解析方法

## 故障排除

### 常見錯誤

1. **API 金鑰無效**
   - 檢查金鑰是否正確複製
   - 確認 API 已啟用

2. **配額超限**
   - 檢查配額使用情況
   - 考慮升級到付費帳戶

3. **網域限制**
   - 確認 HTTP 參照網址設置正確
   - 檢查是否包含正確的協議（http/https）

### 回退機制

如果 YouTube API 不可用，Ariadne 會自動回退到頁面解析方法來獲取影片時長。雖然不如 API 準確，但能確保基本功能正常運作。

## 安全注意事項

1. **不要提交 API 金鑰到版本控制**
2. **使用環境變數或安全的配置管理**
3. **定期輪換 API 金鑰**
4. **監控 API 使用情況，防止濫用** 