//! 数据库初始化和配置
//!
//! 负责应用程序启动时的数据库初始化工作。

use anyhow::Result;
use tracing::{debug, info};

use crate::config::AppConfig;
use minicrm_infrastructure::database::migrations::Migration;
use minicrm_infrastructure::{
    DatabaseConnection, DatabaseHealthChecker, DatabasePool, DatabasePoolConfig, MigrationManager,
};

/// 数据库管理器
#[derive(Debug)]
pub struct DatabaseManager {
    pool: DatabasePool,
    connection: DatabaseConnection,
    health_checker: DatabaseHealthChecker,
}

impl DatabaseManager {
    /// 初始化数据库
    ///
    /// # Arguments
    ///
    /// * `config` - 应用程序配置
    ///
    /// # Errors
    ///
    /// 如果数据库初始化失败，将返回错误。
    pub fn initialize(config: &AppConfig) -> Result<Self> {
        info!("开始初始化数据库");

        // 创建数据库连接池配置
        let pool_config = DatabasePoolConfig {
            database_path: config.database.path.to_string_lossy().to_string(),
            max_connections: config.database.max_connections,
            connection_timeout: config.database.connection_timeout,
            ..Default::default()
        };

        // 创建连接池
        let pool = minicrm_infrastructure::database::pool::DatabasePoolBuilder::new(pool_config)
            .build()?;

        // 创建数据库连接
        let connection = DatabaseConnection::new(pool.clone());

        // 创建健康检查器
        let health_checker = DatabaseHealthChecker::new(connection.clone(), pool.clone());

        // 执行数据库迁移
        Self::run_migrations(&connection)?;

        // 执行健康检查
        let health_result = health_checker.check_health();
        if !health_result.healthy {
            return Err(anyhow::anyhow!(
                "数据库健康检查失败: {:?}",
                health_result.error
            ));
        }

        info!("数据库初始化完成");

        Ok(Self {
            pool,
            connection,
            health_checker,
        })
    }

    /// 执行数据库迁移
    fn run_migrations(connection: &DatabaseConnection) -> Result<()> {
        debug!("开始执行数据库迁移");

        let migration_manager =
            MigrationManager::new((*connection).clone()).add_migrations(Self::get_migrations());

        // 执行迁移到最新版本
        migration_manager.migrate(None)?;

        debug!("数据库迁移完成");
        Ok(())
    }

    /// 获取所有数据库迁移
    fn get_migrations() -> Vec<minicrm_infrastructure::database::migrations::Migration> {
        use minicrm_infrastructure::migration;

        vec![
            // TODO: 在后续任务中添加具体的迁移
            migration!(
                1,
                "initial_schema",
                "创建初始数据库结构",
                "
                -- 创建系统配置表
                CREATE TABLE IF NOT EXISTS system_config (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL,
                    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                );

                -- 插入初始配置
                INSERT OR IGNORE INTO system_config (key, value) VALUES
                    ('schema_version', '1'),
                    ('app_version', '0.1.0'),
                    ('initialized_at', datetime('now'));
                "
            ),
        ]
    }

    /// 获取数据库连接池
    #[must_use]
    pub const fn get_pool(&self) -> &DatabasePool {
        &self.pool
    }

    /// 获取数据库连接
    #[must_use]
    pub const fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    /// 获取健康检查器
    #[must_use]
    pub const fn get_health_checker(&self) -> &DatabaseHealthChecker {
        &self.health_checker
    }

    /// 执行健康检查
    #[must_use]
    pub fn check_health(&self) -> minicrm_infrastructure::database::health::HealthCheckResult {
        self.health_checker.check_health()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_config() -> Result<AppConfig> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");

        let mut config = AppConfig::default();
        config.database.path = db_path;
        Ok(config)
    }

    #[tokio::test]
    async fn test_database_initialization() -> Result<()> {
        let config = create_test_config()?;

        let db_manager = DatabaseManager::initialize(&config)?;

        // 健康检查应该通过
        let health = db_manager.check_health();
        assert!(health.healthy);
        Ok(())
    }
}
