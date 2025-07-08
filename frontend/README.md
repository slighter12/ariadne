# Ariadne Frontend - Chrome Extension

這是 Ariadne 影片內容溯源系統的前端部分，使用 React + TypeScript + Vite 開發的 Chrome Extension。

## 功能特色

- 🎯 **YouTube 影片頁面整合** - 自動偵測 YouTube 影片頁面並注入關聯面板
- ⏱️ **時間範圍選擇器** - 直觀的時間軸選擇，支援當前時間快速設定
- 🔗 **關聯標記功能** - 標記影片片段之間的引用關係
- 📊 **即時統計** - 顯示關聯統計資訊
- 🔔 **通知系統** - 操作結果即時反饋
- 🎨 **現代化 UI** - 使用 Tailwind CSS 設計的美觀介面

## 技術棧

- **React 19** - 現代化 UI 框架
- **TypeScript** - 類型安全的 JavaScript
- **Vite** - 快速的建置工具
- **Tailwind CSS** - 實用優先的 CSS 框架
- **Bun** - 快速的 JavaScript 執行環境
- **Chrome Extension API** - 瀏覽器擴充功能 API

## 開發環境設置

### 1. 安裝依賴

```bash
bun install
```

### 2. 開發模式

```bash
bun run dev
```

這會啟動 Vite 開發伺服器，你可以在瀏覽器中預覽彈出視窗。

### 3. 建置 Chrome Extension

```bash
bun run build:extension
```

這會在建置 `dist/` 目錄中產生 Chrome Extension 檔案。

## 專案結構

```
frontend/
├── src/
│   ├── components/          # React 組件
│   │   ├── ContentScript.tsx    # 主要內容腳本組件
│   │   ├── Popup.tsx            # 彈出視窗組件
│   │   ├── RelationForm.tsx     # 關聯表單組件
│   │   ├── TimeSelector.tsx     # 時間選擇器組件
│   │   └── Notification.tsx     # 通知組件
│   ├── services/            # API 服務
│   │   └── api.ts
│   ├── types/              # TypeScript 類型定義
│   │   ├── index.ts
│   │   └── chrome.d.ts
│   ├── utils/              # 工具函數
│   │   └── youtube.ts
│   ├── content.ts          # 內容腳本入口
│   └── background.ts       # 背景腳本入口
├── public/
│   ├── manifest.json       # Chrome Extension 清單
│   └── icons/              # 圖示檔案
└── dist/                   # 建置輸出
```

## 核心組件說明

### ContentScript.tsx
主要的內容腳本組件，負責：
- YouTube 頁面偵測
- 影片資訊解析
- 關聯資料載入
- UI 面板渲染

### RelationForm.tsx
關聯提交表單，包含：
- 來源影片 URL 輸入
- 目標影片 URL 輸入
- 時間範圍選擇
- 表單驗證

### TimeSelector.tsx
時間選擇器組件，提供：
- 滑桿式時間選擇
- 當前時間快速設定
- 時間範圍驗證
- 快速選擇按鈕

### Popup.tsx
彈出視窗組件，顯示：
- 統計資訊
- 快速操作按鈕
- 使用說明
- 版本資訊

## API 整合

前端透過 `ApiService` 與後端 API 進行整合：

- `submitRelation()` - 提交關聯
- `getVideoRelations()` - 取得影片關聯
- `getSourceRelations()` - 取得來源反向索引
- `approveRelation()` - 審核關聯

## 開發指令

```bash
# 開發模式
bun run dev

# 建置生產版本
bun run build

# 建置 Chrome Extension
bun run build:extension

# 預覽建置版本
bun run preview

# 執行測試
bun run test

# 程式碼檢查
bun run lint
```

## 安裝 Chrome Extension

1. 執行建置指令：
   ```bash
   bun run build:extension
   ```

2. 開啟 Chrome 瀏覽器，進入 `chrome://extensions/`

3. 開啟「開發人員模式」

4. 點擊「載入未封裝項目」

5. 選擇 `frontend/dist/` 目錄

6. Extension 會出現在擴充功能列表中

## 使用方式

1. **在 YouTube 影片頁面**：
   - 自動顯示關聯面板
   - 可以查看現有關聯
   - 可以新增關聯

2. **點擊 Extension 圖示**：
   - 開啟彈出視窗
   - 查看統計資訊
   - 快速操作

3. **新增關聯**：
   - 輸入來源影片 URL
   - 選擇時間範圍
   - 輸入目標影片 URL
   - 選擇時間範圍
   - 提交關聯

## 注意事項

- 目前 API 端點設定為 `http://localhost:3000`，生產環境需要修改
- 需要後端服務運行才能正常使用
- Chrome Extension 需要在 HTTPS 環境下才能使用某些功能

## 貢獻

歡迎提交 Issue 和 Pull Request！
