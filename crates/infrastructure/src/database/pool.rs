//! 数据库连接池管理
//!
//! 提供数据库连接池的创建、配置和管理功能。
//! 使用 r2d2 连接池来管理 SQLite 连接。

use std::time::Duration;

use anyhow::{Context, Result};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use tracing::{debug, info, warn};

use crate::database::health::{DatabaseHealth, PoolStatus};

/// SQLite 数据库连接池类型别名
pub type DatabasePool = Pool<SqliteConnectionManager>;

/// 数据库连接类型别名
pub type DatabaseConnection = PooledConnection<SqliteConnectionManager>;

/// 数据库连接池配置
#[derive(Debug, Clone)]
pub struct PoolConfig {
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

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_idle: Some(1),
            connection_timeout: 30,
            idle_timeout: Some(600), // 10 分钟
            max_lifetime: Some(1800), // 30 分钟
        }
    }
}

/// 数据库连接池构建器
pub struct DatabasePoolBuilder {
    database_path: String,
    config: PoolConfig,
}

impl DatabasePoolBuilder {
    /// 创建新的连接池构建器
    pub fn new<P: AsRef<str>>(database_path: P) -> Self {
        Self {
            database_path: database_path.as_ref().to_string(),
            config: PoolConfig::default(),
        }
    }

    /// 设置连接池配置
    pub fn with_config(mut self, config: PoolConfig) -> Self {
        self.config = config;
        self
    }

    /// 设置最大连接数
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.config.max_connections = max_connections;
        self
    }

    /// 设置最小空闲连接数
    pub fn min_idle(mut self, min_idle: u32) -> Self {
        self.config.min_idle = Some(min_idle);
        self
    }

    /// 设置连接超时时间
    pub fn connection_timeout(mut self, timeout_secs: u64) -> Self {
        self.config.connection_timeout = timeout_secs;
        self
    }

    /// 构建连接池
    pub fn build(self) -> Result<DatabasePool> {
        info!(
            "正在创建数据库连接池: path={}, max_connections={}",
            self.database_path, self.config.max_connections
        );

        // 创建连接管理器
        let manager = SqliteConnectionManager::file(&self.database_path)
            .with_init(|conn| {
                // 配置 SQLite 连接
                conn.execute_batch(
                    "
                    PRAGMA foreign_keys = ON;
                    PRAGMA journal_mode = WAL;
                    PRAGMA synchronous = NORMAL;
                    PRAGMA cache_size = -64000;
                    PRAGMA temp_store = MEMORY;
                    PRAGMA mmap_size = 268435456;
                    ",
                )?;
                Ok(())
            });

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

        let pool = builder
            .build(manager)
            .context("无法创建数据库连接池")?;

        // 测试连接
        let conn = pool.get().context("无法获取数据库连接进行测试")?;
        conn.execute("SELECT 1", [])
            .context("数据库连接测试失败")?;
        drop(conn);

        info!(
            "数据库连接池创建成功: 最大连接数={}, 当前连接数={}",
            self.config.max_connections,
            pool.state().connections
        );

        Ok(pool)
    }
}

/// 数据库连接池扩展 trait
///
/// 为连接池提供额外的管理功能
pub trait DatabasePoolExt {
    /// 获取连接池健康状态
    fn get_pool_status(&self) -> PoolStatus;

    /// 执行健康检查
    fn health_check(&self) -> Result<()>;

    /// 获取详细的健康信息
    fn get_health(&self) -> DatabaseHealth;
}

impl DatabasePoolExt for DatabasePool {
    fn get_pool_status(&self) -> PoolStatus {
        let state = self.state();
        let utilization = if self.max_size() > 0 {
            (state.connections as f64 / self.max_size() as f64) * 100.0
        } else {
            0.0
        };

        PoolStatus {
            healthy: state.connections > 0,
            total_connections: self.max_size(),
            active_connections: state.connections,
            idle_connections: state.idle_connections,
            utilization_percentage: utilization,
        }
    }

    fn health_check(&self) -> Result<()> {
        debug!("开始数据库连接池健康检查");

        let conn = self.get().context("无法获取数据库连接进行健康检查")?;

        // 执行简单查询验证连接
        let result: i32 = conn
            .query_row("SELECT 1", [], |row| row.get(0))
            .context("数据库健康检查查询失败")?;

        if result != 1 {
            return Err(anyhow::anyhow!(
                "数据库健康检查返回了意外的结果: {}",
                result
            ));
        }

        debug!("数据库连接池健康检查通过");
        Ok(())
    }

    fn get_health(&self) -> DatabaseHealth {
        let pool_status = self.get_pool_status();
        let mut checks = Vec::new();

        // 连接池基本检查
        let pool_check = match self.health_check() {
            Ok(()) => ("connection_pool".to_string(), true, None),
            Err(e) => (
                "connection_pool".to_string(),
                false,
                Some(e.to_string()),
            ),
        };
        checks.push(pool_check);

        // 连接数检查
        let connections_ok = pool_status.active_connections > 0;
        let connection_check = (
            "active_connections".to_string(),
            connections_ok,
            if connections_ok {
                None
            } else {
                Some("没有活跃的数据库连接".to_string())
            },
        );
        checks.push(connection_check);

        // 利用率检查
        let utilization_ok = pool_status.utilization_percentage < 90.0;
        let utilization_check = (
            "pool_utilization".to_string(),
            utilization_ok,
            if utilization_ok {
                None
            } else {
                Some(format!(
                    "连接池利用率过高: {:.1}%",
                    pool_status.utilization_percentage
                ))
            },
        );
        checks.push(utilization_check);

        let healthy = checks.iter().all(|(_, status, _)| *status);

        DatabaseHealth {
            healthy,
            pool_status,
            checks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_idle, Some(1));
        assert_eq!(config.connection_timeout, 30);
    }

    #[test]
    fn test_database_pool_builder() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path().to_str().unwrap();

        let pool = DatabasePoolBuilder::new(db_path)
            .max_connections(5)
            .min_idle(1)
            .connection_timeout(10)
            .build()?;

        assert_eq!(pool.max_size(), 5);

        // 测试连接
        let conn = pool.get()?;
        let result: i32 = conn.query_row("SELECT 1", [], |row| row.get(0))?;
        assert_eq!(result, 1);

        Ok(())
    }

    #[test]
    fn test_pool_health_check() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path().to_str().unwrap();

        let pool = DatabasePoolBuilder::new(db_path).build()?;

        // 健康检查应该通过
        pool.health_check()?;

        // 获取健康状态
        let health = pool.get_health();
        assert!(health.healthy);
        assert!(!health.checks.is_empty());

        Ok(())
    }

    #[test]
    fn test_pool_status() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path().to_str().unwrap();

        let pool = DatabasePoolBuilder::new(db_path)
            .max_connections(5)
            .build()?;

        let status = pool.get_pool_status();
        assert_eq!(status.total_connections, 5);
        assert!(status.healthy);
        assert!(status.utilization_percentage >= 0.0);

        Ok(())
    }

    #[test]
    fn test_pool_with_config() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path().to_str().unwrap();

        let config = PoolConfig {
            max_connections: 3,
            min_idle: Some(1),
            connection_timeout: 15,
            idle_timeout: Some(300),
            max_lifetime: Some(900),
        };

        let pool = DatabasePoolBuilder::new(db_path)
            .with_config(config)
            .build()?;

        assert_eq!(pool.max_size(), 3);

        // 验证连接工作正常
        let conn = pool.get()?;
        conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER)", [])?;

        Ok(())
    }
}