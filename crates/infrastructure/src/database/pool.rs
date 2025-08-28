//! 数据库连接池实现
//!
//! 使用r2d2连接池管理SQLite连接，提供高效的数据库访问。

use anyhow::{Context, Result};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tracing::{debug, info};

/// 数据库连接池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePoolConfig {
    /// 数据库文件路径
    pub database_path: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小空闲连接数
    pub min_idle: Option<u32>,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 空闲连接超时时间（秒）
    pub idle_timeout: Option<u64>,
    /// 连接最大生命周期（秒）
    pub max_lifetime: Option<u64>,
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            database_path: "data/minicrm.db".to_string(),
            max_connections: 10,
            min_idle: Some(1),
            connection_timeout: 30,
            idle_timeout: Some(600),  // 10分钟
            max_lifetime: Some(1800), // 30分钟
        }
    }
}

/// 数据库连接池类型别名
pub type DatabasePool = Pool<SqliteConnectionManager>;

/// 数据库连接类型别名
pub type DatabaseConnection = PooledConnection<SqliteConnectionManager>;

/// 数据库连接池构建器
pub struct DatabasePoolBuilder {
    config: DatabasePoolConfig,
}

impl DatabasePoolBuilder {
    /// 创建新的连接池构建器
    pub fn new(config: DatabasePoolConfig) -> Self {
        Self { config }
    }

    /// 构建数据库连接池
    ///
    /// # Errors
    ///
    /// 如果数据库文件无法访问或连接池创建失败，将返回错误。
    pub fn build(self) -> Result<DatabasePool> {
        info!("正在创建数据库连接池，路径: {}", self.config.database_path);

        // 确保数据库目录存在
        if let Some(parent) = Path::new(&self.config.database_path).parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("无法创建数据库目录: {:?}", parent))?;
        }

        // 配置SQLite连接管理器
        let manager = SqliteConnectionManager::file(&self.config.database_path).with_flags(
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI,
        );

        // 构建连接池
        let mut builder = Pool::builder()
            .max_size(self.config.max_connections)
            .connection_timeout(Duration::from_secs(self.config.connection_timeout));

        if let Some(min_idle) = self.config.min_idle {
            builder = builder.min_idle(Some(min_idle));
        }

        if let Some(idle_timeout) = self.config.idle_timeout {
            builder = builder.idle_timeout(Some(Duration::from_secs(idle_timeout)));
        }

        if let Some(max_lifetime) = self.config.max_lifetime {
            builder = builder.max_lifetime(Some(Duration::from_secs(max_lifetime)));
        }

        let pool = builder.build(manager).context("无法创建数据库连接池")?;

        // 测试连接
        debug!("测试数据库连接...");
        let conn = pool.get().context("无法获取数据库连接进行测试")?;

        // 执行简单查询验证连接
        let _: i32 = conn
            .query_row("SELECT 1", [], |row| row.get(0))
            .context("数据库连接测试失败")?;

        info!(
            "数据库连接池创建成功，最大连接数: {}",
            self.config.max_connections
        );

        Ok(pool)
    }
}

/// 数据库连接池扩展trait
pub trait DatabasePoolExt {
    /// 获取连接池统计信息
    fn get_stats(&self) -> PoolStats;

    /// 检查连接池健康状态
    fn health_check(&self) -> Result<()>;
}

/// 连接池统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    /// 当前连接数
    pub connections: u32,
    /// 空闲连接数
    pub idle_connections: u32,
    /// 最大连接数
    pub max_connections: u32,
}

impl DatabasePoolExt for DatabasePool {
    fn get_stats(&self) -> PoolStats {
        let state = self.state();
        PoolStats {
            connections: state.connections,
            idle_connections: state.idle_connections,
            max_connections: self.max_size(),
        }
    }

    fn health_check(&self) -> Result<()> {
        debug!("执行数据库连接池健康检查");

        let conn = self.get().context("无法获取数据库连接进行健康检查")?;

        // 执行简单查询
        conn.execute("SELECT 1", [])
            .context("数据库健康检查查询失败")?;

        debug!("数据库连接池健康检查通过");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_database_pool_creation() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = DatabasePoolConfig {
            database_path: db_path.to_string_lossy().to_string(),
            max_connections: 5,
            ..Default::default()
        };

        let pool = DatabasePoolBuilder::new(config).build().unwrap();

        // 测试获取连接
        let conn = pool.get().unwrap();
        let result: i32 = conn.query_row("SELECT 1", [], |row| row.get(0)).unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_pool_stats() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = DatabasePoolConfig {
            database_path: db_path.to_string_lossy().to_string(),
            max_connections: 3,
            ..Default::default()
        };

        let pool = DatabasePoolBuilder::new(config).build().unwrap();
        let stats = pool.get_stats();

        assert_eq!(stats.max_connections, 3);
        assert!(stats.connections <= stats.max_connections);
    }

    #[tokio::test]
    async fn test_health_check() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = DatabasePoolConfig {
            database_path: db_path.to_string_lossy().to_string(),
            ..Default::default()
        };

        let pool = DatabasePoolBuilder::new(config).build().unwrap();

        // 健康检查应该成功
        pool.health_check().unwrap();
    }
}
