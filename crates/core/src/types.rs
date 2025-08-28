//! 通用类型定义模块
//!
//! 定义系统中使用的通用类型和常量

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// 页码（从1开始）
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl Pagination {
    /// 创建新的分页参数
    pub fn new(page: u32, page_size: u32) -> Self {
        Self { page, page_size }
    }

    /// 计算偏移量
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }

    /// 获取限制数量
    pub fn limit(&self) -> u32 {
        self.page_size
    }
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总记录数
    pub total: u64,
    /// 当前页码
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
    /// 总页数
    pub total_pages: u32,
}

impl<T> PagedResult<T> {
    /// 创建新的分页结果
    pub fn new(items: Vec<T>, total: u64, pagination: &Pagination) -> Self {
        let total_pages = ((total as f64) / (pagination.page_size as f64)).ceil() as u32;

        Self {
            items,
            total,
            page: pagination.page,
            page_size: pagination.page_size,
            total_pages,
        }
    }

    /// 是否有下一页
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// 是否有上一页
    pub fn has_previous(&self) -> bool {
        self.page > 1
    }
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SortDirection {
    /// 升序
    #[default]
    Asc,
    /// 降序
    Desc,
}

/// 排序参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortBy {
    /// 排序字段
    pub field: String,
    /// 排序方向
    pub direction: SortDirection,
}

impl SortBy {
    /// 创建新的排序参数
    pub fn new<S: Into<String>>(field: S, direction: SortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }

    /// 创建升序排序
    pub fn asc<S: Into<String>>(field: S) -> Self {
        Self::new(field, SortDirection::Asc)
    }

    /// 创建降序排序
    pub fn desc<S: Into<String>>(field: S) -> Self {
        Self::new(field, SortDirection::Desc)
    }
}

/// 查询过滤器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryFilter {
    /// 过滤条件
    pub filters: HashMap<String, FilterValue>,
    /// 搜索关键词
    pub search: Option<String>,
    /// 排序参数
    pub sort_by: Option<SortBy>,
    /// 分页参数
    pub pagination: Pagination,
}

/// 过滤器值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValue {
    /// 字符串值
    String(String),
    /// 整数值
    Integer(i64),
    /// 浮点数值
    Float(f64),
    /// 布尔值
    Boolean(bool),
    /// 字符串列表
    StringList(Vec<String>),
    /// 整数列表
    IntegerList(Vec<i64>),
    /// 日期范围
    DateRange {
        /// 开始日期
        start: Option<chrono::DateTime<chrono::Utc>>,
        /// 结束日期
        end: Option<chrono::DateTime<chrono::Utc>>,
    },
}

impl QueryFilter {
    /// 创建新的查询过滤器
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加字符串过滤器
    pub fn with_string_filter<K: Into<String>, V: Into<String>>(
        mut self,
        key: K,
        value: V,
    ) -> Self {
        self.filters
            .insert(key.into(), FilterValue::String(value.into()));
        self
    }

    /// 添加整数过滤器
    pub fn with_integer_filter<K: Into<String>>(mut self, key: K, value: i64) -> Self {
        self.filters.insert(key.into(), FilterValue::Integer(value));
        self
    }

    /// 添加搜索关键词
    pub fn with_search<S: Into<String>>(mut self, search: S) -> Self {
        self.search = Some(search.into());
        self
    }

    /// 添加排序参数
    pub fn with_sort(mut self, sort_by: SortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    /// 设置分页参数
    pub fn with_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = pagination;
        self
    }
}

/// 系统配置常量
pub mod constants {
    /// 默认页面大小
    pub const DEFAULT_PAGE_SIZE: u32 = 20;

    /// 最大页面大小
    pub const MAX_PAGE_SIZE: u32 = 100;

    /// 默认数据库连接池大小
    pub const DEFAULT_DB_POOL_SIZE: u32 = 10;

    /// 默认查询超时时间（秒）
    pub const DEFAULT_QUERY_TIMEOUT: u64 = 30;
}
