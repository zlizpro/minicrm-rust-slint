//! 核心实体定义模块
//!
//! 定义系统中的核心业务实体，包括客户、供应商、任务、报价等

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 客户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// 客户ID
    pub id: Uuid,
    /// 客户名称
    pub name: String,
    /// 联系人
    pub contact_person: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 客户等级
    pub level: CustomerLevel,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 客户等级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerLevel {
    /// 普通客户
    Normal,
    /// VIP客户
    Vip,
    /// 重要客户
    Important,
    /// 黑名单
    Blacklist,
}

/// 供应商实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    /// 供应商ID
    pub id: Uuid,
    /// 供应商名称
    pub name: String,
    /// 联系人
    pub contact_person: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 供应商等级
    pub level: SupplierLevel,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 供应商等级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplierLevel {
    /// 普通供应商
    Normal,
    /// 优质供应商
    Premium,
    /// 战略合作伙伴
    Strategic,
    /// 暂停合作
    Suspended,
}

/// 任务实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务ID
    pub id: Uuid,
    /// 任务标题
    pub title: String,
    /// 任务描述
    pub description: Option<String>,
    /// 任务状态
    pub status: TaskStatus,
    /// 优先级
    pub priority: TaskPriority,
    /// 关联客户ID
    pub customer_id: Option<Uuid>,
    /// 关联供应商ID
    pub supplier_id: Option<Uuid>,
    /// 截止日期
    pub due_date: Option<DateTime<Utc>>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 待处理
    Pending,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 任务优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    /// 低优先级
    Low,
    /// 中等优先级
    Medium,
    /// 高优先级
    High,
    /// 紧急
    Urgent,
}

/// 报价实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// 报价ID
    pub id: Uuid,
    /// 报价编号
    pub quote_number: String,
    /// 客户ID
    pub customer_id: Uuid,
    /// 报价状态
    pub status: QuoteStatus,
    /// 总金额
    pub total_amount: f64,
    /// 有效期
    pub valid_until: DateTime<Utc>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 报价状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuoteStatus {
    /// 草稿
    Draft,
    /// 已发送
    Sent,
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 已过期
    Expired,
}

/// 售后服务工单实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTicket {
    /// 工单ID
    pub id: Uuid,
    /// 工单编号
    pub ticket_number: String,
    /// 客户ID
    pub customer_id: Uuid,
    /// 问题分类
    pub problem_category: String,
    /// 问题描述
    pub description: String,
    /// 处理方式
    pub solution_method: Option<String>,
    /// 工单状态
    pub status: ServiceTicketStatus,
    /// 优先级
    pub priority: TaskPriority,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 售后工单状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceTicketStatus {
    /// 新建
    New,
    /// 处理中
    InProgress,
    /// 待客户确认
    PendingCustomerConfirmation,
    /// 已关闭
    Closed,
}
