//! 数据库健康检查
//!
//! 提供数据库连接状态监控和健康检查功能。

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, error, warn};

use super::connection::DatabaseConnection;
use super::pool::{DatabasePool, DatabasePoolExt, PoolStats};

/// 数据库健康检查器
#[derive(Debug)]
pub struct DatabaseHealthChecker {
    connection: DatabaseConnection,
    pool: DatabasePool,
}

/// 健康检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// 检查时间
    pub timestamp: DateTime<Utc>,
    /// 整体健康状态
    pub healthy: bool,
    /// 连接池状态
    pub pool_status: PoolHealthStatus,
    /// 数据库响应时间（毫秒）
    pub response_time_ms: u64,
    /// 检查详情
    pub checks: Vec<HealthCheck>,
    /// 错误信息（如果有）
    pub error: Option<String>,
}

/// 连接池健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolHealthStatus {
    /// 连接池统计
    pub stats: PoolStats,
    /// 连接池是否健康
    pub healthy: bool,
    /// 连接使用率
    pub utilization_percentage: f64,
}

/// 单项健康检查
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// 检查名称
    pub name: String,
    /// 检查是否通过
    pub passed: bool,
    /// 检查耗时（毫秒）
    pub duration_ms: u64,
    /// 检查结果详情
    pub details: Option<String>,
    /// 错误信息（如果检查失败）
    pub error: Option<String>,
}

impl DatabaseHealthChecker {
    /// 创建新的健康检查器
    pub fn new(connection: DatabaseConnection, pool: DatabasePool) -> Self {
        Self { connection, pool }
    }

    /// 执行完整的健康检查
    pub fn check_health(&self) -> HealthCheckResult {
        let start_time = Instant::now();
        let timestamp = Utc::now();

        debug!("开始数据库健康检查");

        let mut checks = Vec::new();
        let mut overall_healthy = true;
        let mut error_message = None;

        // 1. 连接池健康检查
        let pool_status = self.check_pool_health();
        if !pool_status.healthy {
            overall_healthy = false;
        }

        // 2. 基本连接检查
        let connection_check = self.check_basic_connection();
        if !connection_check.passed {
            overall_healthy = false;
            error_message = connection_check.error.clone();
        }
        checks.push(connection_check);

        // 3. 数据库完整性检查
        let integrity_check = self.check_database_integrity();
        if !integrity_check.passed {
            overall_healthy = false;
        }
        checks.push(integrity_check);

        // 4. 性能检查
        let performance_check = self.check_performance();
        if !performance_check.passed {
            // 性能问题不影响整体健康状态，只记录警告
            warn!("数据库性能检查未通过: {:?}", performance_check.error);
        }
        checks.push(performance_check);

        // 5. 磁盘空间检查
        let disk_check = self.check_disk_space();
        if !disk_check.passed {
            overall_healthy = false;
        }
        checks.push(disk_check);

        let response_time_ms = start_time.elapsed().as_millis() as u64;

        let result = HealthCheckResult {
            timestamp,
            healthy: overall_healthy,
            pool_status,
            response_time_ms,
            checks,
            error: error_message,
        };

        if overall_healthy {
            debug!("数据库健康检查通过，耗时: {}ms", response_time_ms);
        } else {
            error!("数据库健康检查失败，耗时: {}ms", response_time_ms);
        }

        result
    }

    /// 检查连接池健康状态
    fn check_pool_health(&self) -> PoolHealthStatus {
        let stats = self.pool.get_stats();
        let utilization = (stats.connections as f64 / stats.max_connections as f64) * 100.0;

        // 连接池使用率超过90%认为不健康
        let healthy = utilization < 90.0 && self.pool.health_check().is_ok();

        PoolHealthStatus {
            stats,
            healthy,
            utilization_percentage: utilization,
        }
    }

    /// 基本连接检查
    fn check_basic_connection(&self) -> HealthCheck {
        let start_time = Instant::now();
        let name = "基本连接检查".to_string();

        match self
            .connection
            .query_row("SELECT 1", [], |row| row.get::<_, i32>(0))
        {
            Ok(result) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                HealthCheck {
                    name,
                    passed: result == 1,
                    duration_ms,
                    details: Some(format!("查询结果: {}", result)),
                    error: None,
                }
            }
            Err(e) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                HealthCheck {
                    name,
                    passed: false,
                    duration_ms,
                    details: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// 数据库完整性检查
    fn check_database_integrity(&self) -> HealthCheck {
        let start_time = Instant::now();
        let name = "数据库完整性检查".to_string();

        match self
            .connection
            .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        {
            Ok(result) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                let passed = result == "ok";
                HealthCheck {
                    name,
                    passed,
                    duration_ms,
                    details: Some(format!("完整性检查结果: {}", result)),
                    error: if passed { None } else { Some(result) },
                }
            }
            Err(e) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                HealthCheck {
                    name,
                    passed: false,
                    duration_ms,
                    details: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// 性能检查
    fn check_performance(&self) -> HealthCheck {
        let start_time = Instant::now();
        let name = "性能检查".to_string();

        // 执行一个稍微复杂的查询来测试性能
        match self.connection.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |row| row.get::<_, i64>(0),
        ) {
            Ok(table_count) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                // 如果查询耗时超过100ms，认为性能有问题
                let passed = duration_ms < 100;
                HealthCheck {
                    name,
                    passed,
                    duration_ms,
                    details: Some(format!(
                        "表数量: {}, 查询耗时: {}ms",
                        table_count, duration_ms
                    )),
                    error: if passed {
                        None
                    } else {
                        Some(format!("查询耗时过长: {}ms", duration_ms))
                    },
                }
            }
            Err(e) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                HealthCheck {
                    name,
                    passed: false,
                    duration_ms,
                    details: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// 磁盘空间检查
    fn check_disk_space(&self) -> HealthCheck {
        let start_time = Instant::now();
        let name = "磁盘空间检查".to_string();

        match self
            .connection
            .query_row("PRAGMA page_count", [], |row| row.get::<_, i64>(0))
        {
            Ok(page_count) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;

                // 获取页面大小
                let page_size = self
                    .connection
                    .query_row("PRAGMA page_size", [], |row| row.get::<_, i64>(0))
                    .unwrap_or(4096);

                let db_size_bytes = page_count * page_size;
                let db_size_mb = db_size_bytes as f64 / (1024.0 * 1024.0);

                // 简单的磁盘空间检查：如果数据库大于1GB，给出警告
                let passed = db_size_mb < 1024.0;

                HealthCheck {
                    name,
                    passed,
                    duration_ms,
                    details: Some(format!(
                        "数据库大小: {:.2} MB ({} 页, {} 字节/页)",
                        db_size_mb, page_count, page_size
                    )),
                    error: if passed {
                        None
                    } else {
                        Some(format!("数据库文件过大: {:.2} MB", db_size_mb))
                    },
                }
            }
            Err(e) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                HealthCheck {
                    name,
                    passed: false,
                    duration_ms,
                    details: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// 快速健康检查（只检查基本连接）
    pub fn quick_health_check(&self) -> Result<bool> {
        self.connection
            .query_row("SELECT 1", [], |row| row.get::<_, i32>(0))
            .map(|result| result == 1)
            .context("快速健康检查失败")
    }

    /// 获取数据库统计信息
    pub fn get_database_stats(&self) -> Result<DatabaseStats> {
        let table_count = self.connection.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |row| row.get::<_, i64>(0),
        )?;

        let index_count = self.connection.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index'",
            [],
            |row| row.get::<_, i64>(0),
        )?;

        let page_count = self
            .connection
            .query_row("PRAGMA page_count", [], |row| row.get::<_, i64>(0))?;

        let page_size = self
            .connection
            .query_row("PRAGMA page_size", [], |row| row.get::<_, i64>(0))?;

        let db_size_bytes = page_count * page_size;

        Ok(DatabaseStats {
            table_count,
            index_count,
            page_count,
            page_size,
            database_size_bytes: db_size_bytes,
            database_size_mb: db_size_bytes as f64 / (1024.0 * 1024.0),
        })
    }
}

/// 数据库统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    /// 表数量
    pub table_count: i64,
    /// 索引数量
    pub index_count: i64,
    /// 页面数量
    pub page_count: i64,
    /// 页面大小（字节）
    pub page_size: i64,
    /// 数据库大小（字节）
    pub database_size_bytes: i64,
    /// 数据库大小（MB）
    pub database_size_mb: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::pool::{DatabasePoolBuilder, DatabasePoolConfig};
    use tempfile::tempdir;

    fn create_test_health_checker() -> DatabaseHealthChecker {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = DatabasePoolConfig {
            database_path: db_path.to_string_lossy().to_string(),
            ..Default::default()
        };

        let pool = DatabasePoolBuilder::new(config).build().unwrap();
        let connection = DatabaseConnection::new(pool.clone());

        DatabaseHealthChecker::new(connection, pool)
    }

    #[tokio::test]
    async fn test_health_check() {
        let checker = create_test_health_checker();

        let result = checker.check_health();

        // 基本健康检查应该通过
        assert!(result.healthy);
        assert!(result.response_time_ms > 0);
        assert!(!result.checks.is_empty());
    }

    #[tokio::test]
    async fn test_quick_health_check() {
        let checker = create_test_health_checker();

        let result = checker.quick_health_check().unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_database_stats() {
        let checker = create_test_health_checker();

        let stats = checker.get_database_stats().unwrap();

        // 新数据库应该有基本的统计信息
        assert!(stats.page_count > 0);
        assert!(stats.page_size > 0);
        assert!(stats.database_size_bytes > 0);
    }
}
