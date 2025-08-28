//! 数据库基础设施模块
//!
//! 提供SQLite数据库连接、连接池管理和基础数据库操作。

pub mod connection;
pub mod health;
pub mod migrations;
pub mod pool;

// 重新导出主要类型
pub use connection::DatabaseConnection;
pub use health::DatabaseHealthChecker;
pub use migrations::MigrationManager;
pub use pool::{DatabasePool, DatabasePoolConfig};
