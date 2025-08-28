//! 数据库管理模块
//!
//! 提供数据库初始化、连接管理和健康检查功能。
//! 集成了 SQLite 数据库和连接池管理。

use anyhow::{Context, Result};
use std::path::Path;
use tracing::{info, warn};

use crate::config::AppConfig;
use infrastructure::database::{
    health::DatabaseHealth,
    pool::{DatabasePool, DatabasePoolBuilder, DatabasePoolExt},
};

/// 数据库管理器
///
/// 负责数据库的初始化、连接池管理和健康检查
#[derive(Debug)]
pub struct DatabaseManager {
    pool: DatabasePool,
    database_path: String,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    ///
    /// # 参数
    /// * `config` - 应用配置
    ///
    /// # 返回
    /// 返回初始化完成的数据库管理器或错误
    pub fn new(config: &AppConfig) -> Result<Self> {
        info!("正在初始化数据库管理器: {}", config.database.path);

        // 确保数据库目录存在
        let db_path = Path::new(&config.database.path);
        if let Some(parent_dir) = db_path.parent() {
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir)
                    .with_context(|| format!("无法创建数据库目录: {:?}", parent_dir))?;
                info!("已创建数据库目录: {:?}", parent_dir);
            }
        }

        // 检查数据库文件是否为新建
        let is_new_database = !db_path.exists();
        if is_new_database {
            info!("检测到新数据库，将进行初始化");
        }

        // 创建连接池
        let pool = DatabasePoolBuilder::new(&config.database.path)
            .max_connections(config.database.max_connections)
            .connection_timeout(config.database.connection_timeout_secs)
            .build()
            .context("无法创建数据库连接池")?;

        let manager = Self {
            pool,
            database_path: config.database.path.clone(),
        };

        // 如果是新数据库，执行初始化
        if is_new_database {
            manager.initialize_database()?;
        }

        // 执行健康检查
        manager.pool.health_check().with_context(|| {
            format!("数据库健康检查失败: {}", config.database.path)
        })?;

        info!("数据库管理器初始化完成");
        Ok(manager)
    }

    /// 获取数据库连接池引用
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    /// 获取数据库路径
    pub fn database_path(&self) -> &str {
        &self.database_path
    }

    /// 执行数据库健康检查
    ///
    /// # 返回
    /// 返回详细的健康检查结果
    pub fn check_health(&self) -> DatabaseHealth {
        self.pool.get_health()
    }

    /// 初始化数据库结构
    ///
    /// 创建必要的表和索引
    fn initialize_database(&self) -> Result<()> {
        info!("正在初始化数据库结构");

        let conn = self
            .pool
            .get()
            .context("无法获取数据库连接进行初始化")?;

        // 启用外键约束
        conn.execute("PRAGMA foreign_keys = ON", [])
            .context("无法启用外键约束")?;

        // 创建客户表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS customers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                company TEXT,
                email TEXT,
                phone TEXT,
                address TEXT,
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
            [],
        )
        .context("无法创建客户表")?;

        // 创建任务表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                customer_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                priority TEXT NOT NULL DEFAULT 'medium',
                due_date TEXT,
                completed_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES customers (id) ON DELETE CASCADE
            )
            "#,
            [],
        )
        .context("无法创建任务表")?;

        // 创建报价表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS quotes (
                id TEXT PRIMARY KEY,
                customer_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                total_amount REAL NOT NULL,
                currency TEXT NOT NULL DEFAULT 'CNY',
                status TEXT NOT NULL DEFAULT 'draft',
                valid_until TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES customers (id) ON DELETE CASCADE
            )
            "#,
            [],
        )
        .context("无法创建报价表")?;

        // 创建索引
        self.create_indexes(&conn)?;

        info!("数据库结构初始化完成");
        Ok(())
    }

    /// 创建数据库索引
    fn create_indexes(&self, conn: &rusqlite::Connection) -> Result<()> {
        let indexes = [
            "CREATE INDEX IF NOT EXISTS idx_customers_email ON customers(email)",
            "CREATE INDEX IF NOT EXISTS idx_customers_company ON customers(company)",
            "CREATE INDEX IF NOT EXISTS idx_tasks_customer_id ON tasks(customer_id)",
            "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
            "CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)",
            "CREATE INDEX IF NOT EXISTS idx_quotes_customer_id ON quotes(customer_id)",
            "CREATE INDEX IF NOT EXISTS idx_quotes_status ON quotes(status)",
        ];

        for (i, index_sql) in indexes.iter().enumerate() {
            conn.execute(index_sql, [])
                .with_context(|| format!("无法创建索引 {}: {}", i + 1, index_sql))?;
        }

        info!("数据库索引创建完成");
        Ok(())
    }

    /// 执行数据库备份
    ///
    /// # 参数
    /// * `backup_path` - 备份文件路径
    pub fn backup_database<P: AsRef<Path>>(&self, backup_path: P) -> Result<()> {
        let backup_path = backup_path.as_ref();
        info!("正在备份数据库到: {:?}", backup_path);

        // 确保备份目录存在
        if let Some(parent_dir) = backup_path.parent() {
            std::fs::create_dir_all(parent_dir)
                .with_context(|| format!("无法创建备份目录: {:?}", parent_dir))?;
        }

        let conn = self.pool.get().context("无法获取数据库连接进行备份")?;

        // 使用 SQLite 的 VACUUM INTO 命令进行备份
        conn.execute(
            "VACUUM INTO ?",
            [backup_path.to_string_lossy().as_ref()],
        )
        .with_context(|| format!("数据库备份失败: {:?}", backup_path))?;

        info!("数据库备份完成: {:?}", backup_path);
        Ok(())
    }

    /// 获取数据库统计信息
    pub fn get_database_stats(&self) -> Result<DatabaseStats> {
        let conn = self.pool.get().context("无法获取数据库连接")?;

        let customer_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM customers", [], |row| row.get(0))
            .context("无法查询客户数量")?;

        let task_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
            .context("无法查询任务数量")?;

        let quote_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM quotes", [], |row| row.get(0))
            .context("无法查询报价数量")?;

        // 获取数据库文件大小
        let file_size = std::fs::metadata(&self.database_path)
            .map(|metadata| metadata.len())
            .unwrap_or(0);

        Ok(DatabaseStats {
            customer_count: customer_count as u64,
            task_count: task_count as u64,
            quote_count: quote_count as u64,
            file_size_bytes: file_size,
        })
    }
}

/// 数据库统计信息
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    /// 客户数量
    pub customer_count: u64,
    /// 任务数量
    pub task_count: u64,
    /// 报价数量
    pub quote_count: u64,
    /// 数据库文件大小（字节）
    pub file_size_bytes: u64,
}

impl DatabaseStats {
    /// 获取格式化的文件大小
    pub fn formatted_file_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = self.file_size_bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config() -> Result<AppConfig> {
        let temp_dir = TempDir::new().context("无法创建临时目录")?;
        let db_path = temp_dir.path().join("test.db").to_string_lossy().to_string();

        let mut config = AppConfig::default();
        config.database.path = db_path;

        // 确保临时目录不会被删除（在测试期间）
        // 注意：这会导致内存泄漏，但在测试中是可接受的
        std::mem::forget(temp_dir);

        Ok(config)
    }

    #[test]
    fn test_database_manager_creation() -> Result<()> {
        let config = create_test_config()?;
        let db_manager = DatabaseManager::new(&config)?;

        // 健康检查应该通过
        let health = db_manager.check_health();
        assert!(health.healthy);

        // 验证基本功能
        assert!(!health.checks.is_empty());

        // 验证连接池状态
        assert!(health.pool_status.healthy);
        assert!(health.pool_status.utilization_percentage < 90.0);

        Ok(())
    }

    #[test]
    fn test_database_stats() -> Result<()> {
        let config = create_test_config()?;
        let db_manager = DatabaseManager::new(&config)?;

        let stats = db_manager.get_database_stats()?;
        
        // 新数据库应该有0条记录
        assert_eq!(stats.customer_count, 0);
        assert_eq!(stats.task_count, 0);
        assert_eq!(stats.quote_count, 0);
        
        // 文件大小应该大于0
        assert!(stats.file_size_bytes > 0);
        
        // 测试格式化文件大小
        let formatted_size = stats.formatted_file_size();
        assert!(formatted_size.contains("B") || formatted_size.contains("KB"));

        Ok(())
    }

    #[test]
    fn test_database_backup() -> Result<()> {
        let config = create_test_config()?;
        let db_manager = DatabaseManager::new(&config)?;

        let temp_dir = TempDir::new()?;
        let backup_path = temp_dir.path().join("backup.db");

        db_manager.backup_database(&backup_path)?;
        assert!(backup_path.exists());

        // 验证备份文件不为空
        let backup_size = std::fs::metadata(&backup_path)?.len();
        assert!(backup_size > 0);

        Ok(())
    }

    #[test]
    fn test_database_initialization() -> Result<()> {
        let config = create_test_config()?;
        let db_manager = DatabaseManager::new(&config)?;

        // 验证表是否创建
        let conn = db_manager.pool().get()?;
        
        // 检查customers表
        let customer_table_exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='customers'",
            [],
            |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            },
        )?;
        assert!(customer_table_exists);

        // 检查tasks表
        let task_table_exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='tasks'",
            [],
            |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            },
        )?;
        assert!(task_table_exists);

        // 检查quotes表
        let quote_table_exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='quotes'",
            [],
            |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            },
        )?;
        assert!(quote_table_exists);

        Ok(())
    }
}