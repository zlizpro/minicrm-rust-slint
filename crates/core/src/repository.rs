//! 仓储接口定义模块
//!
//! 定义数据访问层的抽象接口

use crate::{
    entity::*,
    error::CoreResult,
    types::{PagedResult, QueryFilter},
};
use async_trait::async_trait;
use uuid::Uuid;

/// 通用仓储接口
#[async_trait]
pub trait Repository<T, ID> {
    /// 根据ID查找实体
    async fn find_by_id(&self, id: ID) -> CoreResult<Option<T>>;

    /// 保存实体
    async fn save(&self, entity: &T) -> CoreResult<T>;

    /// 更新实体
    async fn update(&self, entity: &T) -> CoreResult<T>;

    /// 根据ID删除实体
    async fn delete_by_id(&self, id: ID) -> CoreResult<bool>;

    /// 查询所有实体
    async fn find_all(&self) -> CoreResult<Vec<T>>;

    /// 分页查询实体
    async fn find_with_filter(&self, filter: &QueryFilter) -> CoreResult<PagedResult<T>>;
}

/// 客户仓储接口
#[async_trait]
pub trait CustomerRepository: Repository<Customer, Uuid> {
    /// 根据名称查找客户
    async fn find_by_name(&self, name: &str) -> CoreResult<Vec<Customer>>;

    /// 根据电话查找客户
    async fn find_by_phone(&self, phone: &str) -> CoreResult<Option<Customer>>;

    /// 根据邮箱查找客户
    async fn find_by_email(&self, email: &str) -> CoreResult<Option<Customer>>;

    /// 根据等级查找客户
    async fn find_by_level(&self, level: &CustomerLevel) -> CoreResult<Vec<Customer>>;

    /// 搜索客户
    async fn search(&self, keyword: &str) -> CoreResult<Vec<Customer>>;
}

/// 供应商仓储接口
#[async_trait]
pub trait SupplierRepository: Repository<Supplier, Uuid> {
    /// 根据名称查找供应商
    async fn find_by_name(&self, name: &str) -> CoreResult<Vec<Supplier>>;

    /// 根据电话查找供应商
    async fn find_by_phone(&self, phone: &str) -> CoreResult<Option<Supplier>>;

    /// 根据邮箱查找供应商
    async fn find_by_email(&self, email: &str) -> CoreResult<Option<Supplier>>;

    /// 根据等级查找供应商
    async fn find_by_level(&self, level: &SupplierLevel) -> CoreResult<Vec<Supplier>>;

    /// 搜索供应商
    async fn search(&self, keyword: &str) -> CoreResult<Vec<Supplier>>;
}

/// 任务仓储接口
#[async_trait]
pub trait TaskRepository: Repository<Task, Uuid> {
    /// 根据客户ID查找任务
    async fn find_by_customer_id(&self, customer_id: Uuid) -> CoreResult<Vec<Task>>;

    /// 根据供应商ID查找任务
    async fn find_by_supplier_id(&self, supplier_id: Uuid) -> CoreResult<Vec<Task>>;

    /// 根据状态查找任务
    async fn find_by_status(&self, status: &TaskStatus) -> CoreResult<Vec<Task>>;

    /// 根据优先级查找任务
    async fn find_by_priority(&self, priority: &TaskPriority) -> CoreResult<Vec<Task>>;

    /// 查找即将到期的任务
    async fn find_due_soon(&self, days: u32) -> CoreResult<Vec<Task>>;

    /// 查找逾期任务
    async fn find_overdue(&self) -> CoreResult<Vec<Task>>;
}

/// 报价仓储接口
#[async_trait]
pub trait QuoteRepository: Repository<Quote, Uuid> {
    /// 根据客户ID查找报价
    async fn find_by_customer_id(&self, customer_id: Uuid) -> CoreResult<Vec<Quote>>;

    /// 根据状态查找报价
    async fn find_by_status(&self, status: &QuoteStatus) -> CoreResult<Vec<Quote>>;

    /// 根据报价编号查找报价
    async fn find_by_quote_number(&self, quote_number: &str) -> CoreResult<Option<Quote>>;

    /// 查找即将过期的报价
    async fn find_expiring_soon(&self, days: u32) -> CoreResult<Vec<Quote>>;

    /// 查找已过期的报价
    async fn find_expired(&self) -> CoreResult<Vec<Quote>>;
}

/// 售后服务工单仓储接口
#[async_trait]
pub trait ServiceTicketRepository: Repository<ServiceTicket, Uuid> {
    /// 根据客户ID查找工单
    async fn find_by_customer_id(&self, customer_id: Uuid) -> CoreResult<Vec<ServiceTicket>>;

    /// 根据状态查找工单
    async fn find_by_status(&self, status: &ServiceTicketStatus) -> CoreResult<Vec<ServiceTicket>>;

    /// 根据问题分类查找工单
    async fn find_by_problem_category(&self, category: &str) -> CoreResult<Vec<ServiceTicket>>;

    /// 根据工单编号查找工单
    async fn find_by_ticket_number(&self, ticket_number: &str)
    -> CoreResult<Option<ServiceTicket>>;

    /// 根据优先级查找工单
    async fn find_by_priority(&self, priority: &TaskPriority) -> CoreResult<Vec<ServiceTicket>>;
}
