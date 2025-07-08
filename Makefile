# Ariadne Project Makefile

.PHONY: help install setup build run test clean
.PHONY: db-setup db-migrate db-rollback db-status db-reset db-prepare
.PHONY: docker-up docker-down docker-logs docker-build
.PHONY: backend frontend

# 預設目標
help:
	@echo "🚀 Ariadne Project Commands"
	@echo ""
	@echo "📦 Project Management:"
	@echo "  install       - 安裝所有依賴"
	@echo "  setup         - 完整設置開發環境"
	@echo "  build         - 建置所有專案"
	@echo "  test          - 執行所有測試"
	@echo "  clean         - 清理所有建置檔案"
	@echo ""
	@echo "🗄️  Database Management:"
	@echo "  db-setup      - 設置資料庫和執行遷移"
	@echo "  db-migrate    - 執行所有待處理的遷移"
	@echo "  db-rollback   - 回滾最後一次遷移"
	@echo "  db-rollback-all - 回滾所有遷移（回到最初狀態）"
	@echo "  db-status     - 檢查遷移狀態"
	@echo "  db-prepare    - 準備 SQLx 查詢快取"
	@echo ""
	@echo "🐳 Infrastructure:"
	@echo "  docker-up     - 啟動所有服務"
	@echo "  docker-down   - 停止所有服務"
	@echo "  docker-logs   - 查看服務日誌"
	@echo "  docker-build  - 建置 Docker 映像"
	@echo ""
	@echo "🔧 Development:"
	@echo "  backend       - 後端開發命令"
	@echo "  frontend      - 前端開發命令"

# =============================================================================
# 專案管理
# =============================================================================

# 安裝所有依賴
install:
	@echo "📦 Installing dependencies..."
	@echo "Installing SQLx CLI..."
	cargo install sqlx-cli --no-default-features --features postgres
	@echo "Installing backend dependencies..."
	cd backend && cargo build
	@echo "Installing frontend dependencies..."
	cd frontend && bun install
	@echo "✅ Dependencies installed!"

# 完整設置開發環境
setup: install db-setup
	@echo "✅ Full development environment setup complete!"
	@echo "Run 'make docker-up' to start services"
	@echo "Run 'make backend-run' for backend server"
	@echo "Run 'make frontend-run' for frontend development server"

# 建置所有專案
build:
	@echo "🔨 Building all projects..."
	cd backend && cargo build
	cd frontend && bun run build
	@echo "✅ Build complete!"

# 執行所有測試
test:
	@echo "🧪 Running all tests..."
	cd backend && cargo test
	cd frontend && bun run test
	@echo "✅ Tests complete!"

# 清理所有建置檔案
clean:
	@echo "🧹 Cleaning build files..."
	cd backend && cargo clean
	cd frontend && rm -rf dist node_modules/.vite
	@echo "✅ Clean complete!"

# =============================================================================
# 資料庫管理
# =============================================================================

# 設置資料庫
db-setup:
	@echo "🗄️  Setting up database..."
	@if [ ! -f backend/.env ]; then \
		echo "Error: backend/.env file not found. Please copy env.example to .env and configure it."; \
		exit 1; \
	fi
	@echo "Starting PostgreSQL with Docker Compose..."
	docker-compose -f infrastructure/docker-compose.yml up -d postgres
	@echo "Waiting for database to be ready..."
	@sleep 5
	@echo "Running migrations..."
	export $$(grep -v '^#' backend/.env | xargs) && sqlx migrate run --source database/migrations/postgres
	@echo "✅ Database setup complete!"

# 執行遷移
db-migrate:
	@if [ ! -f backend/.env ]; then \
		echo "Error: backend/.env file not found. Please copy env.example to .env and configure it."; \
		exit 1; \
	fi
	export $$(grep -v '^#' backend/.env | xargs) && sqlx migrate run --source database/migrations/postgres

# 回滾遷移
db-rollback:
	@if [ ! -f backend/.env ]; then \
		echo "Error: backend/.env file not found. Please copy env.example to .env and configure it."; \
		exit 1; \
	fi
	export $$(grep -v '^#' backend/.env | xargs) && sqlx migrate revert --source database/migrations/postgres

# 回滾所有遷移（回到最初狀態）
db-rollback-all:
	@if [ ! -f backend/.env ]; then \
		echo "Error: backend/.env file not found. Please copy env.example to .env and configure it."; \
		exit 1; \
	fi
	@echo "⚠️  Warning: This will revert ALL migrations!"
	@read -p "Are you sure? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
	@echo "Reverting all migrations ..."
	export $$(grep -v '^#' backend/.env | xargs) && sqlx migrate revert --source database/migrations/postgres --target-version 0
	@echo "✅ All migrations reverted!"

# 檢查遷移狀態
db-status:
	@if [ ! -f backend/.env ]; then \
		echo "Error: backend/.env file not found. Please copy env.example to .env and configure it."; \
		exit 1; \
	fi
	export $$(grep -v '^#' backend/.env | xargs) && sqlx migrate info --source database/migrations/postgres

# 準備 SQLx 查詢快取
db-prepare:
	@if [ ! -f backend/.env ]; then \
		echo "Error: backend/.env file not found. Please copy env.example to .env and configure it."; \
		exit 1; \
	fi
	cd backend && export $$(grep -v '^#' .env | xargs) && cargo sqlx prepare

# =============================================================================
# 基礎設施管理
# =============================================================================

# 啟動所有服務
docker-up:
	@echo "🐳 Starting all services..."
	docker-compose -f infrastructure/docker-compose.yml up -d
	@echo "✅ Services started!"
	@echo "📊 PgAdmin: http://localhost:8080 (admin@ariadne.local / admin)"

# 停止所有服務
docker-down:
	@echo "🐳 Stopping all services..."
	docker-compose -f infrastructure/docker-compose.yml down
	@echo "✅ Services stopped!"

# 查看服務日誌
docker-logs:
	docker-compose -f infrastructure/docker-compose.yml logs -f

# 建置 Docker 映像
docker-build:
	@echo "🔨 Building Docker images..."
	docker-compose -f infrastructure/docker-compose.yml build
	@echo "✅ Docker images built!"

# =============================================================================
# 開發命令
# =============================================================================

# 後端開發命令
backend:
	@echo "🔧 Backend Development Commands:"
	@echo ""
	@echo "  make backend-run     - 執行後端伺服器"
	@echo "  make backend-build   - 建置後端"
	@echo "  make backend-test    - 測試後端"
	@echo "  make backend-clean   - 清理後端"
	@echo ""
	@echo "  make backend-install - 安裝後端依賴"

# 前端開發命令
frontend:
	@echo "🎨 Frontend Development Commands:"
	@echo ""
	@echo "  make frontend-run      - 執行前端開發伺服器"
	@echo "  make frontend-build    - 建置前端"
	@echo "  make frontend-test     - 測試前端"
	@echo "  make frontend-clean    - 清理前端"
	@echo "  make frontend-preview  - 預覽前端建置"
	@echo "  make frontend-extension - 建置 Chrome Extension"
	@echo ""
	@echo "  make frontend-install  - 安裝前端依賴"

# =============================================================================
# 後端特定命令
# =============================================================================

backend-run:
	@echo "🚀 Starting backend server..."
	cd backend && cargo run

backend-build:
	@echo "🔨 Building backend..."
	cd backend && cargo build

backend-test:
	@echo "🧪 Testing backend..."
	cd backend && cargo test

backend-clean:
	@echo "🧹 Cleaning backend..."
	cd backend && cargo clean

backend-install:
	@echo "📦 Installing backend dependencies..."
	cd backend && cargo install

# =============================================================================
# 前端特定命令
# =============================================================================

frontend-run:
	@echo "🎨 Starting frontend development server..."
	cd frontend && bun run dev

frontend-build:
	@echo "🔨 Building frontend..."
	cd frontend && bun run build

frontend-test:
	@echo "🧪 Testing frontend..."
	cd frontend && bun run test

frontend-clean:
	@echo "🧹 Cleaning frontend..."
	cd frontend && rm -rf dist node_modules/.vite

frontend-install:
	@echo "📦 Installing frontend dependencies..."
	cd frontend && bun install

frontend-preview:
	@echo "👀 Previewing frontend build..."
	cd frontend && bun run preview

frontend-extension:
	@echo "🔧 Building Chrome Extension..."
	cd frontend && rm -rf dist && bun run build:extension
