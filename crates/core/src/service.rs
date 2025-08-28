//! 服务接口定义模块
//!
//! 定义业务逻辑层的抽象接口

use crate::{
    entity::*,
    error::CoreResult,
    types::{PagedResult, QueryFilter},
};
use async_trait::async_trait;
use uuid::Uuid;

/// 客户服务接口
#[async_trait]
pub trait CustomerService {
    /// 创建客户
    async fn create_customer(&self, customer: Customer) -> CoreResult<Customer>;

    /// 更新客户信息
    async fn update_customer(&self, customer: Customer) -> CoreResult<Customer>;

    /// 根据ID获取客户
    async fn get_customer_by_id(&self, id: Uuid) -> CoreResult<Option<Customer>>;

    /// 删除客户
    async fn delete_customer(&self, id: Uuid) -> CoreResult<bool>;

    /// 搜索客户
    async fn search_customers(&self, filter: &QueryFilter) -> CoreResult<PagedResult<Customer>>;

    /// 更新客户等级
    async fn update_customer_level(&self, id: Uuid, level: CustomerLevel) -> CoreResult<Customer>;

    /// 获取客户统计信息
    async fn get_customer_statistics(&self) -> CoreResult<CustomerStatistics>;
}

/// 供应商服务接口
#[async_trait]
pub trait SupplierService {
    /// 创建供应商
    async fn create_supplier(&self, supplier: Supplier) -> CoreResult<Supplier>;

    /// 更新供应商信息
    async fn update_supplier(&self, supplier: Supplier) -> CoreResult<Supplier>;

    /// 根据ID获取供应商
    async fn get_supplier_by_id(&self, id: Uuid) -> CoreResult<Option<Supplier>>;

    /// 删除供应商
    async fn delete_supplier(&self, id: Uuid) -> CoreResult<bool>;

    /// 搜索供应商
    async fn search_suppliers(&self, filter: &QueryFilter) -> CoreResult<PagedResult<Supplier>>;

    /// 更新供应商等级
    async fn update_supplier_level(&self, id: Uuid, level: SupplierLevel) -> CoreResult<Supplier>;

    /// 获取供应商统计信息
    async fn get_supplier_statistics(&self) -> CoreResult<SupplierStatistics>;
}

/// 任务服务接口
#[async_trait]
pub trait TaskService {
    /// 创建任务
    async fn create_task(&self, task: Task) -> CoreResult<Task>;

    /// 更新任务
    async fn update_task(&self, task: Task) -> CoreResult<Task>;

    /// 根据ID获取任务
    async fn get_task_by_id(&self, id: Uuid) -> CoreResult<Option<Task>>;

    /// 删除任务
    async fn delete_task(&self, id: Uuid) -> CoreResult<bool>;

    /// 搜索任务
    async fn search_tasks(&self, filter: &QueryFilter) -> CoreResult<PagedResult<Task>>;

    /// 更新任务状态
    async fn update_task_status(&self, id: Uuid, status: TaskStatus) -> CoreResult<Task>;

    /// 获取即将到期的任务
    async fn get_due_tasks(&self, days: u32) -> CoreResult<Vec<Task>>;

    /// 获取任务统计信息
    async fn get_task_statistics(&self) -> CoreResult<TaskStatistics>;
}

/// 报价服务接口
#[async_trait]
pub trait QuoteService {
    /// 创建报价
    async fn create_quote(&self, quote: Quote) -> CoreResult<Quote>;

    /// 更新报价
    async fn update_quote(&self, quote: Quote) -> CoreResult<Quote>;

    /// 根据ID获取报价
    async fn get_quote_by_id(&self, id: Uuid) -> CoreResult<Option<Quote>>;

    /// 删除报价
    async fn delete_quote(&self, id: Uuid) -> CoreResult<bool>;

    /// 搜索报价
    async fn search_quotes(&self, filter: &QueryFilter) -> CoreResult<PagedResult<Quote>>;

    /// 更新报价状态
    async fn update_quote_status(&self, id: Uuid, status: QuoteStatus) -> CoreResult<Quote>;

    /// 获取即将过期的报价
    async fn get_expiring_quotes(&self, days: u32) -> CoreResult<Vec<Quote>>;

    /// 获取报价统计信息
    async fn get_quote_statistics(&self) -> CoreResult<QuoteStatistics>;
}

/// 售后服务接口
#[async_trait]
pub trait ServiceTicketService {
    /// 创建工单
    async fn create_ticket(&self, ticket: ServiceTicket) -> CoreResult<ServiceTicket>;

    /// 更新工单
    async fn update_ticket(&self, ticket: ServiceTicket) -> CoreResult<ServiceTicket>;

    /// 根据ID获取工单
    async fn get_ticket_by_id(&self, id: Uuid) -> CoreResult<Option<ServiceTicket>>;

    /// 删除工单
    async fn delete_ticket(&self, id: Uuid) -> CoreResult<bool>;

    /// 搜索工单
    async fn search_tickets(&self, filter: &QueryFilter) -> CoreResult<PagedResult<ServiceTicket>>;

    /// 更新工单状态
    async fn update_ticket_status(
        &self,
        id: Uuid,
        status: ServiceTicketStatus,
    ) -> CoreResult<ServiceTicket>;

    /// 获取工单统计信息
    async fn get_ticket_statistics(&self) -> CoreResult<ServiceTicketStatistics>;
}

/// 客户统计信息
#[derive(Debug, Clone)]
pub struct CustomerStatistics {
    /// 总客户数
    pub total_customers: u64,
    /// 各等级客户数量
    pub customers_by_level: std::collections::HashMap<String, u64>,
    /// 本月新增客户数
    pub new_customers_this_month: u64,
}

/// 供应商统计信息
#[derive(Debug, Clone)]
pub struct SupplierStatistics {
    /// 总供应商数
    pub total_suppliers: u64,
    /// 各等级供应商数量
    pub suppliers_by_level: std::collections::HashMap<String, u64>,
    /// 本月新增供应商数
    pub new_suppliers_this_month: u64,
}

/// 任务统计信息
#[derive(Debug, Clone)]
pub struct TaskStatistics {
    /// 总任务数
    pub total_tasks: u64,
    /// 各状态任务数量
    pub tasks_by_status: std::collections::HashMap<String, u64>,
    /// 各优先级任务数量
    pub tasks_by_priority: std::collections::HashMap<String, u64>,
    /// 即将到期任务数
    pub due_soon_tasks: u64,
    /// 逾期任务数
    pub overdue_tasks: u64,
}

/// 报价统计信息
#[derive(Debug, Clone)]
pub struct QuoteStatistics {
    /// 总报价数
    pub total_quotes: u64,
    /// 各状态报价数量
    pub quotes_by_status: std::collections::HashMap<String, u64>,
    /// 本月报价总金额
    pub total_amount_this_month: f64,
    /// 报价成功率
    pub success_rate: f64,
}

/// 售后工单统计信息
#[derive(Debug, Clone)]
pub struct ServiceTicketStatistics {
    /// 总工单数
    pub total_tickets: u64,
    /// 各状态工单数量
    pub tickets_by_status: std::collections::HashMap<String, u64>,
    /// 各问题分类工单数量
    pub tickets_by_category: std::collections::HashMap<String, u64>,
    /// 平均处理时间（小时）
    pub average_resolution_time: f64,
}
