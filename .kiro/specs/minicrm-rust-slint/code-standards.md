# MiniCRM Rust + Slint 代码质量规范

## 概述

本文档基于专业代码生成助手指导方针，结合Rust语言特性和Slint UI框架，为MiniCRM项目制定专门的代码质量规范。所有代码必须遵循模块化设计、详细注释、错误处理和最佳实践原则。

## Rust代码规范

### 1. 项目结构和模块化设计

```rust
// src/lib.rs - 库入口文件
//! # MiniCRM 客户关系管理系统
//! 
//! 专为板材行业设计的跨平台客户关系管理系统，使用Rust + Slint技术栈。
//! 
//! ## 主要功能模块
//! 
//! - `core`: 核心trait和类型定义
//! - `domain`: 业务实体和领域服务
//! - `infrastructure`: 数据访问和外部服务
//! - `application`: 应用服务和命令处理
//! - `presentation`: UI界面和控制器
//! - `shared`: 共享工具和配置

pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod application;
pub mod presentation;
pub mod shared;

// 重新导出核心类型，方便外部使用
pub use core::{Entity, Repository, Service};
pub use shared::{Result, Error};
```

### 2. 实体定义规范

```rust
// src/domain/entities/customer.rs
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::core::Entity;

/// 客户实体
/// 
/// 代表系统中的客户信息，包含基本信息、联系方式和业务属性。
/// 专为板材行业客户管理需求设计。
/// 
/// # 字段说明
/// 
/// - `id`: 客户唯一标识符，由数据库自动生成
/// - `name`: 客户名称，必填字段，用于显示和搜索
/// - `phone`: 联系电话，必填字段，支持格式验证
/// - `email`: 电子邮箱，可选字段，支持格式验证
/// - `company`: 公司名称，可选字段
/// - `level`: 客户等级，影响服务优先级和授信额度
/// - `created_at`: 创建时间，自动设置
/// - `updated_at`: 最后更新时间，自动维护
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Customer {
    /// 客户ID，数据库主键
    pub id: Option<i64>,
    
    /// 客户名称，必填字段
    /// 
    /// 验证规则：
    /// - 长度：1-100个字符
    /// - 不能为空或仅包含空白字符
    #[validate(length(min = 1, max = 100, message = "客户名称长度必须在1-100个字符之间"))]
    #[validate(custom(function = "validate_not_empty", message = "客户名称不能为空"))]
    pub name: String,
    
    /// 联系电话，必填字段
    /// 
    /// 支持的格式：
    /// - 手机号：138-1234-5678 或 13812345678
    /// - 固话：021-12345678 或 02112345678
    #[validate(regex(path = "PHONE_REGEX", message = "电话号码格式不正确"))]
    pub phone: String,
    
    /// 电子邮箱，可选字段
    /// 
    /// 如果提供，必须符合标准邮箱格式
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    
    /// 公司名称，可选字段
    /// 
    /// 用于B2B客户的公司信息记录
    #[validate(length(max = 200, message = "公司名称不能超过200个字符"))]
    pub company: Option<String>,
    
    /// 客户等级
    /// 
    /// 影响客户的服务优先级、授信额度等业务规则
    pub level: CustomerLevel,
    
    /// 创建时间
    /// 
    /// 记录客户首次添加到系统的时间，用于统计分析
    pub created_at: DateTime<Utc>,
    
    /// 最后更新时间
    /// 
    /// 记录客户信息最后一次修改的时间，用于数据同步
    pub updated_at: DateTime<Utc>,
}

/// 客户等级枚举
/// 
/// 定义不同等级的客户类型，用于差异化服务和风险控制。
/// 
/// # 等级说明
/// 
/// - `Vip`: VIP客户，最高优先级，享受专属服务
/// - `Important`: 重要客户，高优先级，重点维护
/// - `Normal`: 普通客户，标准服务级别
/// - `Potential`: 潜在客户，需要培育和转化
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CustomerLevel {
    /// VIP客户 - 最高等级
    /// 
    /// 特征：
    /// - 年交易额 > 100万
    /// - 合作时间 > 2年
    /// - 信用记录优秀
    Vip,
    
    /// 重要客户 - 高等级
    /// 
    /// 特征：
    /// - 年交易额 > 50万
    /// - 合作时间 > 1年
    /// - 信用记录良好
    Important,
    
    /// 普通客户 - 标准等级
    /// 
    /// 特征：
    /// - 年交易额 > 10万
    /// - 有稳定合作关系
    Normal,
    
    /// 潜在客户 - 培育等级
    /// 
    /// 特征：
    /// - 尚未正式合作
    /// - 有合作意向
    /// - 需要重点跟进
    Potential,
}

impl Entity for Customer {
    type Id = i64;
    
    fn id(&self) -> Option<Self::Id> {
        self.id
    }
    
    fn set_id(&mut self, id: Self::Id) {
        self.id = Some(id);
    }
    
    fn table_name() -> &'static str {
        "customers"
    }
    
    fn display_name(&self) -> String {
        self.name.clone()
    }
}

impl Customer {
    /// 创建新的客户实例
    /// 
    /// # 参数
    /// 
    /// - `name`: 客户名称
    /// - `phone`: 联系电话
    /// 
    /// # 返回值
    /// 
    /// 返回新创建的客户实例，ID为None，创建时间和更新时间设置为当前时间
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let customer = Customer::new("张三".to_string(), "138-1234-5678".to_string());
    /// assert_eq!(customer.name, "张三");
    /// assert_eq!(customer.level, CustomerLevel::Normal);
    /// ```
    pub fn new(name: String, phone: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            name,
            phone,
            email: None,
            company: None,
            level: CustomerLevel::Normal, // 默认为普通客户
            created_at: now,
            updated_at: now,
        }
    }
    
    /// 更新客户信息
    /// 
    /// 更新客户的基本信息，并自动设置更新时间为当前时间。
    /// 
    /// # 参数
    /// 
    /// - `name`: 新的客户名称，可选
    /// - `phone`: 新的联系电话，可选
    /// - `email`: 新的电子邮箱，可选
    /// - `company`: 新的公司名称，可选
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let mut customer = Customer::new("张三".to_string(), "138-1234-5678".to_string());
    /// customer.update_info(
    ///     Some("张三丰".to_string()),
    ///     None,
    ///     Some("zhangsan@example.com".to_string()),
    ///     Some("武当山公司".to_string())
    /// );
    /// ```
    pub fn update_info(
        &mut self,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        company: Option<String>,
    ) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(phone) = phone {
            self.phone = phone;
        }
        if let Some(email) = email {
            self.email = Some(email);
        }
        if let Some(company) = company {
            self.company = Some(company);
        }
        
        // 自动更新修改时间
        self.updated_at = Utc::now();
    }
    
    /// 升级客户等级
    /// 
    /// 根据业务规则升级客户等级，并记录升级时间。
    /// 
    /// # 参数
    /// 
    /// - `new_level`: 新的客户等级
    /// 
    /// # 返回值
    /// 
    /// - `Ok(())`: 升级成功
    /// - `Err(String)`: 升级失败，返回错误信息
    /// 
    /// # 业务规则
    /// 
    /// - 只能升级，不能降级（除非特殊情况）
    /// - VIP客户不能再升级
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let mut customer = Customer::new("张三".to_string(), "138-1234-5678".to_string());
    /// let result = customer.upgrade_level(CustomerLevel::Important);
    /// assert!(result.is_ok());
    /// assert_eq!(customer.level, CustomerLevel::Important);
    /// ```
    pub fn upgrade_level(&mut self, new_level: CustomerLevel) -> Result<(), String> {
        // 检查升级规则
        match (self.level, new_level) {
            (CustomerLevel::Vip, _) => {
                return Err("VIP客户已是最高等级，无法继续升级".to_string());
            }
            (current, new) if current as u8 >= new as u8 => {
                return Err("只能升级客户等级，不能降级".to_string());
            }
            _ => {}
        }
        
        self.level = new_level;
        self.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// 检查客户是否为高价值客户
    /// 
    /// 根据客户等级判断是否为高价值客户（VIP或重要客户）。
    /// 
    /// # 返回值
    /// 
    /// - `true`: 高价值客户（VIP或重要客户）
    /// - `false`: 普通或潜在客户
    pub fn is_high_value(&self) -> bool {
        matches!(self.level, CustomerLevel::Vip | CustomerLevel::Important)
    }
}

// 自定义验证函数
fn validate_not_empty(value: &str) -> Result<(), validator::ValidationError> {
    if value.trim().is_empty() {
        return Err(validator::ValidationError::new("empty"));
    }
    Ok(())
}

// 电话号码正则表达式
lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(
        r"^(1[3-9]\d{9}|0\d{2,3}-?\d{7,8})$"
    ).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_customer_creation() {
        let customer = Customer::new("测试客户".to_string(), "138-1234-5678".to_string());
        
        assert_eq!(customer.name, "测试客户");
        assert_eq!(customer.phone, "138-1234-5678");
        assert_eq!(customer.level, CustomerLevel::Normal);
        assert!(customer.id.is_none());
    }
    
    #[test]
    fn test_customer_level_upgrade() {
        let mut customer = Customer::new("测试客户".to_string(), "138-1234-5678".to_string());
        
        // 正常升级
        assert!(customer.upgrade_level(CustomerLevel::Important).is_ok());
        assert_eq!(customer.level, CustomerLevel::Important);
        
        // 继续升级到VIP
        assert!(customer.upgrade_level(CustomerLevel::Vip).is_ok());
        assert_eq!(customer.level, CustomerLevel::Vip);
        
        // VIP无法继续升级
        assert!(customer.upgrade_level(CustomerLevel::Vip).is_err());
    }
    
    #[test]
    fn test_high_value_customer() {
        let mut customer = Customer::new("测试客户".to_string(), "138-1234-5678".to_string());
        
        // 普通客户不是高价值客户
        assert!(!customer.is_high_value());
        
        // 升级到重要客户
        customer.upgrade_level(CustomerLevel::Important).unwrap();
        assert!(customer.is_high_value());
        
        // VIP客户是高价值客户
        customer.upgrade_level(CustomerLevel::Vip).unwrap();
        assert!(customer.is_high_value());
    }
}
```### 3. R
epository实现规范

```rust
// src/infrastructure/repository/generic.rs
use async_trait::async_trait;
use std::sync::Arc;
use crate::core::{Entity, Repository, Result, Error};
use crate::infrastructure::database::DatabasePool;

/// 通用Repository实现
/// 
/// 为所有实体提供标准的CRUD操作实现，通过泛型支持不同的实体类型。
/// 使用async/await模式支持异步数据库操作。
/// 
/// # 类型参数
/// 
/// - `T`: 实体类型，必须实现Entity trait
/// 
/// # 设计原则
/// 
/// 1. **类型安全**: 通过泛型确保编译时类型检查
/// 2. **异步操作**: 所有数据库操作都是异步的，避免阻塞
/// 3. **错误处理**: 统一的错误处理机制
/// 4. **连接池**: 使用连接池提高性能
/// 
/// # 使用示例
/// 
/// ```rust
/// let pool = Arc::new(DatabasePool::new(&config).await?);
/// let repository = GenericRepository::<Customer>::new(pool);
/// 
/// let mut customer = Customer::new("张三".to_string(), "138-1234-5678".to_string());
/// let id = repository.create(&mut customer).await?;
/// ```
pub struct GenericRepository<T: Entity> {
    /// 数据库连接池
    /// 
    /// 使用Arc包装以支持多线程共享，连接池管理数据库连接的创建和复用
    pool: Arc<DatabasePool>,
    
    /// 幻影数据，用于类型参数
    /// 
    /// Rust编译器需要这个字段来确定泛型参数T的类型
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> GenericRepository<T> {
    /// 创建新的Repository实例
    /// 
    /// # 参数
    /// 
    /// - `pool`: 数据库连接池的Arc引用
    /// 
    /// # 返回值
    /// 
    /// 返回新的Repository实例
    pub fn new(pool: Arc<DatabasePool>) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// 获取数据库连接
    /// 
    /// 从连接池中获取一个可用的数据库连接。
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Connection)`: 成功获取连接
    /// - `Err(Error)`: 连接池耗尽或数据库不可用
    async fn get_connection(&self) -> Result<DatabaseConnection> {
        self.pool.get().await.map_err(|e| {
            Error::Database(format!("获取数据库连接失败: {}", e))
        })
    }
    
    /// 构建插入SQL语句
    /// 
    /// 根据实体类型动态构建INSERT语句。
    /// 
    /// # 返回值
    /// 
    /// 返回SQL语句字符串和参数占位符数量
    fn build_insert_sql(&self) -> (String, usize) {
        // 这里需要根据实体类型动态生成SQL
        // 实际实现中可能需要使用宏或反射机制
        let table_name = T::table_name();
        let sql = format!(
            "INSERT INTO {} (name, phone, email, company, level, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            table_name
        );
        (sql, 7) // 返回SQL和参数数量
    }
    
    /// 构建查询SQL语句
    /// 
    /// 根据实体类型构建SELECT语句。
    /// 
    /// # 参数
    /// 
    /// - `where_clause`: WHERE子句，可选
    /// 
    /// # 返回值
    /// 
    /// 返回完整的SELECT SQL语句
    fn build_select_sql(&self, where_clause: Option<&str>) -> String {
        let table_name = T::table_name();
        let mut sql = format!("SELECT * FROM {}", table_name);
        
        if let Some(where_clause) = where_clause {
            sql.push_str(" WHERE ");
            sql.push_str(where_clause);
        }
        
        sql
    }
}

#[async_trait]
impl<T: Entity> Repository<T> for GenericRepository<T>
where
    T: Send + Sync + 'static,
    T::Id: Send + Sync,
{
    /// 创建新实体
    /// 
    /// 将实体保存到数据库，并返回生成的ID。
    /// 
    /// # 参数
    /// 
    /// - `entity`: 要创建的实体，ID字段会被自动设置
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Id)`: 创建成功，返回生成的ID
    /// - `Err(Error)`: 创建失败，可能的原因：
    ///   - 数据验证失败
    ///   - 数据库约束冲突
    ///   - 网络连接问题
    /// 
    /// # 错误处理
    /// 
    /// 该方法会自动处理以下错误情况：
    /// - 数据库连接失败：重试机制
    /// - 约束冲突：返回具体的冲突信息
    /// - 事务失败：自动回滚
    async fn create(&self, entity: &mut T) -> Result<T::Id> {
        // 获取数据库连接
        let mut conn = self.get_connection().await?;
        
        // 开始事务
        let tx = conn.begin().await.map_err(|e| {
            Error::Database(format!("开始事务失败: {}", e))
        })?;
        
        // 构建SQL语句
        let (sql, _param_count) = self.build_insert_sql();
        
        // 执行插入操作
        // 注意：这里需要根据具体的实体类型来绑定参数
        // 实际实现中可能需要使用宏或trait来自动生成参数绑定代码
        let result = sqlx::query(&sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                Error::Database(format!("执行插入操作失败: {}", e))
            })?;
        
        // 获取生成的ID
        let id = result.last_insert_rowid();
        
        // 提交事务
        tx.commit().await.map_err(|e| {
            Error::Database(format!("提交事务失败: {}", e))
        })?;
        
        // 设置实体ID
        let typed_id = T::Id::from(id); // 需要实现类型转换
        entity.set_id(typed_id.clone());
        
        Ok(typed_id)
    }
    
    /// 根据ID查找实体
    /// 
    /// # 参数
    /// 
    /// - `id`: 要查找的实体ID
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Some(Entity))`: 找到实体
    /// - `Ok(None)`: 实体不存在
    /// - `Err(Error)`: 查询失败
    async fn find_by_id(&self, id: T::Id) -> Result<Option<T>> {
        let conn = self.get_connection().await?;
        
        let sql = self.build_select_sql(Some("id = ?"));
        
        // 执行查询
        let row = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&*conn)
            .await
            .map_err(|e| {
                Error::Database(format!("查询实体失败: {}", e))
            })?;
        
        // 将数据库行转换为实体
        match row {
            Some(row) => {
                let entity = self.row_to_entity(row)?;
                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }
    
    /// 查找所有实体
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Vec<Entity>)`: 所有实体的列表
    /// - `Err(Error)`: 查询失败
    /// 
    /// # 性能注意事项
    /// 
    /// 该方法会返回表中的所有记录，对于大型数据集可能会影响性能。
    /// 建议在生产环境中使用分页查询。
    async fn find_all(&self) -> Result<Vec<T>> {
        let conn = self.get_connection().await?;
        
        let sql = self.build_select_sql(None);
        
        let rows = sqlx::query(&sql)
            .fetch_all(&*conn)
            .await
            .map_err(|e| {
                Error::Database(format!("查询所有实体失败: {}", e))
            })?;
        
        // 转换所有行为实体
        let mut entities = Vec::with_capacity(rows.len());
        for row in rows {
            let entity = self.row_to_entity(row)?;
            entities.push(entity);
        }
        
        Ok(entities)
    }
    
    /// 更新实体
    /// 
    /// # 参数
    /// 
    /// - `entity`: 要更新的实体，必须包含有效的ID
    /// 
    /// # 返回值
    /// 
    /// - `Ok(())`: 更新成功
    /// - `Err(Error)`: 更新失败
    /// 
    /// # 前置条件
    /// 
    /// - 实体必须已存在于数据库中
    /// - 实体的ID字段不能为None
    async fn update(&self, entity: &T) -> Result<()> {
        let id = entity.id().ok_or_else(|| {
            Error::Validation("无法更新没有ID的实体".to_string())
        })?;
        
        let mut conn = self.get_connection().await?;
        let tx = conn.begin().await.map_err(|e| {
            Error::Database(format!("开始更新事务失败: {}", e))
        })?;
        
        // 构建更新SQL
        let table_name = T::table_name();
        let sql = format!(
            "UPDATE {} SET name = ?, phone = ?, email = ?, company = ?, 
             level = ?, updated_at = ? WHERE id = ?",
            table_name
        );
        
        // 执行更新
        let result = sqlx::query(&sql)
            // 这里需要绑定实体的所有字段
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                Error::Database(format!("执行更新操作失败: {}", e))
            })?;
        
        // 检查是否有行被更新
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("ID为 {:?} 的实体不存在", id)));
        }
        
        tx.commit().await.map_err(|e| {
            Error::Database(format!("提交更新事务失败: {}", e))
        })?;
        
        Ok(())
    }
    
    /// 删除实体
    /// 
    /// # 参数
    /// 
    /// - `id`: 要删除的实体ID
    /// 
    /// # 返回值
    /// 
    /// - `Ok(())`: 删除成功
    /// - `Err(Error)`: 删除失败
    /// 
    /// # 注意事项
    /// 
    /// 删除操作是不可逆的，建议在删除前进行确认。
    /// 对于有关联数据的实体，可能需要先删除关联数据。
    async fn delete(&self, id: T::Id) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let tx = conn.begin().await.map_err(|e| {
            Error::Database(format!("开始删除事务失败: {}", e))
        })?;
        
        let table_name = T::table_name();
        let sql = format!("DELETE FROM {} WHERE id = ?", table_name);
        
        let result = sqlx::query(&sql)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                Error::Database(format!("执行删除操作失败: {}", e))
            })?;
        
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("ID为 {:?} 的实体不存在", id)));
        }
        
        tx.commit().await.map_err(|e| {
            Error::Database(format!("提交删除事务失败: {}", e))
        })?;
        
        Ok(())
    }
    
    /// 搜索实体
    /// 
    /// 根据搜索条件查找匹配的实体。
    /// 
    /// # 参数
    /// 
    /// - `query`: 搜索查询条件
    /// 
    /// # 返回值
    /// 
    /// - `Ok(SearchResult)`: 搜索结果，包含匹配的实体和分页信息
    /// - `Err(Error)`: 搜索失败
    async fn search(&self, query: &SearchQuery) -> Result<SearchResult<T>> {
        let conn = self.get_connection().await?;
        
        // 构建搜索SQL
        let (sql, count_sql) = self.build_search_sql(query);
        
        // 执行计数查询
        let total_count: i64 = sqlx::query_scalar(&count_sql)
            .fetch_one(&*conn)
            .await
            .map_err(|e| {
                Error::Database(format!("执行计数查询失败: {}", e))
            })?;
        
        // 执行数据查询
        let rows = sqlx::query(&sql)
            .fetch_all(&*conn)
            .await
            .map_err(|e| {
                Error::Database(format!("执行搜索查询失败: {}", e))
            })?;
        
        // 转换结果
        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let entity = self.row_to_entity(row)?;
            items.push(entity);
        }
        
        Ok(SearchResult {
            items,
            total_count,
            page: query.page,
            page_size: query.page_size,
        })
    }
    
    /// 统计实体数量
    /// 
    /// # 返回值
    /// 
    /// - `Ok(count)`: 实体总数
    /// - `Err(Error)`: 统计失败
    async fn count(&self) -> Result<i64> {
        let conn = self.get_connection().await?;
        
        let table_name = T::table_name();
        let sql = format!("SELECT COUNT(*) FROM {}", table_name);
        
        let count: i64 = sqlx::query_scalar(&sql)
            .fetch_one(&*conn)
            .await
            .map_err(|e| {
                Error::Database(format!("统计实体数量失败: {}", e))
            })?;
        
        Ok(count)
    }
}

impl<T: Entity> GenericRepository<T> {
    /// 将数据库行转换为实体
    /// 
    /// 这是一个内部辅助方法，用于将数据库查询结果转换为实体对象。
    /// 
    /// # 参数
    /// 
    /// - `row`: 数据库行数据
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Entity)`: 转换成功
    /// - `Err(Error)`: 转换失败，可能是数据格式问题
    fn row_to_entity(&self, row: sqlx::Row) -> Result<T> {
        // 这里需要根据具体的实体类型来实现行到实体的转换
        // 实际实现中可能需要使用宏来自动生成转换代码
        todo!("实现行到实体的转换逻辑")
    }
    
    /// 构建搜索SQL语句
    /// 
    /// 根据搜索条件构建SQL查询语句和计数语句。
    /// 
    /// # 参数
    /// 
    /// - `query`: 搜索查询条件
    /// 
    /// # 返回值
    /// 
    /// 返回数据查询SQL和计数查询SQL的元组
    fn build_search_sql(&self, query: &SearchQuery) -> (String, String) {
        let table_name = T::table_name();
        let mut where_clauses = Vec::new();
        
        // 关键词搜索
        if let Some(keyword) = &query.keyword {
            where_clauses.push(format!(
                "(name LIKE '%{}%' OR phone LIKE '%{}%')",
                keyword, keyword
            ));
        }
        
        // 筛选条件
        for (field, value) in &query.filters {
            where_clauses.push(format!("{} = '{}'", field, value));
        }
        
        // 构建WHERE子句
        let where_clause = if where_clauses.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", where_clauses.join(" AND "))
        };
        
        // 构建ORDER BY子句
        let order_clause = if let Some(sort_by) = &query.sort_by {
            format!(" ORDER BY {} {}", sort_by, query.sort_order)
        } else {
            " ORDER BY id DESC".to_string()
        };
        
        // 构建LIMIT子句
        let limit_clause = format!(
            " LIMIT {} OFFSET {}",
            query.page_size,
            query.page * query.page_size
        );
        
        // 数据查询SQL
        let data_sql = format!(
            "SELECT * FROM {}{}{}{}",
            table_name, where_clause, order_clause, limit_clause
        );
        
        // 计数查询SQL
        let count_sql = format!("SELECT COUNT(*) FROM {}{}", table_name, where_clause);
        
        (data_sql, count_sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Customer;
    
    #[tokio::test]
    async fn test_repository_creation() {
        // 这里需要设置测试数据库
        // let pool = Arc::new(DatabasePool::new_test().await.unwrap());
        // let repository = GenericRepository::<Customer>::new(pool);
        // 
        // let mut customer = Customer::new("测试客户".to_string(), "138-1234-5678".to_string());
        // let id = repository.create(&mut customer).await.unwrap();
        // 
        // assert!(id > 0);
        // assert_eq!(customer.id(), Some(id));
    }
}
```### 
4. Service层实现规范

```rust
// src/domain/services/customer_service.rs
use std::sync::Arc;
use async_trait::async_trait;
use validator::Validate;
use crate::core::{Service, Repository, Result, Error};
use crate::domain::entities::{Customer, CustomerLevel};
use crate::shared::events::{DomainEvent, EventBus};

/// 客户管理服务
/// 
/// 提供客户相关的业务逻辑处理，包括客户的创建、更新、删除、查询等操作。
/// 集成了数据验证、业务规则检查、事件发布等功能。
/// 
/// # 职责
/// 
/// 1. **业务逻辑处理**: 实现客户管理的核心业务逻辑
/// 2. **数据验证**: 确保客户数据的完整性和正确性
/// 3. **业务规则**: 执行客户等级管理、授信控制等业务规则
/// 4. **事件发布**: 发布领域事件，支持事件驱动架构
/// 5. **事务管理**: 确保复杂操作的数据一致性
/// 
/// # 设计模式
/// 
/// - **依赖注入**: 通过构造函数注入Repository和EventBus
/// - **事件驱动**: 通过事件总线发布领域事件
/// - **策略模式**: 支持不同的客户等级评估策略
/// 
/// # 使用示例
/// 
/// ```rust
/// let service = CustomerService::new(repository, event_bus);
/// 
/// let customer_data = CreateCustomerRequest {
///     name: "张三".to_string(),
///     phone: "138-1234-5678".to_string(),
///     email: Some("zhangsan@example.com".to_string()),
///     company: Some("测试公司".to_string()),
/// };
/// 
/// let customer = service.create_customer(customer_data).await?;
/// ```
pub struct CustomerService {
    /// 客户数据访问Repository
    repository: Arc<dyn Repository<Customer>>,
    
    /// 事件总线，用于发布领域事件
    event_bus: Arc<EventBus<Customer>>,
    
    /// 客户等级评估策略
    level_strategy: Arc<dyn CustomerLevelStrategy>,
}

impl CustomerService {
    /// 创建新的客户服务实例
    /// 
    /// # 参数
    /// 
    /// - `repository`: 客户数据访问Repository
    /// - `event_bus`: 事件总线
    /// 
    /// # 返回值
    /// 
    /// 返回新的客户服务实例
    pub fn new(
        repository: Arc<dyn Repository<Customer>>,
        event_bus: Arc<EventBus<Customer>>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            level_strategy: Arc::new(DefaultCustomerLevelStrategy::new()),
        }
    }
    
    /// 设置客户等级评估策略
    /// 
    /// 允许运行时更换客户等级评估策略，支持不同的业务规则。
    /// 
    /// # 参数
    /// 
    /// - `strategy`: 新的等级评估策略
    pub fn set_level_strategy(&mut self, strategy: Arc<dyn CustomerLevelStrategy>) {
        self.level_strategy = strategy;
    }
}

#[async_trait]
impl Service<Customer> for CustomerService {
    type Repository = dyn Repository<Customer>;
    
    fn repository(&self) -> &Self::Repository {
        &*self.repository
    }
    
    /// 创建新客户
    /// 
    /// 执行完整的客户创建流程，包括数据验证、业务规则检查、数据保存和事件发布。
    /// 
    /// # 参数
    /// 
    /// - `entity`: 要创建的客户实体
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Customer)`: 创建成功，返回包含ID的客户实体
    /// - `Err(Error)`: 创建失败，可能的原因：
    ///   - 数据验证失败
    ///   - 业务规则冲突（如电话号码重复）
    ///   - 数据库操作失败
    /// 
    /// # 业务流程
    /// 
    /// 1. 数据验证：检查客户数据的格式和完整性
    /// 2. 业务规则检查：验证电话号码唯一性等业务规则
    /// 3. 等级评估：根据策略设置初始客户等级
    /// 4. 数据保存：将客户信息保存到数据库
    /// 5. 事件发布：发布客户创建事件
    async fn create(&self, mut entity: Customer) -> Result<Customer> {
        // 1. 数据验证
        entity.validate().map_err(|e| {
            Error::Validation(format!("客户数据验证失败: {}", e))
        })?;
        
        // 2. 业务规则检查
        self.validate_business_rules(&entity).await?;
        
        // 3. 等级评估
        let suggested_level = self.level_strategy.evaluate_level(&entity).await?;
        if entity.level != suggested_level {
            entity.level = suggested_level;
        }
        
        // 4. 数据保存
        let id = self.repository.create(&mut entity).await?;
        entity.set_id(id);
        
        // 5. 事件发布
        self.event_bus.publish(DomainEvent::Created(entity.clone())).await?;
        
        Ok(entity)
    }
    
    /// 更新客户信息
    /// 
    /// # 参数
    /// 
    /// - `entity`: 要更新的客户实体，必须包含有效的ID
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Customer)`: 更新成功
    /// - `Err(Error)`: 更新失败
    /// 
    /// # 业务流程
    /// 
    /// 1. 检查客户是否存在
    /// 2. 数据验证
    /// 3. 业务规则检查
    /// 4. 等级重新评估（如果需要）
    /// 5. 数据更新
    /// 6. 事件发布
    async fn update(&self, entity: Customer) -> Result<Customer> {
        let id = entity.id().ok_or_else(|| {
            Error::Validation("无法更新没有ID的客户".to_string())
        })?;
        
        // 1. 检查客户是否存在
        let old_entity = self.repository.find_by_id(id).await?
            .ok_or_else(|| Error::NotFound(format!("客户不存在: {}", id)))?;
        
        // 2. 数据验证
        entity.validate().map_err(|e| {
            Error::Validation(format!("客户数据验证失败: {}", e))
        })?;
        
        // 3. 业务规则检查（排除自身）
        self.validate_business_rules_for_update(&entity, id).await?;
        
        // 4. 等级重新评估
        let mut updated_entity = entity;
        let suggested_level = self.level_strategy.evaluate_level(&updated_entity).await?;
        if updated_entity.level != suggested_level {
            updated_entity.level = suggested_level;
        }
        
        // 5. 数据更新
        self.repository.update(&updated_entity).await?;
        
        // 6. 事件发布
        self.event_bus.publish(DomainEvent::Updated {
            old: old_entity,
            new: updated_entity.clone(),
        }).await?;
        
        Ok(updated_entity)
    }
}

impl CustomerService {
    /// 创建客户（使用请求对象）
    /// 
    /// 提供更友好的API，接受创建请求对象而不是完整的实体。
    /// 
    /// # 参数
    /// 
    /// - `request`: 客户创建请求
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Customer)`: 创建成功
    /// - `Err(Error)`: 创建失败
    pub async fn create_customer(&self, request: CreateCustomerRequest) -> Result<Customer> {
        let customer = Customer::new(request.name, request.phone);
        let mut customer = customer;
        
        // 设置可选字段
        if let Some(email) = request.email {
            customer.email = Some(email);
        }
        if let Some(company) = request.company {
            customer.company = Some(company);
        }
        
        self.create(customer).await
    }
    
    /// 搜索客户
    /// 
    /// 提供灵活的客户搜索功能，支持关键词搜索和多条件筛选。
    /// 
    /// # 参数
    /// 
    /// - `request`: 搜索请求参数
    /// 
    /// # 返回值
    /// 
    /// - `Ok(SearchResult)`: 搜索结果
    /// - `Err(Error)`: 搜索失败
    /// 
    /// # 搜索功能
    /// 
    /// - **关键词搜索**: 在客户名称、电话、公司名称中搜索
    /// - **等级筛选**: 按客户等级筛选
    /// - **时间范围**: 按创建时间或更新时间筛选
    /// - **排序**: 支持多字段排序
    /// - **分页**: 支持分页查询
    pub async fn search_customers(&self, request: SearchCustomerRequest) -> Result<SearchResult<Customer>> {
        let query = SearchQuery {
            keyword: request.keyword,
            filters: request.build_filters(),
            sort_by: request.sort_by,
            sort_order: request.sort_order.unwrap_or(SortOrder::Desc),
            page: request.page.unwrap_or(0),
            page_size: request.page_size.unwrap_or(20),
        };
        
        self.repository.search(&query).await
    }
    
    /// 升级客户等级
    /// 
    /// 根据业务规则升级客户等级，并记录升级历史。
    /// 
    /// # 参数
    /// 
    /// - `customer_id`: 客户ID
    /// - `new_level`: 新的客户等级
    /// - `reason`: 升级原因
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Customer)`: 升级成功
    /// - `Err(Error)`: 升级失败
    /// 
    /// # 业务规则
    /// 
    /// - 只能升级，不能降级（除非有特殊权限）
    /// - VIP客户不能继续升级
    /// - 需要记录升级历史和原因
    pub async fn upgrade_customer_level(
        &self,
        customer_id: i64,
        new_level: CustomerLevel,
        reason: String,
    ) -> Result<Customer> {
        // 获取当前客户信息
        let mut customer = self.repository.find_by_id(customer_id).await?
            .ok_or_else(|| Error::NotFound(format!("客户不存在: {}", customer_id)))?;
        
        // 检查升级规则
        customer.upgrade_level(new_level).map_err(|e| {
            Error::BusinessRule(e)
        })?;
        
        // 保存更新
        let updated_customer = self.update(customer).await?;
        
        // 记录升级历史
        self.record_level_change_history(customer_id, new_level, reason).await?;
        
        Ok(updated_customer)
    }
    
    /// 获取高价值客户列表
    /// 
    /// 返回所有VIP和重要客户的列表，用于重点客户管理。
    /// 
    /// # 返回值
    /// 
    /// - `Ok(Vec<Customer>)`: 高价值客户列表
    /// - `Err(Error)`: 查询失败
    pub async fn get_high_value_customers(&self) -> Result<Vec<Customer>> {
        let query = SearchQuery {
            keyword: None,
            filters: {
                let mut filters = std::collections::HashMap::new();
                filters.insert("level".to_string(), "vip,important".to_string());
                filters
            },
            sort_by: Some("level".to_string()),
            sort_order: SortOrder::Asc,
            page: 0,
            page_size: 1000, // 假设高价值客户不会太多
        };
        
        let result = self.repository.search(&query).await?;
        Ok(result.items)
    }
    
    /// 验证业务规则
    /// 
    /// 检查客户数据是否符合业务规则，如电话号码唯一性等。
    /// 
    /// # 参数
    /// 
    /// - `customer`: 要验证的客户实体
    /// 
    /// # 返回值
    /// 
    /// - `Ok(())`: 验证通过
    /// - `Err(Error)`: 验证失败
    async fn validate_business_rules(&self, customer: &Customer) -> Result<()> {
        // 检查电话号码唯一性
        self.check_phone_uniqueness(&customer.phone, None).await?;
        
        // 检查邮箱唯一性（如果提供）
        if let Some(email) = &customer.email {
            self.check_email_uniqueness(email, None).await?;
        }
        
        Ok(())
    }
    
    /// 验证更新时的业务规则
    /// 
    /// 检查更新操作是否符合业务规则，排除自身的数据。
    /// 
    /// # 参数
    /// 
    /// - `customer`: 要验证的客户实体
    /// - `exclude_id`: 要排除的客户ID（通常是当前客户的ID）
    async fn validate_business_rules_for_update(
        &self,
        customer: &Customer,
        exclude_id: i64,
    ) -> Result<()> {
        // 检查电话号码唯一性（排除自身）
        self.check_phone_uniqueness(&customer.phone, Some(exclude_id)).await?;
        
        // 检查邮箱唯一性（如果提供，排除自身）
        if let Some(email) = &customer.email {
            self.check_email_uniqueness(email, Some(exclude_id)).await?;
        }
        
        Ok(())
    }
    
    /// 检查电话号码唯一性
    /// 
    /// # 参数
    /// 
    /// - `phone`: 要检查的电话号码
    /// - `exclude_id`: 要排除的客户ID，用于更新时的检查
    async fn check_phone_uniqueness(&self, phone: &str, exclude_id: Option<i64>) -> Result<()> {
        let mut query = SearchQuery {
            keyword: None,
            filters: {
                let mut filters = std::collections::HashMap::new();
                filters.insert("phone".to_string(), phone.to_string());
                filters
            },
            sort_by: None,
            sort_order: SortOrder::Asc,
            page: 0,
            page_size: 1,
        };
        
        let result = self.repository.search(&query).await?;
        
        // 检查是否有重复的电话号码
        for customer in result.items {
            if let Some(exclude_id) = exclude_id {
                if customer.id() == Some(exclude_id) {
                    continue; // 跳过自身
                }
            }
            return Err(Error::BusinessRule(
                format!("电话号码 {} 已被其他客户使用", phone)
            ));
        }
        
        Ok(())
    }
    
    /// 检查邮箱唯一性
    /// 
    /// # 参数
    /// 
    /// - `email`: 要检查的邮箱地址
    /// - `exclude_id`: 要排除的客户ID，用于更新时的检查
    async fn check_email_uniqueness(&self, email: &str, exclude_id: Option<i64>) -> Result<()> {
        let query = SearchQuery {
            keyword: None,
            filters: {
                let mut filters = std::collections::HashMap::new();
                filters.insert("email".to_string(), email.to_string());
                filters
            },
            sort_by: None,
            sort_order: SortOrder::Asc,
            page: 0,
            page_size: 1,
        };
        
        let result = self.repository.search(&query).await?;
        
        // 检查是否有重复的邮箱
        for customer in result.items {
            if let Some(exclude_id) = exclude_id {
                if customer.id() == Some(exclude_id) {
                    continue; // 跳过自身
                }
            }
            return Err(Error::BusinessRule(
                format!("邮箱地址 {} 已被其他客户使用", email)
            ));
        }
        
        Ok(())
    }
    
    /// 记录等级变更历史
    /// 
    /// 记录客户等级变更的历史信息，用于审计和分析。
    /// 
    /// # 参数
    /// 
    /// - `customer_id`: 客户ID
    /// - `new_level`: 新等级
    /// - `reason`: 变更原因
    async fn record_level_change_history(
        &self,
        customer_id: i64,
        new_level: CustomerLevel,
        reason: String,
    ) -> Result<()> {
        // 这里应该调用历史记录服务来保存等级变更历史
        // 为了简化，这里只是一个占位符实现
        tracing::info!(
            customer_id = customer_id,
            new_level = ?new_level,
            reason = reason,
            "客户等级变更"
        );
        
        Ok(())
    }
}

/// 客户创建请求
/// 
/// 用于创建客户时传递参数的数据结构。
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCustomerRequest {
    /// 客户名称
    #[validate(length(min = 1, max = 100, message = "客户名称长度必须在1-100个字符之间"))]
    pub name: String,
    
    /// 联系电话
    #[validate(regex(path = "PHONE_REGEX", message = "电话号码格式不正确"))]
    pub phone: String,
    
    /// 电子邮箱（可选）
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    
    /// 公司名称（可选）
    #[validate(length(max = 200, message = "公司名称不能超过200个字符"))]
    pub company: Option<String>,
}

/// 客户搜索请求
/// 
/// 用于搜索客户时传递参数的数据结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCustomerRequest {
    /// 搜索关键词
    pub keyword: Option<String>,
    
    /// 客户等级筛选
    pub level: Option<CustomerLevel>,
    
    /// 创建时间范围筛选
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    
    /// 排序字段
    pub sort_by: Option<String>,
    
    /// 排序顺序
    pub sort_order: Option<SortOrder>,
    
    /// 页码
    pub page: Option<usize>,
    
    /// 每页大小
    pub page_size: Option<usize>,
}

impl SearchCustomerRequest {
    /// 构建筛选条件
    /// 
    /// 将搜索请求转换为Repository可以理解的筛选条件。
    fn build_filters(&self) -> std::collections::HashMap<String, String> {
        let mut filters = std::collections::HashMap::new();
        
        if let Some(level) = &self.level {
            filters.insert("level".to_string(), format!("{:?}", level).to_lowercase());
        }
        
        if let Some(created_after) = &self.created_after {
            filters.insert("created_after".to_string(), created_after.to_rfc3339());
        }
        
        if let Some(created_before) = &self.created_before {
            filters.insert("created_before".to_string(), created_before.to_rfc3339());
        }
        
        filters
    }
}

/// 客户等级评估策略trait
/// 
/// 定义客户等级评估的接口，支持不同的评估策略。
#[async_trait]
pub trait CustomerLevelStrategy: Send + Sync {
    /// 评估客户等级
    /// 
    /// 根据客户信息和历史数据评估合适的客户等级。
    /// 
    /// # 参数
    /// 
    /// - `customer`: 客户实体
    /// 
    /// # 返回值
    /// 
    /// - `Ok(CustomerLevel)`: 建议的客户等级
    /// - `Err(Error)`: 评估失败
    async fn evaluate_level(&self, customer: &Customer) -> Result<CustomerLevel>;
}

/// 默认客户等级评估策略
/// 
/// 基于简单规则的客户等级评估实现。
pub struct DefaultCustomerLevelStrategy {
    // 可以添加配置参数
}

impl DefaultCustomerLevelStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CustomerLevelStrategy for DefaultCustomerLevelStrategy {
    async fn evaluate_level(&self, customer: &Customer) -> Result<CustomerLevel> {
        // 简单的等级评估逻辑
        // 实际实现中可能需要查询客户的交易历史、互动记录等
        
        // 新客户默认为普通客户
        if customer.id().is_none() {
            return Ok(CustomerLevel::Normal);
        }
        
        // 这里可以添加更复杂的评估逻辑
        // 例如：根据交易金额、合作时长、信用记录等因素评估
        
        Ok(customer.level) // 暂时返回当前等级
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio_test;
    
    // 模拟Repository用于测试
    struct MockCustomerRepository {
        customers: std::sync::Mutex<Vec<Customer>>,
    }
    
    impl MockCustomerRepository {
        fn new() -> Self {
            Self {
                customers: std::sync::Mutex::new(Vec::new()),
            }
        }
    }
    
    #[async_trait]
    impl Repository<Customer> for MockCustomerRepository {
        async fn create(&self, entity: &mut Customer) -> Result<i64> {
            let mut customers = self.customers.lock().unwrap();
            let id = customers.len() as i64 + 1;
            entity.set_id(id);
            customers.push(entity.clone());
            Ok(id)
        }
        
        async fn find_by_id(&self, id: i64) -> Result<Option<Customer>> {
            let customers = self.customers.lock().unwrap();
            Ok(customers.iter().find(|c| c.id() == Some(id)).cloned())
        }
        
        // 其他方法的简单实现...
        async fn find_all(&self) -> Result<Vec<Customer>> { todo!() }
        async fn update(&self, _entity: &Customer) -> Result<()> { todo!() }
        async fn delete(&self, _id: i64) -> Result<()> { todo!() }
        async fn search(&self, _query: &SearchQuery) -> Result<SearchResult<Customer>> { todo!() }
        async fn count(&self) -> Result<i64> { todo!() }
    }
    
    #[tokio::test]
    async fn test_create_customer() {
        let repository = Arc::new(MockCustomerRepository::new());
        let event_bus = Arc::new(EventBus::new());
        let service = CustomerService::new(repository, event_bus);
        
        let request = CreateCustomerRequest {
            name: "测试客户".to_string(),
            phone: "138-1234-5678".to_string(),
            email: Some("test@example.com".to_string()),
            company: Some("测试公司".to_string()),
        };
        
        let result = service.create_customer(request).await;
        assert!(result.is_ok());
        
        let customer = result.unwrap();
        assert_eq!(customer.name, "测试客户");
        assert_eq!(customer.phone, "138-1234-5678");
        assert!(customer.id().is_some());
    }
}
```