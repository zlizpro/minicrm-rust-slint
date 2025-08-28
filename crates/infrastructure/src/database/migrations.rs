//! 数据库迁移管理
//!
//! 提供数据库schema版本管理和自动迁移功能。

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::connection::DatabaseConnection;

/// 数据库迁移管理器
pub struct MigrationManager {
    connection: DatabaseConnection,
    migrations: Vec<Migration>,
}

/// 数据库迁移定义
#[derive(Debug, Clone)]
pub struct Migration {
    /// 迁移版本号
    pub version: u32,
    /// 迁移名称
    pub name: String,
    /// 向上迁移SQL
    pub up_sql: String,
    /// 向下迁移SQL（可选）
    pub down_sql: Option<String>,
    /// 迁移描述
    pub description: String,
}

/// 迁移记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// 版本号
    pub version: u32,
    /// 迁移名称
    pub name: String,
    /// 执行时间
    pub applied_at: DateTime<Utc>,
    /// 执行耗时（毫秒）
    pub execution_time_ms: u64,
}

impl MigrationManager {
    /// 创建新的迁移管理器
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {
            connection,
            migrations: Vec::new(),
        }
    }

    /// 添加迁移
    pub fn add_migration(mut self, migration: Migration) -> Self {
        self.migrations.push(migration);
        // 按版本号排序
        self.migrations.sort_by_key(|m| m.version);
        self
    }

    /// 批量添加迁移
    pub fn add_migrations(mut self, migrations: Vec<Migration>) -> Self {
        self.migrations.extend(migrations);
        // 按版本号排序
        self.migrations.sort_by_key(|m| m.version);
        self
    }

    /// 初始化迁移系统
    ///
    /// 创建迁移记录表
    pub fn initialize(&self) -> Result<()> {
        info!("初始化数据库迁移系统");

        let sql = r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL,
                execution_time_ms INTEGER NOT NULL
            )
        "#;

        self.connection.execute(sql, [])?;

        debug!("迁移系统初始化完成");
        Ok(())
    }

    /// 获取当前数据库版本
    pub fn get_current_version(&self) -> Result<u32> {
        match self
            .connection
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| {
                row.get::<_, Option<u32>>(0)
            }) {
            Ok(Some(version)) => Ok(version),
            Ok(None) => Ok(0), // 没有迁移记录，版本为0
            Err(_e) => {
                // 如果表不存在，返回版本0
                if _e.to_string().contains("no such table") {
                    Ok(0)
                } else {
                    Err(_e)
                }
            }
        }
    }

    /// 获取已应用的迁移记录
    pub fn get_applied_migrations(&self) -> Result<Vec<MigrationRecord>> {
        self.connection.query_map(
            "SELECT version, name, applied_at, execution_time_ms FROM schema_migrations ORDER BY version",
            [],
            |row| {
                Ok(MigrationRecord {
                    version: row.get("version")?,
                    name: row.get("name")?,
                    applied_at: DateTime::parse_from_rfc3339(&row.get::<_, String>("applied_at")?)
                        .map_err(|_e| rusqlite::Error::InvalidColumnType(
                            0, "applied_at".to_string(), rusqlite::types::Type::Text
                        ))?
                        .with_timezone(&Utc),
                    execution_time_ms: row.get("execution_time_ms")?,
                })
            },
        )
    }

    /// 执行迁移到指定版本
    ///
    /// # Arguments
    ///
    /// * `target_version` - 目标版本，None表示迁移到最新版本
    pub fn migrate(&self, target_version: Option<u32>) -> Result<()> {
        self.initialize()?;

        let current_version = self.get_current_version()?;
        let target = target_version
            .unwrap_or_else(|| self.migrations.iter().map(|m| m.version).max().unwrap_or(0));

        info!(
            "开始数据库迁移：从版本 {} 到版本 {}",
            current_version, target
        );

        if current_version == target {
            info!("数据库已是最新版本 {}", target);
            return Ok(());
        }

        if current_version < target {
            self.migrate_up(current_version, target)?;
        } else {
            self.migrate_down(current_version, target)?;
        }

        info!("数据库迁移完成，当前版本: {}", target);
        Ok(())
    }

    /// 向上迁移
    fn migrate_up(&self, from_version: u32, to_version: u32) -> Result<()> {
        let migrations_to_apply: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version > from_version && m.version <= to_version)
            .collect();

        if migrations_to_apply.is_empty() {
            warn!("没有找到版本 {} 到 {} 之间的迁移", from_version, to_version);
            return Ok(());
        }

        for migration in migrations_to_apply {
            self.apply_migration(migration)?;
        }

        Ok(())
    }

    /// 向下迁移
    fn migrate_down(&self, from_version: u32, to_version: u32) -> Result<()> {
        let migrations_to_revert: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version > to_version && m.version <= from_version)
            .rev() // 反向迁移
            .collect();

        if migrations_to_revert.is_empty() {
            warn!(
                "没有找到版本 {} 到 {} 之间的回滚迁移",
                from_version, to_version
            );
            return Ok(());
        }

        for migration in migrations_to_revert {
            self.revert_migration(migration)?;
        }

        Ok(())
    }

    /// 应用单个迁移
    fn apply_migration(&self, migration: &Migration) -> Result<()> {
        info!("应用迁移 v{}: {}", migration.version, migration.name);

        let start_time = std::time::Instant::now();

        self.connection.with_transaction(|tx| {
            // 执行迁移SQL
            tx.execute(&migration.up_sql, [])?;

            // 记录迁移
            let execution_time = start_time.elapsed().as_millis() as u64;
            tx.execute(
                "INSERT INTO schema_migrations (version, name, applied_at, execution_time_ms) VALUES (?1, ?2, ?3, ?4)",
                [
                    &migration.version.to_string(),
                    &migration.name,
                    &Utc::now().to_rfc3339(),
                    &execution_time.to_string(),
                ],
            )?;

            Ok(())
        })?;

        info!(
            "迁移 v{} 应用成功，耗时: {}ms",
            migration.version,
            start_time.elapsed().as_millis()
        );
        Ok(())
    }

    /// 回滚单个迁移
    fn revert_migration(&self, migration: &Migration) -> Result<()> {
        info!("回滚迁移 v{}: {}", migration.version, migration.name);

        let down_sql = migration
            .down_sql
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("迁移 v{} 没有提供回滚SQL", migration.version))?;

        let start_time = std::time::Instant::now();

        self.connection.with_transaction(|tx| {
            // 执行回滚SQL
            tx.execute(down_sql, [])?;

            // 删除迁移记录
            tx.execute(
                "DELETE FROM schema_migrations WHERE version = ?1",
                [migration.version],
            )?;

            Ok(())
        })?;

        info!(
            "迁移 v{} 回滚成功，耗时: {}ms",
            migration.version,
            start_time.elapsed().as_millis()
        );
        Ok(())
    }

    /// 获取迁移状态
    pub fn get_migration_status(&self) -> Result<MigrationStatus> {
        let current_version = self.get_current_version()?;
        let applied_migrations = self.get_applied_migrations()?;
        let available_migrations = self.migrations.len();
        let latest_version = self.migrations.iter().map(|m| m.version).max().unwrap_or(0);

        let pending_migrations: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version > current_version)
            .map(|m| PendingMigration {
                version: m.version,
                name: m.name.clone(),
                description: m.description.clone(),
            })
            .collect();

        Ok(MigrationStatus {
            current_version,
            latest_version,
            applied_migrations,
            pending_migrations,
            available_migrations,
            is_up_to_date: current_version == latest_version,
        })
    }
}

/// 迁移状态
#[derive(Debug, Serialize)]
pub struct MigrationStatus {
    /// 当前版本
    pub current_version: u32,
    /// 最新版本
    pub latest_version: u32,
    /// 已应用的迁移
    pub applied_migrations: Vec<MigrationRecord>,
    /// 待应用的迁移
    pub pending_migrations: Vec<PendingMigration>,
    /// 可用迁移总数
    pub available_migrations: usize,
    /// 是否为最新版本
    pub is_up_to_date: bool,
}

/// 待应用的迁移
#[derive(Debug, Serialize)]
pub struct PendingMigration {
    /// 版本号
    pub version: u32,
    /// 迁移名称
    pub name: String,
    /// 迁移描述
    pub description: String,
}

/// 创建迁移的辅助宏
#[macro_export]
macro_rules! migration {
    ($version:expr, $name:expr, $description:expr, $up_sql:expr) => {
        Migration {
            version: $version,
            name: $name.to_string(),
            up_sql: $up_sql.to_string(),
            down_sql: None,
            description: $description.to_string(),
        }
    };
    ($version:expr, $name:expr, $description:expr, $up_sql:expr, $down_sql:expr) => {
        Migration {
            version: $version,
            name: $name.to_string(),
            up_sql: $up_sql.to_string(),
            down_sql: Some($down_sql.to_string()),
            description: $description.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::pool::{DatabasePoolBuilder, DatabasePoolConfig};
    use tempfile::tempdir;

    fn create_test_migration_manager() -> MigrationManager {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = DatabasePoolConfig {
            database_path: db_path.to_string_lossy().to_string(),
            ..Default::default()
        };

        let pool = DatabasePoolBuilder::new(config).build().unwrap();
        let connection = DatabaseConnection::new(pool);

        MigrationManager::new(connection)
    }

    #[tokio::test]
    async fn test_migration_initialization() {
        let manager = create_test_migration_manager();

        // 初始化应该成功
        manager.initialize().unwrap();

        // 初始版本应该是0
        assert_eq!(manager.get_current_version().unwrap(), 0);
    }

    #[tokio::test]
    async fn test_migration_application() {
        let manager = create_test_migration_manager().add_migration(migration!(
            1,
            "create_users_table",
            "创建用户表",
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
        ));

        // 执行迁移
        manager.migrate(None).unwrap();

        // 检查版本
        assert_eq!(manager.get_current_version().unwrap(), 1);

        // 检查表是否创建
        assert!(manager.connection.table_exists("users").unwrap());
    }

    #[tokio::test]
    async fn test_migration_status() {
        let manager = create_test_migration_manager()
            .add_migration(migration!(
                1,
                "create_users_table",
                "创建用户表",
                "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
            ))
            .add_migration(migration!(
                2,
                "create_posts_table",
                "创建文章表",
                "CREATE TABLE posts (id INTEGER PRIMARY KEY, title TEXT NOT NULL)"
            ));

        // 获取初始状态
        let status = manager.get_migration_status().unwrap();
        assert_eq!(status.current_version, 0);
        assert_eq!(status.latest_version, 2);
        assert!(!status.is_up_to_date);
        assert_eq!(status.pending_migrations.len(), 2);

        // 执行部分迁移
        manager.migrate(Some(1)).unwrap();

        let status = manager.get_migration_status().unwrap();
        assert_eq!(status.current_version, 1);
        assert!(!status.is_up_to_date);
        assert_eq!(status.pending_migrations.len(), 1);
    }
}
