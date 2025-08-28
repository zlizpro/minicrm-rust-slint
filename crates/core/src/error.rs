//! 错误处理模块
//!
//! 定义系统中的错误类型和错误处理机制

use thiserror::Error;

/// 核心错误类型
#[derive(Error, Debug)]
pub enum CoreError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(#[from] DatabaseError),

    /// 验证错误
    #[error("验证错误: {0}")]
    Validation(String),

    /// 业务逻辑错误
    #[error("业务逻辑错误: {0}")]
    Business(String),

    /// 资源未找到错误
    #[error("资源未找到: {0}")]
    NotFound(String),

    /// 权限错误
    #[error("权限不足: {0}")]
    Permission(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    Configuration(String),

    /// 外部服务错误
    #[error("外部服务错误: {0}")]
    ExternalService(String),

    /// 序列化/反序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    /// 其他错误
    #[error("未知错误: {0}")]
    Other(String),
}

/// 数据库错误类型
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// 连接错误
    #[error("数据库连接错误: {0}")]
    Connection(String),

    /// 查询错误
    #[error("数据库查询错误: {0}")]
    Query(String),

    /// 事务错误
    #[error("数据库事务错误: {0}")]
    Transaction(String),

    /// 约束违反错误
    #[error("数据库约束违反: {0}")]
    Constraint(String),

    /// 迁移错误
    #[error("数据库迁移错误: {0}")]
    Migration(String),
}

/// 核心结果类型
pub type CoreResult<T> = Result<T, CoreError>;

/// 数据库结果类型
pub type DatabaseResult<T> = Result<T, DatabaseError>;

impl From<anyhow::Error> for CoreError {
    fn from(err: anyhow::Error) -> Self {
        CoreError::Other(err.to_string())
    }
}

impl CoreError {
    /// 创建验证错误
    pub fn validation<S: Into<String>>(message: S) -> Self {
        CoreError::Validation(message.into())
    }

    /// 创建业务逻辑错误
    pub fn business<S: Into<String>>(message: S) -> Self {
        CoreError::Business(message.into())
    }

    /// 创建资源未找到错误
    pub fn not_found<S: Into<String>>(resource: S) -> Self {
        CoreError::NotFound(resource.into())
    }

    /// 创建权限错误
    pub fn permission<S: Into<String>>(message: S) -> Self {
        CoreError::Permission(message.into())
    }

    /// 创建配置错误
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        CoreError::Configuration(message.into())
    }
}

impl DatabaseError {
    /// 创建连接错误
    pub fn connection<S: Into<String>>(message: S) -> Self {
        DatabaseError::Connection(message.into())
    }

    /// 创建查询错误
    pub fn query<S: Into<String>>(message: S) -> Self {
        DatabaseError::Query(message.into())
    }

    /// 创建事务错误
    pub fn transaction<S: Into<String>>(message: S) -> Self {
        DatabaseError::Transaction(message.into())
    }

    /// 创建约束违反错误
    pub fn constraint<S: Into<String>>(message: S) -> Self {
        DatabaseError::Constraint(message.into())
    }
}
