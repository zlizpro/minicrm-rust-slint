//! 错误处理模块
//!
//! 定义了应用程序中使用的各种错误类型和统一的错误处理机制。

use thiserror::Error;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// 数据库相关错误
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),

    /// 配置相关错误
    #[error("配置错误: {0}")]
    Config(String),

    /// 用户界面相关错误
    #[error("界面错误: {0}")]
    Ui(String),

    /// 验证错误
    #[error("验证错误: {0}")]
    Validation(String),

    /// IO错误
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    /// 序列化/反序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    /// 通用错误
    #[error("应用程序错误: {0}")]
    Generic(String),
}

/// 应用程序Result类型别名
pub type Result<T> = std::result::Result<T, Error>;
