# Rust 泛型約束在乾淨架構中的應用

## 概述

本專案使用 Rust 的泛型約束（Generic Constraints）來實現乾淨架構（Clean Architecture），提供更好的依賴注入、測試性和可維護性。

## 架構設計

### 1. Trait 定義（Interface Layer）

```rust
// src/traits/repository_traits.rs
#[async_trait]
pub trait RelationRepositoryTrait: Send + Sync {
    async fn find_by_video_id(&self, params: &GetRelationsParams) -> Result<Vec<Relation>, sqlx::Error>;
    async fn create(&self, payload: &CreateRelationPayload) -> Result<Relation, sqlx::Error>;
    // ... 其他方法
}
```

### 2. Service 層使用泛型約束

```rust
// src/services/relation_service.rs
pub struct RelationService<R>
where
    R: RelationRepositoryTrait,
{
    repository: R,
}

impl<R> RelationService<R>
where
    R: RelationRepositoryTrait,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    // 業務邏輯方法...
}
```

### 3. AppState 使用泛型約束

```rust
// src/state.rs
pub struct AppState<R, U>
where
    R: RelationRepositoryTrait + Clone,
    U: UserRepositoryTrait + Clone,
{
    pub db_pool: Arc<Pool<Postgres>>,
    pub relation_service: RelationService<R>,
    pub user_service: UserService<U>,
}
```

## 優勢

### 1. 零成本抽象

泛型約束在編譯時會被單態化（monomorphization），不會有運行時開銷：

```rust
// 編譯時會生成具體的類型
let real_service = RelationService::new(RelationRepository::new(pool));
let mock_service = RelationService::new(MockRelationRepository::new());
```

### 2. 易於測試

可以輕鬆替換真實實作為 Mock：

```rust
#[tokio::test]
async fn test_relation_service() {
    // 使用 Mock，不需要資料庫
    let mock_repo = MockRelationRepository::new();
    let service = RelationService::new(mock_repo);
    
    let result = service.create_relation(payload).await;
    assert!(result.is_ok());
}
```

### 3. 依賴注入

可以輕鬆注入不同的實作：

```rust
// 生產環境
let real_repo = RelationRepository::new(pool);
let service = RelationService::new(real_repo);

// 測試環境
let mock_repo = MockRelationRepository::new();
let service = RelationService::new(mock_repo);
```

### 4. 編譯時檢查

編譯器會確保所有 trait 方法都被正確實作：

```rust
// 如果 MockRepository 沒有實作所有必要方法，編譯會失敗
impl RelationRepositoryTrait for MockRelationRepository {
    // 必須實作所有 trait 方法
}
```

## 使用方式

### 1. 建立 Service

```rust
// 使用真實 Repository
let pool = sqlx::postgres::PgPoolOptions::new()
    .connect(&database_url)
    .await?;
let relation_repo = RelationRepository::new(pool);
let relation_service = RelationService::new(relation_repo);

// 使用 Mock Repository
let mock_repo = MockRelationRepository::new();
let relation_service = RelationService::new(mock_repo);
```

### 2. 泛型函數

```rust
async fn process_relations<R>(service: &RelationService<R>) 
where
    R: RelationRepositoryTrait,
{
    // 可以處理任何實作 RelationRepositoryTrait 的類型
    let result = service.get_relations(params).await;
    // ...
}
```

### 3. 測試

```rust
#[tokio::test]
async fn test_service_with_mock() {
    let mock_repo = MockRelationRepository::new()
        .with_relations(vec![test_relation()]);
    
    let service = RelationService::new(mock_repo);
    
    let result = service.get_relations(params).await;
    assert!(result.is_ok());
}
```

## 最佳實踐

### 1. Trait 設計

```rust
// 好的設計：明確的約束
pub trait RepositoryTrait: Send + Sync {
    // 方法定義
}

// 避免：過於寬鬆的約束
pub trait RepositoryTrait {
    // 缺少必要的約束
}
```

### 2. 泛型約束

```rust
// 好的設計：明確的約束
pub struct Service<R>
where
    R: RepositoryTrait + Clone,
{
    repository: R,
}

// 避免：過於複雜的約束
pub struct Service<R, U, V>
where
    R: RepositoryTrait + Clone + Debug + Send + Sync,
    U: AnotherTrait + Clone,
    V: YetAnotherTrait,
{
    // 過於複雜
}
```

### 3. 測試策略

```rust
// 使用 Mock 進行單元測試
#[tokio::test]
async fn test_service_logic() {
    let mock_repo = MockRepository::new();
    let service = Service::new(mock_repo);
    // 測試業務邏輯
}

// 使用真實 Repository 進行整合測試
#[tokio::test]
async fn test_integration() {
    let pool = create_test_pool().await;
    let repo = Repository::new(pool);
    let service = Service::new(repo);
    // 測試完整流程
}
```

## 總結

Rust 的泛型約束提供了：

1. **零成本抽象**：編譯時優化，無運行時開銷
2. **強類型安全**：編譯時檢查所有依賴
3. **易於測試**：可以輕鬆替換實作
4. **清晰的架構**：明確的依賴關係和約束

這種設計模式特別適合需要高可測試性和可維護性的專案，如本專案的影片關聯系統。 