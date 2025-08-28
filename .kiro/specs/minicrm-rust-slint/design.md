# MiniCRM Rust + Slint 设计文档

## 概述

MiniCRM是一个基于Rust和Slint的现代化跨平台客户关系管理系统，专为板材行业设计。系统采用高度模块化的架构设计，通过trait系统和泛型实现代码复用，最大化减少重复代码。

## 模块化架构设计

### 整体架构
```
┌─────────────────────────────────────────────────────────────┐
│                    MiniCRM 模块化架构                        │
├─────────────────────────────────────────────────────────────┤
│  表示层 (Presentation Layer)                                │
│  ├── Slint UI组件 (可复用UI组件库)                          │
│  ├── 通用表单组件 (GenericForm<T>)                          │
│  ├── 通用列表组件 (GenericList<T>)                          │
│  ├── 通用图表组件 (GenericChart<T>)                         │
│  └── 事件处理器 (EventHandler<T>)                           │
├─────────────────────────────────────────────────────────────┤
│  业务逻辑层 (Business Logic Layer)                          │
│  ├── 通用服务层 (GenericService<T>)                         │
│  ├── 通用CRUD操作 (CrudOperations<T>)                       │
│  ├── 通用搜索服务 (SearchService<T>)                        │
│  ├── 通用验证器 (Validator<T>)                              │
│  └── 通用报表生成器 (ReportGenerator<T>)                    │
├─────────────────────────────────────────────────────────────┤
│  数据访问层 (Data Access Layer)                             │
│  ├── 通用Repository (GenericRepository<T>)                  │
│  ├── 数据库连接池 (DatabasePool)                            │
│  ├── 查询构建器 (QueryBuilder<T>)                           │
│  └── 数据映射器 (DataMapper<T>)                             │
├─────────────────────────────────────────────────────────────┤
│  共享模块层 (Shared Modules Layer)                          │
│  ├── 通用错误处理 (ErrorHandler)                            │
│  ├── 通用配置管理 (ConfigManager)                           │
│  ├── 通用日志系统 (Logger)                                  │
│  ├── 通用工具函数 (Utils)                                   │
│  └── 通用类型定义 (CommonTypes)                             │
└─────────────────────────────────────────────────────────────┘
```

## 核心Trait设计 (代码复用基础)

### 1. 实体Trait (Entity)
```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 所有业务实体的基础trait
pub trait Entity: Clone + Serialize + for<'de> Deserialize<'de> + Validate + Send + Sync {
    type Id: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync;
    
    fn id(&self) -> Option<Self::Id>;
    fn set_id(&mut self, id: Self::Id);
    fn table_name() -> &'static str;
    fn display_name(&self) -> String;
}

/// 客户实体实现
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct Customer {
    pub id: Option<i64>,
    #[validate(length(min = 1, message = "客户名称不能为空"))]
    pub name: String,
    #[validate(regex(path = "PHONE_REGEX", message = "电话格式不正确"))]
    pub phone: String,
    pub email: Option<String>,
    pub level: CustomerLevel,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Entity for Customer {
    type Id = i64;
    
    fn id(&self) -> Option<Self::Id> { self.id }
    fn set_id(&mut self, id: Self::Id) { self.id = Some(id); }
    fn table_name() -> &'static str { "customers" }
    fn display_name(&self) -> String { self.name.clone() }
}

/// 供应商实体实现
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct Supplier {
    pub id: Option<i64>,
    #[validate(length(min = 1, message = "供应商名称不能为空"))]
    pub name: String,
    #[validate(regex(path = "PHONE_REGEX", message = "电话格式不正确"))]
    pub phone: String,
    pub level: SupplierLevel,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Entity for Supplier {
    type Id = i64;
    
    fn id(&self) -> Option<Self::Id> { self.id }
    fn set_id(&mut self, id: Self::Id) { self.id = Some(id); }
    fn table_name() -> &'static str { "suppliers" }
    fn display_name(&self) -> String { self.name.clone() }
}
```

### 2. 通用Repository Trait
```rust
use async_trait::async_trait;
use crate::error::Result;

/// 通用数据访问trait，所有实体的Repository都实现此trait
#[async_trait]
pub trait Repository<T: Entity> {
    async fn create(&self, entity: &mut T) -> Result<T::Id>;
    async fn find_by_id(&self, id: T::Id) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn update(&self, entity: &T) -> Result<()>;
    async fn delete(&self, id: T::Id) -> Result<()>;
    async fn search(&self, query: &SearchQuery) -> Result<SearchResult<T>>;
    async fn count(&self) -> Result<i64>;
}

/// 通用Repository实现
pub struct GenericRepository<T: Entity> {
    pool: Arc<DatabasePool>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> GenericRepository<T> {
    pub fn new(pool: Arc<DatabasePool>) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<T: Entity> Repository<T> for GenericRepository<T> {
    async fn create(&self, entity: &mut T) -> Result<T::Id> {
        let conn = self.pool.get().await?;
        
        // 通用插入逻辑
        let sql = format!("INSERT INTO {} (...) VALUES (...)", T::table_name());
        // 使用反射或宏生成具体的SQL
        
        // 执行插入并返回ID
        todo!("实现通用插入逻辑")
    }
    
    async fn find_by_id(&self, id: T::Id) -> Result<Option<T>> {
        let conn = self.pool.get().await?;
        
        let sql = format!("SELECT * FROM {} WHERE id = ?", T::table_name());
        // 执行查询并映射到实体
        
        todo!("实现通用查询逻辑")
    }
    
    // 其他方法的通用实现...
}
```

### 3. 通用Service Trait
```rust
/// 通用业务服务trait
#[async_trait]
pub trait Service<T: Entity> {
    type Repository: Repository<T>;
    
    fn repository(&self) -> &Self::Repository;
    
    async fn create(&self, mut entity: T) -> Result<T> {
        // 通用验证逻辑
        entity.validate()?;
        
        // 通用创建逻辑
        let id = self.repository().create(&mut entity).await?;
        entity.set_id(id);
        
        Ok(entity)
    }
    
    async fn get_by_id(&self, id: T::Id) -> Result<Option<T>> {
        self.repository().find_by_id(id).await
    }
    
    async fn update(&self, entity: T) -> Result<T> {
        // 通用验证和更新逻辑
        entity.validate()?;
        self.repository().update(&entity).await?;
        Ok(entity)
    }
    
    async fn delete(&self, id: T::Id) -> Result<()> {
        self.repository().delete(id).await
    }
    
    async fn search(&self, query: SearchQuery) -> Result<SearchResult<T>> {
        self.repository().search(&query).await
    }
}

/// 客户服务实现
pub struct CustomerService {
    repository: Arc<GenericRepository<Customer>>,
}

impl Service<Customer> for CustomerService {
    type Repository = GenericRepository<Customer>;
    
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

/// 供应商服务实现
pub struct SupplierService {
    repository: Arc<GenericRepository<Supplier>>,
}

impl Service<Supplier> for SupplierService {
    type Repository = GenericRepository<Supplier>;
    
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}
```###
 4. 通用UI组件设计

```rust
// 通用列表组件
slint::slint! {
    export component GenericList<T> {
        in property <[T]> items;
        in property <string> title;
        callback item-selected(T);
        callback item-edited(T);
        callback item-deleted(T);
        
        VerticalBox {
            Text {
                text: title;
                font-size: 18px;
                font-weight: 700;
            }
            
            ListView {
                for item in items: Rectangle {
                    // 通用列表项渲染
                    TouchArea {
                        clicked => { item-selected(item) }
                    }
                }
            }
        }
    }
}

// 通用表单组件
slint::slint! {
    export component GenericForm<T> {
        in property <T> entity;
        in property <[FormField]> fields;
        callback save-clicked(T);
        callback cancel-clicked();
        
        VerticalBox {
            for field in fields: HorizontalBox {
                Text { text: field.label; }
                LineEdit { 
                    text: field.value;
                    // 绑定到实体字段
                }
            }
            
            HorizontalBox {
                Button {
                    text: "保存";
                    clicked => { save-clicked(entity) }
                }
                Button {
                    text: "取消";
                    clicked => { cancel-clicked() }
                }
            }
        }
    }
}
```

### 5. 通用搜索和筛选系统

```rust
/// 通用搜索查询
#[derive(Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub filters: HashMap<String, FilterValue>,
    pub sort_by: Option<String>,
    pub sort_order: SortOrder,
    pub page: usize,
    pub page_size: usize,
}

/// 通用搜索结果
#[derive(Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: usize,
    pub page_size: usize,
}

/// 通用搜索服务
pub struct SearchService<T: Entity> {
    repository: Arc<dyn Repository<T>>,
}

impl<T: Entity> SearchService<T> {
    pub async fn search(&self, query: SearchQuery) -> Result<SearchResult<T>> {
        // 通用搜索逻辑
        self.repository.search(&query).await
    }
    
    pub async fn get_filter_options(&self, field: &str) -> Result<Vec<FilterOption>> {
        // 通用筛选选项获取
        todo!("实现通用筛选选项获取")
    }
}
```

### 6. 通用验证系统

```rust
/// 通用验证器trait
pub trait Validator<T> {
    fn validate(&self, entity: &T) -> Result<()>;
}

/// 通用验证器实现
pub struct GenericValidator<T: Entity> {
    rules: Vec<Box<dyn ValidationRule<T>>>,
}

impl<T: Entity> GenericValidator<T> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    
    pub fn add_rule(mut self, rule: Box<dyn ValidationRule<T>>) -> Self {
        self.rules.push(rule);
        self
    }
}

impl<T: Entity> Validator<T> for GenericValidator<T> {
    fn validate(&self, entity: &T) -> Result<()> {
        // 首先执行derive(Validate)的验证
        entity.validate()?;
        
        // 然后执行自定义验证规则
        for rule in &self.rules {
            rule.validate(entity)?;
        }
        
        Ok(())
    }
}

/// 验证规则trait
pub trait ValidationRule<T>: Send + Sync {
    fn validate(&self, entity: &T) -> Result<()>;
}

/// 客户特定验证规则
pub struct CustomerUniquePhoneRule {
    repository: Arc<GenericRepository<Customer>>,
}

impl ValidationRule<Customer> for CustomerUniquePhoneRule {
    fn validate(&self, customer: &Customer) -> Result<()> {
        // 检查电话号码唯一性
        todo!("实现电话号码唯一性验证")
    }
}
```

### 7. 通用报表生成系统

```rust
/// 通用报表生成器
pub struct ReportGenerator<T: Entity> {
    template_engine: Arc<TemplateEngine>,
    data_source: Arc<dyn Repository<T>>,
}

impl<T: Entity> ReportGenerator<T> {
    pub async fn generate_excel_report(
        &self,
        template: &str,
        query: SearchQuery,
    ) -> Result<Vec<u8>> {
        // 获取数据
        let data = self.data_source.search(&query).await?;
        
        // 使用通用Excel生成逻辑
        let workbook = self.create_excel_workbook(&data, template).await?;
        
        Ok(workbook.to_bytes()?)
    }
    
    pub async fn generate_word_report(
        &self,
        template: &str,
        entity_id: T::Id,
    ) -> Result<Vec<u8>> {
        // 获取实体数据
        let entity = self.data_source.find_by_id(entity_id).await?
            .ok_or_else(|| Error::NotFound)?;
        
        // 使用通用Word生成逻辑
        let document = self.create_word_document(&entity, template).await?;
        
        Ok(document.to_bytes()?)
    }
}
```

## 模块化项目结构

```
src/
├── main.rs                     # 应用程序入口
├── lib.rs                      # 库入口
├── core/                       # 核心模块
│   ├── mod.rs
│   ├── entity.rs              # Entity trait定义
│   ├── repository.rs          # Repository trait定义
│   ├── service.rs             # Service trait定义
│   ├── error.rs               # 错误类型定义
│   └── types.rs               # 通用类型定义
├── infrastructure/             # 基础设施层
│   ├── mod.rs
│   ├── database/              # 数据库相关
│   │   ├── mod.rs
│   │   ├── pool.rs           # 连接池
│   │   ├── migrations.rs     # 数据库迁移
│   │   └── query_builder.rs  # 查询构建器
│   ├── repository/            # Repository实现
│   │   ├── mod.rs
│   │   ├── generic.rs        # 通用Repository
│   │   ├── customer.rs       # 客户Repository
│   │   └── supplier.rs       # 供应商Repository
│   └── external/              # 外部服务
│       ├── mod.rs
│       ├── document.rs       # 文档生成服务
│       └── notification.rs   # 通知服务
├── domain/                     # 领域层
│   ├── mod.rs
│   ├── entities/              # 实体定义
│   │   ├── mod.rs
│   │   ├── customer.rs       # 客户实体
│   │   ├── supplier.rs       # 供应商实体
│   │   ├── quote.rs          # 报价实体
│   │   └── contract.rs       # 合同实体
│   ├── services/              # 领域服务
│   │   ├── mod.rs
│   │   ├── customer_service.rs
│   │   ├── supplier_service.rs
│   │   ├── quote_service.rs
│   │   └── analytics_service.rs
│   └── validators/            # 验证器
│       ├── mod.rs
│       ├── customer_validator.rs
│       └── supplier_validator.rs
├── application/               # 应用层
│   ├── mod.rs
│   ├── commands/             # 命令处理
│   │   ├── mod.rs
│   │   ├── customer_commands.rs
│   │   └── supplier_commands.rs
│   ├── queries/              # 查询处理
│   │   ├── mod.rs
│   │   ├── customer_queries.rs
│   │   └── supplier_queries.rs
│   └── handlers/             # 事件处理
│       ├── mod.rs
│       └── notification_handler.rs
├── presentation/              # 表示层
│   ├── mod.rs
│   ├── ui/                   # Slint UI组件
│   │   ├── mod.rs
│   │   ├── components/       # 通用UI组件
│   │   │   ├── mod.rs
│   │   │   ├── generic_list.slint
│   │   │   ├── generic_form.slint
│   │   │   └── generic_chart.slint
│   │   ├── pages/            # 页面组件
│   │   │   ├── mod.rs
│   │   │   ├── dashboard.slint
│   │   │   ├── customer_page.slint
│   │   │   └── supplier_page.slint
│   │   └── main_window.slint # 主窗口
│   └── controllers/          # 控制器
│       ├── mod.rs
│       ├── customer_controller.rs
│       └── supplier_controller.rs
├── shared/                    # 共享模块
│   ├── mod.rs
│   ├── config.rs             # 配置管理
│   ├── logger.rs             # 日志系统
│   ├── utils.rs              # 工具函数
│   └── constants.rs          # 常量定义
└── tests/                     # 测试
    ├── integration/          # 集成测试
    ├── unit/                 # 单元测试
    └── common/               # 测试工具
```##
 代码复用策略

### 1. 宏系统实现代码生成

```rust
/// 自动生成Repository实现的宏
macro_rules! impl_repository {
    ($entity:ty, $table:expr) => {
        impl Repository<$entity> for GenericRepository<$entity> {
            async fn create(&self, entity: &mut $entity) -> Result<<$entity as Entity>::Id> {
                let conn = self.pool.get().await?;
                
                // 使用宏生成的SQL语句
                let sql = concat!("INSERT INTO ", $table, " (", 
                    stringify_fields!($entity), ") VALUES (", 
                    placeholder_fields!($entity), ")");
                
                // 执行插入
                let id = conn.execute(sql, entity_to_params!(entity)).await?;
                Ok(id)
            }
            
            // 其他方法的自动实现...
        }
    };
}

// 使用宏为具体实体生成Repository实现
impl_repository!(Customer, "customers");
impl_repository!(Supplier, "suppliers");
impl_repository!(Quote, "quotes");
impl_repository!(Contract, "contracts");
```

### 2. 通用UI控制器模式

```rust
/// 通用CRUD控制器
pub struct CrudController<T: Entity> {
    service: Arc<dyn Service<T>>,
    validator: Arc<dyn Validator<T>>,
    ui_adapter: Arc<dyn UiAdapter<T>>,
}

impl<T: Entity> CrudController<T> {
    pub async fn handle_create(&self, data: CreateRequest<T>) -> Result<T> {
        // 通用创建逻辑
        let mut entity = data.into_entity();
        self.validator.validate(&entity)?;
        let created = self.service.create(entity).await?;
        self.ui_adapter.notify_created(&created).await?;
        Ok(created)
    }
    
    pub async fn handle_update(&self, data: UpdateRequest<T>) -> Result<T> {
        // 通用更新逻辑
        let entity = data.into_entity();
        self.validator.validate(&entity)?;
        let updated = self.service.update(entity).await?;
        self.ui_adapter.notify_updated(&updated).await?;
        Ok(updated)
    }
    
    pub async fn handle_delete(&self, id: T::Id) -> Result<()> {
        // 通用删除逻辑
        self.service.delete(id).await?;
        self.ui_adapter.notify_deleted(id).await?;
        Ok(())
    }
    
    pub async fn handle_search(&self, query: SearchQuery) -> Result<SearchResult<T>> {
        // 通用搜索逻辑
        self.service.search(query).await
    }
}

/// UI适配器trait，用于UI通知
#[async_trait]
pub trait UiAdapter<T: Entity>: Send + Sync {
    async fn notify_created(&self, entity: &T) -> Result<()>;
    async fn notify_updated(&self, entity: &T) -> Result<()>;
    async fn notify_deleted(&self, id: T::Id) -> Result<()>;
}
```

### 3. 通用配置和工厂模式

```rust
/// 应用配置
#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub ui: UiConfig,
    pub templates: TemplateConfig,
    pub logging: LoggingConfig,
}

/// 服务工厂
pub struct ServiceFactory {
    config: AppConfig,
    database_pool: Arc<DatabasePool>,
}

impl ServiceFactory {
    pub fn new(config: AppConfig) -> Result<Self> {
        let database_pool = Arc::new(DatabasePool::new(&config.database)?);
        Ok(Self { config, database_pool })
    }
    
    /// 创建客户服务
    pub fn create_customer_service(&self) -> Arc<CustomerService> {
        let repository = Arc::new(GenericRepository::<Customer>::new(
            self.database_pool.clone()
        ));
        Arc::new(CustomerService::new(repository))
    }
    
    /// 创建供应商服务
    pub fn create_supplier_service(&self) -> Arc<SupplierService> {
        let repository = Arc::new(GenericRepository::<Supplier>::new(
            self.database_pool.clone()
        ));
        Arc::new(SupplierService::new(repository))
    }
    
    /// 通用服务创建方法
    pub fn create_service<T: Entity>(&self) -> Arc<GenericService<T>> 
    where
        GenericRepository<T>: Repository<T>,
    {
        let repository = Arc::new(GenericRepository::<T>::new(
            self.database_pool.clone()
        ));
        Arc::new(GenericService::new(repository))
    }
}
```

### 4. 事件驱动架构

```rust
/// 通用事件系统
#[derive(Clone, Debug)]
pub enum DomainEvent<T: Entity> {
    Created(T),
    Updated { old: T, new: T },
    Deleted(T::Id),
}

/// 事件处理器trait
#[async_trait]
pub trait EventHandler<T: Entity>: Send + Sync {
    async fn handle(&self, event: DomainEvent<T>) -> Result<()>;
}

/// 事件总线
pub struct EventBus<T: Entity> {
    handlers: Vec<Arc<dyn EventHandler<T>>>,
}

impl<T: Entity> EventBus<T> {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }
    
    pub fn subscribe(&mut self, handler: Arc<dyn EventHandler<T>>) {
        self.handlers.push(handler);
    }
    
    pub async fn publish(&self, event: DomainEvent<T>) -> Result<()> {
        for handler in &self.handlers {
            handler.handle(event.clone()).await?;
        }
        Ok(())
    }
}

/// 集成事件系统的服务
pub struct EventDrivenService<T: Entity> {
    inner_service: Arc<dyn Service<T>>,
    event_bus: Arc<EventBus<T>>,
}

impl<T: Entity> EventDrivenService<T> {
    pub async fn create(&self, entity: T) -> Result<T> {
        let created = self.inner_service.create(entity).await?;
        self.event_bus.publish(DomainEvent::Created(created.clone())).await?;
        Ok(created)
    }
    
    // 其他方法类似...
}
```

## 具体功能模块的复用实现

### 客户和供应商管理的统一实现

```rust
/// 联系人trait - 客户和供应商的共同特征
pub trait Contact: Entity {
    fn name(&self) -> &str;
    fn phone(&self) -> &str;
    fn email(&self) -> Option<&str>;
    fn level(&self) -> ContactLevel;
}

impl Contact for Customer {
    fn name(&self) -> &str { &self.name }
    fn phone(&self) -> &str { &self.phone }
    fn email(&self) -> Option<&str> { self.email.as_deref() }
    fn level(&self) -> ContactLevel { self.level.into() }
}

impl Contact for Supplier {
    fn name(&self) -> &str { &self.name }
    fn phone(&self) -> &str { &self.phone }
    fn email(&self) -> Option<&str> { self.email.as_deref() }
    fn level(&self) -> ContactLevel { self.level.into() }
}

/// 通用联系人服务
pub struct ContactService<T: Contact> {
    repository: Arc<dyn Repository<T>>,
    validator: Arc<dyn Validator<T>>,
}

impl<T: Contact> ContactService<T> {
    /// 通用搜索联系人方法
    pub async fn search_contacts(&self, keyword: &str) -> Result<Vec<T>> {
        let query = SearchQuery {
            keyword: Some(keyword.to_string()),
            filters: HashMap::new(),
            sort_by: Some("name".to_string()),
            sort_order: SortOrder::Asc,
            page: 0,
            page_size: 100,
        };
        
        let result = self.repository.search(&query).await?;
        Ok(result.items)
    }
    
    /// 通用联系人验证
    pub async fn validate_contact(&self, contact: &T) -> Result<()> {
        self.validator.validate(contact)
    }
}
```

### 报价和询价的统一实现

```rust
/// 报价trait - 客户报价和供应商询价的共同特征
pub trait Quotation: Entity {
    type ContactId;
    
    fn contact_id(&self) -> Self::ContactId;
    fn total_amount(&self) -> Decimal;
    fn valid_until(&self) -> chrono::DateTime<chrono::Utc>;
    fn items(&self) -> &[QuotationItem];
}

/// 通用报价服务
pub struct QuotationService<T: Quotation> {
    repository: Arc<dyn Repository<T>>,
    template_service: Arc<TemplateService>,
}

impl<T: Quotation> QuotationService<T> {
    /// 通用报价比较
    pub async fn compare_quotations(&self, ids: Vec<T::Id>) -> Result<QuotationComparison<T>> {
        let quotations = futures::future::try_join_all(
            ids.into_iter().map(|id| self.repository.find_by_id(id))
        ).await?;
        
        // 通用比较逻辑
        Ok(QuotationComparison::new(quotations.into_iter().flatten().collect()))
    }
    
    /// 通用文档生成
    pub async fn generate_document(&self, id: T::Id, template: &str) -> Result<Vec<u8>> {
        let quotation = self.repository.find_by_id(id).await?
            .ok_or_else(|| Error::NotFound)?;
        
        self.template_service.generate_document(&quotation, template).await
    }
}
```

这种模块化设计的优势：

1. **代码复用率高** - 通过trait和泛型实现最大化代码复用
2. **类型安全** - Rust的类型系统确保编译时安全
3. **易于扩展** - 新增实体只需实现相应trait
4. **测试友好** - 每个模块都可以独立测试
5. **维护性好** - 修改通用逻辑会自动应用到所有实现## 主界面
设计方案

### 整体布局架构

```
┌─────────────────────────────────────────────────────────────┐
│  MiniCRM - 板材行业客户管理系统           [🔄][⚙️][❓][×] │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────────┐ │
│ │  侧边导航   │ │              主内容区域                 │ │
│ │             │ │                                         │ │
│ │ 📊 仪表盘   │ │  ┌─────────────────────────────────┐   │ │
│ │ 👥 客户管理 │ │  │          动态内容区域           │   │ │
│ │ 🏭 供应商   │ │  │                                 │   │ │
│ │ 💰 财务管理 │ │  │  根据左侧导航显示不同页面       │   │ │
│ │ 📄 合同管理 │ │  │                                 │   │ │
│ │ 📊 报表分析 │ │  │  - 仪表盘：图表和关键指标       │   │ │
│ │ ⚙️  系统设置 │ │  │  - 客户管理：列表和详情         │   │ │
│ │             │ │  │  - 供应商：列表和详情           │   │ │
│ │ ────────── │ │  │  - 其他功能模块                 │   │ │
│ │ 🔍 全局搜索 │ │  │                                 │   │ │
│ │ 📋 快速操作 │ │  └─────────────────────────────────┘   │ │
│ │ 🔔 通知中心 │ │                                         │ │
│ └─────────────┘ └─────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  状态栏: 就绪 | 数据库: 正常 | 用户: 管理员 | 时间: 14:30   │
└─────────────────────────────────────────────────────────────┘
```

### Slint主界面实现

```slint
// main_window.slint
import { Button, VerticalBox, HorizontalBox, ListView, ScrollView } from "std-widgets.slint";
import { DashboardPage } from "pages/dashboard.slint";
import { CustomerPage } from "pages/customer.slint";
import { SupplierPage } from "pages/supplier.slint";

export component MainWindow inherits Window {
    title: "MiniCRM - 板材行业客户管理系统";
    width: 1280px;
    height: 800px;
    min-width: 1024px;
    min-height: 768px;
    
    // 状态属性
    in-out property <string> current-page: "dashboard";
    in-out property <bool> sidebar-collapsed: false;
    
    // 回调函数
    callback navigate-to(string);
    callback show-notification(string);
    callback perform-search(string);
    
    // 主布局
    HorizontalBox {
        // 侧边导航栏
        sidebar := Rectangle {
            width: sidebar-collapsed ? 60px : 240px;
            background: #f8f9fa;
            border-right: 1px solid #dee2e6;
            
            animate width { duration: 200ms; easing: ease-in-out; }
            
            VerticalBox {
                padding: 16px;
                spacing: 8px;
                
                // Logo区域
                logo-area := Rectangle {
                    height: 60px;
                    
                    HorizontalBox {
                        alignment: center;
                        
                        if !sidebar-collapsed: Text {
                            text: "MiniCRM";
                            font-size: 20px;
                            font-weight: 700;
                            color: #007bff;
                        }
                        
                        if sidebar-collapsed: Text {
                            text: "M";
                            font-size: 24px;
                            font-weight: 700;
                            color: #007bff;
                        }
                    }
                }
                
                // 导航菜单
                navigation-menu := VerticalBox {
                    spacing: 4px;
                    
                    // 仪表盘
                    nav-item-dashboard := Rectangle {
                        height: 44px;
                        background: current-page == "dashboard" ? #e3f2fd : transparent;
                        border-radius: 6px;
                        
                        HorizontalBox {
                            padding-left: 12px;
                            padding-right: 12px;
                            alignment: start;
                            
                            Text {
                                text: "📊";
                                font-size: 16px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: Text {
                                text: "仪表盘";
                                font-size: 14px;
                                color: current-page == "dashboard" ? #1976d2 : #495057;
                                vertical-alignment: center;
                            }
                        }
                        
                        TouchArea {
                            clicked => { navigate-to("dashboard"); }
                        }
                    }
                    
                    // 客户管理
                    nav-item-customers := Rectangle {
                        height: 44px;
                        background: current-page == "customers" ? #e8f5e8 : transparent;
                        border-radius: 6px;
                        
                        HorizontalBox {
                            padding-left: 12px;
                            padding-right: 12px;
                            alignment: start;
                            
                            Text {
                                text: "👥";
                                font-size: 16px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: Text {
                                text: "客户管理";
                                font-size: 14px;
                                color: current-page == "customers" ? #2e7d32 : #495057;
                                vertical-alignment: center;
                            }
                        }
                        
                        TouchArea {
                            clicked => { navigate-to("customers"); }
                        }
                    }
                    
                    // 供应商管理
                    nav-item-suppliers := Rectangle {
                        height: 44px;
                        background: current-page == "suppliers" ? #fff3e0 : transparent;
                        border-radius: 6px;
                        
                        HorizontalBox {
                            padding-left: 12px;
                            padding-right: 12px;
                            alignment: start;
                            
                            Text {
                                text: "🏭";
                                font-size: 16px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: Text {
                                text: "供应商";
                                font-size: 14px;
                                color: current-page == "suppliers" ? #f57c00 : #495057;
                                vertical-alignment: center;
                            }
                        }
                        
                        TouchArea {
                            clicked => { navigate-to("suppliers"); }
                        }
                    }
                    
                    // 财务管理
                    nav-item-finance := Rectangle {
                        height: 44px;
                        background: current-page == "finance" ? #f3e5f5 : transparent;
                        border-radius: 6px;
                        
                        HorizontalBox {
                            padding-left: 12px;
                            padding-right: 12px;
                            alignment: start;
                            
                            Text {
                                text: "💰";
                                font-size: 16px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: Text {
                                text: "财务管理";
                                font-size: 14px;
                                color: current-page == "finance" ? #7b1fa2 : #495057;
                                vertical-alignment: center;
                            }
                        }
                        
                        TouchArea {
                            clicked => { navigate-to("finance"); }
                        }
                    }
                    
                    // 合同管理
                    nav-item-contracts := Rectangle {
                        height: 44px;
                        background: current-page == "contracts" ? #e0f2f1 : transparent;
                        border-radius: 6px;
                        
                        HorizontalBox {
                            padding-left: 12px;
                            padding-right: 12px;
                            alignment: start;
                            
                            Text {
                                text: "📄";
                                font-size: 16px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: Text {
                                text: "合同管理";
                                font-size: 14px;
                                color: current-page == "contracts" ? #00695c : #495057;
                                vertical-alignment: center;
                            }
                        }
                        
                        TouchArea {
                            clicked => { navigate-to("contracts"); }
                        }
                    }
                    
                    // 报表分析
                    nav-item-reports := Rectangle {
                        height: 44px;
                        background: current-page == "reports" ? #fce4ec : transparent;
                        border-radius: 6px;
                        
                        HorizontalBox {
                            padding-left: 12px;
                            padding-right: 12px;
                            alignment: start;
                            
                            Text {
                                text: "📊";
                                font-size: 16px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: Text {
                                text: "报表分析";
                                font-size: 14px;
                                color: current-page == "reports" ? #c2185b : #495057;
                                vertical-alignment: center;
                            }
                        }
                        
                        TouchArea {
                            clicked => { navigate-to("reports"); }
                        }
                    }
                }
                
                // 分隔线
                Rectangle {
                    height: 1px;
                    background: #dee2e6;
                    margin-top: 16px;
                    margin-bottom: 16px;
                }
                
                // 快速功能区
                quick-actions := VerticalBox {
                    spacing: 8px;
                    
                    // 全局搜索
                    search-area := Rectangle {
                        height: 36px;
                        background: white;
                        border: 1px solid #ced4da;
                        border-radius: 4px;
                        
                        HorizontalBox {
                            padding: 8px;
                            
                            Text {
                                text: "🔍";
                                font-size: 14px;
                                vertical-alignment: center;
                            }
                            
                            if !sidebar-collapsed: LineEdit {
                                placeholder-text: "全局搜索...";
                                font-size: 12px;
                                border: none;
                                background: transparent;
                            }
                        }
                    }
                    
                    // 快速操作按钮
                    if !sidebar-collapsed: VerticalBox {
                        spacing: 4px;
                        
                        Button {
                            text: "➕ 新增客户";
                            height: 32px;
                            clicked => { navigate-to("customers/new"); }
                        }
                        
                        Button {
                            text: "📋 创建报价";
                            height: 32px;
                            clicked => { navigate-to("quotes/new"); }
                        }
                        
                        Button {
                            text: "📄 新建合同";
                            height: 32px;
                            clicked => { navigate-to("contracts/new"); }
                        }
                    }
                }
                
                // 底部区域
                Rectangle {
                    // 占位，推送内容到底部
                }
                
                // 通知中心
                notification-area := Rectangle {
                    height: 40px;
                    background: #fff3cd;
                    border: 1px solid #ffeaa7;
                    border-radius: 4px;
                    
                    HorizontalBox {
                        padding: 8px;
                        alignment: center;
                        
                        Text {
                            text: "🔔";
                            font-size: 14px;
                        }
                        
                        if !sidebar-collapsed: Text {
                            text: "3条新通知";
                            font-size: 12px;
                            color: #856404;
                        }
                    }
                    
                    TouchArea {
                        clicked => { navigate-to("notifications"); }
                    }
                }
                
                // 折叠按钮
                collapse-button := Rectangle {
                    height: 32px;
                    
                    Button {
                        text: sidebar-collapsed ? "▶" : "◀";
                        width: 100%;
                        clicked => { sidebar-collapsed = !sidebar-collapsed; }
                    }
                }
            }
        }
        
        // 主内容区域
        main-content := Rectangle {
            background: white;
            
            VerticalBox {
                // 顶部工具栏
                toolbar := Rectangle {
                    height: 56px;
                    background: #f8f9fa;
                    border-bottom: 1px solid #dee2e6;
                    
                    HorizontalBox {
                        padding: 16px;
                        alignment: space-between;
                        
                        // 页面标题
                        page-title := Text {
                            text: get-page-title(current-page);
                            font-size: 18px;
                            font-weight: 600;
                            color: #212529;
                            vertical-alignment: center;
                        }
                        
                        // 右侧工具按钮
                        HorizontalBox {
                            spacing: 8px;
                            
                            Button {
                                text: "🔄";
                                width: 36px;
                                height: 36px;
                                clicked => { /* 刷新当前页面 */ }
                            }
                            
                            Button {
                                text: "⚙️";
                                width: 36px;
                                height: 36px;
                                clicked => { navigate-to("settings"); }
                            }
                            
                            Button {
                                text: "❓";
                                width: 36px;
                                height: 36px;
                                clicked => { navigate-to("help"); }
                            }
                        }
                    }
                }
                
                // 动态页面内容
                page-content := Rectangle {
                    // 根据current-page显示不同的页面组件
                    if current-page == "dashboard": DashboardPage {
                        // 仪表盘页面
                    }
                    
                    if current-page == "customers": CustomerPage {
                        // 客户管理页面
                    }
                    
                    if current-page == "suppliers": SupplierPage {
                        // 供应商管理页面
                    }
                    
                    // 其他页面...
                }
            }
        }
    }
    
    // 状态栏
    status-bar := Rectangle {
        height: 24px;
        background: #e9ecef;
        border-top: 1px solid #dee2e6;
        
        HorizontalBox {
            padding-left: 16px;
            padding-right: 16px;
            alignment: space-between;
            
            HorizontalBox {
                spacing: 16px;
                
                Text {
                    text: "状态: 就绪";
                    font-size: 11px;
                    color: #6c757d;
                    vertical-alignment: center;
                }
                
                Text {
                    text: "数据库: 正常";
                    font-size: 11px;
                    color: #28a745;
                    vertical-alignment: center;
                }
            }
            
            HorizontalBox {
                spacing: 16px;
                
                Text {
                    text: "用户: 管理员";
                    font-size: 11px;
                    color: #6c757d;
                    vertical-alignment: center;
                }
                
                Text {
                    text: get-current-time();
                    font-size: 11px;
                    color: #6c757d;
                    vertical-alignment: center;
                }
            }
        }
    }
}

// 辅助函数
pure function get-page-title(page: string) -> string {
    if page == "dashboard" { "数据仪表盘" }
    else if page == "customers" { "客户管理" }
    else if page == "suppliers" { "供应商管理" }
    else if page == "finance" { "财务管理" }
    else if page == "contracts" { "合同管理" }
    else if page == "reports" { "报表分析" }
    else if page == "settings" { "系统设置" }
    else { "MiniCRM" }
}

pure function get-current-time() -> string {
    // 这里需要从Rust后端获取当前时间
    "14:30"
}
```

### 响应式设计和主题支持

```slint
// themes/theme.slint
export struct Theme {
    // 主色调
    primary: color,
    primary-light: color,
    primary-dark: color,
    
    // 背景色
    background: color,
    surface: color,
    
    // 文字颜色
    text-primary: color,
    text-secondary: color,
    
    // 边框和分割线
    border: color,
    divider: color,
    
    // 状态颜色
    success: color,
    warning: color,
    error: color,
    info: color,
}

export global AppTheme {
    // 浅色主题
    in-out property <Theme> light-theme: {
        primary: #007bff,
        primary-light: #e3f2fd,
        primary-dark: #1976d2,
        background: #ffffff,
        surface: #f8f9fa,
        text-primary: #212529,
        text-secondary: #6c757d,
        border: #dee2e6,
        divider: #e9ecef,
        success: #28a745,
        warning: #ffc107,
        error: #dc3545,
        info: #17a2b8,
    };
    
    // 深色主题
    in-out property <Theme> dark-theme: {
        primary: #4a9eff,
        primary-light: #1e3a5f,
        primary-dark: #0d47a1,
        background: #121212,
        surface: #1e1e1e,
        text-primary: #ffffff,
        text-secondary: #b0b0b0,
        border: #333333,
        divider: #2a2a2a,
        success: #4caf50,
        warning: #ff9800,
        error: #f44336,
        info: #2196f3,
    };
    
    // 当前主题
    in-out property <bool> is-dark-mode: false;
    in-out property <Theme> current-theme: is-dark-mode ? dark-theme : light-theme;
}
```

### 主界面控制器实现

```rust
// src/presentation/controllers/main_controller.rs
use slint::{ComponentHandle, Weak};
use std::sync::Arc;
use crate::application::services::*;

pub struct MainController {
    ui: Weak<MainWindow>,
    customer_service: Arc<CustomerService>,
    supplier_service: Arc<SupplierService>,
    notification_service: Arc<NotificationService>,
}

impl MainController {
    pub fn new(
        ui: Weak<MainWindow>,
        customer_service: Arc<CustomerService>,
        supplier_service: Arc<SupplierService>,
        notification_service: Arc<NotificationService>,
    ) -> Self {
        Self {
            ui,
            customer_service,
            supplier_service,
            notification_service,
        }
    }
    
    pub fn setup_callbacks(&self) {
        let ui = self.ui.upgrade().unwrap();
        
        // 导航回调
        let controller = self.clone();
        ui.on_navigate_to(move |page| {
            controller.handle_navigation(page.as_str());
        });
        
        // 搜索回调
        let controller = self.clone();
        ui.on_perform_search(move |query| {
            controller.handle_global_search(query.as_str());
        });
        
        // 通知回调
        let controller = self.clone();
        ui.on_show_notification(move |message| {
            controller.show_notification(message.as_str());
        });
    }
    
    fn handle_navigation(&self, page: &str) {
        let ui = self.ui.upgrade().unwrap();
        
        match page {
            "dashboard" => {
                ui.set_current_page("dashboard".into());
                self.load_dashboard_data();
            }
            "customers" => {
                ui.set_current_page("customers".into());
                self.load_customer_data();
            }
            "suppliers" => {
                ui.set_current_page("suppliers".into());
                self.load_supplier_data();
            }
            "customers/new" => {
                self.show_new_customer_dialog();
            }
            _ => {
                // 处理其他页面
            }
        }
    }
    
    fn handle_global_search(&self, query: &str) {
        // 实现全局搜索逻辑
        tokio::spawn(async move {
            // 搜索客户
            // 搜索供应商
            // 搜索合同
            // 显示搜索结果
        });
    }
    
    fn load_dashboard_data(&self) {
        let customer_service = self.customer_service.clone();
        let ui = self.ui.clone();
        
        tokio::spawn(async move {
            // 加载仪表盘数据
            let customer_count = customer_service.count().await.unwrap_or(0);
            
            if let Some(ui) = ui.upgrade() {
                // 更新UI数据
            }
        });
    }
}
```

这个主界面设计的特点：

1. **现代化布局** - 侧边导航 + 主内容区域的经典布局
2. **响应式设计** - 支持侧边栏折叠，适应不同屏幕尺寸
3. **主题支持** - 浅色/深色主题切换
4. **快速操作** - 侧边栏集成搜索和快速操作按钮
5. **状态显示** - 底部状态栏显示系统状态和时间
6. **模块化组件** - 每个页面都是独立的Slint组件
7. **统一交互** - 通过回调函数统一处理用户交互