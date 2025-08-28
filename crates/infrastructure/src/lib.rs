//! MiniCRM 基础设施模块
//!
//! 提供数据库访问、外部服务集成等基础设施功能。

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod database;
pub mod repository;

// 重新导出主要类型
pub use database::{
    DatabaseConnection, DatabaseHealthChecker, DatabasePool, DatabasePoolConfig, MigrationManager,
};
