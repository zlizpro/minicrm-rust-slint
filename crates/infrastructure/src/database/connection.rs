//! 数据库连接管理
//!
//! 提供数据库连接的高级封装和事务管理。

use anyhow::{Context, Result};
use rusqlite::{Transaction, TransactionBehavior};
use tracing::{debug, error};

use super::pool::{DatabaseConnection as PooledConnection, DatabasePool};

/// 数据库连接封装
#[derive(Clone, Debug)]
pub struct DatabaseConnection {
    pool: DatabasePool,
}

impl DatabaseConnection {
    /// 创建新的数据库连接管理器
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// 获取连接池中的连接
    pub fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get().context("无法从连接池获取数据库连接")
    }

    /// 执行事务
    ///
    /// # Arguments
    ///
    /// * `f` - 事务执行函数，接收事务对象并返回结果
    ///
    /// # Errors
    ///
    /// 如果事务执行失败或提交失败，将返回错误。
    pub fn with_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Transaction<'_>) -> Result<R>,
    {
        let mut conn = self.get_connection()?;

        debug!("开始数据库事务");
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .context("无法开始数据库事务")?;

        match f(&tx) {
            Ok(result) => {
                tx.commit().context("无法提交数据库事务")?;
                debug!("数据库事务提交成功");
                Ok(result)
            }
            Err(e) => {
                error!("事务执行失败，正在回滚: {}", e);
                if let Err(rollback_err) = tx.rollback() {
                    error!("事务回滚失败: {}", rollback_err);
                }
                Err(e)
            }
        }
    }

    /// 执行只读事务
    ///
    /// # Arguments
    ///
    /// * `f` - 只读事务执行函数
    ///
    /// # Errors
    ///
    /// 如果事务执行失败，将返回错误。
    pub fn with_read_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Transaction<'_>) -> Result<R>,
    {
        let mut conn = self.get_connection()?;

        debug!("开始只读数据库事务");
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Deferred)
            .context("无法开始只读数据库事务")?;

        let result = f(&tx)?;

        // 只读事务可以直接提交（实际上是回滚，因为没有修改）
        tx.commit().context("无法提交只读数据库事务")?;

        debug!("只读数据库事务完成");
        Ok(result)
    }

    /// 执行单个SQL语句
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL语句
    /// * `params` - 参数
    ///
    /// # Errors
    ///
    /// 如果SQL执行失败，将返回错误。
    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize>
    where
        P: rusqlite::Params,
    {
        let conn = self.get_connection()?;

        debug!("执行SQL: {}", sql);
        let affected_rows = conn
            .execute(sql, params)
            .with_context(|| format!("SQL执行失败: {}", sql))?;

        debug!("SQL执行成功，影响行数: {}", affected_rows);
        Ok(affected_rows)
    }

    /// 查询单行数据
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL查询语句
    /// * `params` - 参数
    /// * `f` - 行处理函数
    ///
    /// # Errors
    ///
    /// 如果查询失败或没有找到数据，将返回错误。
    pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let conn = self.get_connection()?;

        debug!("执行查询: {}", sql);
        let result = conn
            .query_row(sql, params, f)
            .with_context(|| format!("查询执行失败: {}", sql))?;

        debug!("查询执行成功");
        Ok(result)
    }

    /// 查询多行数据
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL查询语句
    /// * `params` - 参数
    /// * `f` - 行处理函数
    ///
    /// # Errors
    ///
    /// 如果查询失败，将返回错误。
    pub fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Vec<T>>
    where
        P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let conn = self.get_connection()?;

        debug!("执行批量查询: {}", sql);
        let mut stmt = conn
            .prepare(sql)
            .with_context(|| format!("SQL语句准备失败: {}", sql))?;

        let rows = stmt
            .query_map(params, f)
            .with_context(|| format!("查询执行失败: {}", sql))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.context("行数据处理失败")?);
        }

        debug!("批量查询执行成功，返回 {} 行", results.len());
        Ok(results)
    }

    /// 检查表是否存在
    ///
    /// # Arguments
    ///
    /// * `table_name` - 表名
    ///
    /// # Errors
    ///
    /// 如果查询失败，将返回错误。
    pub fn table_exists(&self, table_name: &str) -> Result<bool> {
        let count: i64 = self.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
            [table_name],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }

    /// 获取表的列信息
    ///
    /// # Arguments
    ///
    /// * `table_name` - 表名
    ///
    /// # Errors
    ///
    /// 如果查询失败，将返回错误。
    pub fn get_table_columns(&self, table_name: &str) -> Result<Vec<ColumnInfo>> {
        let sql = format!("PRAGMA table_info({})", table_name);

        self.query_map(&sql, [], |row| {
            Ok(ColumnInfo {
                name: row.get("name")?,
                data_type: row.get("type")?,
                not_null: row.get::<_, i32>("notnull")? != 0,
                default_value: row.get("dflt_value")?,
                primary_key: row.get::<_, i32>("pk")? != 0,
            })
        })
    }
}

/// 表列信息
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    /// 列名
    pub name: String,
    /// 数据类型
    pub data_type: String,
    /// 是否非空
    pub not_null: bool,
    /// 默认值
    pub default_value: Option<String>,
    /// 是否为主键
    pub primary_key: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::pool::{DatabasePoolBuilder, DatabasePoolConfig};
    use tempfile::tempdir;

    fn create_test_connection() -> DatabaseConnection {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let config = DatabasePoolConfig {
            database_path: db_path.to_string_lossy().to_string(),
            ..Default::default()
        };

        let pool = DatabasePoolBuilder::new(config).build().unwrap();
        DatabaseConnection::new(pool)
    }

    #[tokio::test]
    async fn test_execute_sql() {
        let conn = create_test_connection();

        // 创建测试表
        let affected = conn
            .execute(
                "CREATE TABLE test_table (id INTEGER PRIMARY KEY, name TEXT)",
                [],
            )
            .unwrap();

        assert_eq!(affected, 0); // CREATE TABLE不返回影响行数
    }

    #[tokio::test]
    async fn test_transaction() {
        let conn = create_test_connection();

        // 创建测试表
        conn.execute(
            "CREATE TABLE test_table (id INTEGER PRIMARY KEY, name TEXT)",
            [],
        )
        .unwrap();

        // 测试事务
        let result = conn.with_transaction(|tx| {
            tx.execute("INSERT INTO test_table (name) VALUES (?1)", ["test1"])?;
            tx.execute("INSERT INTO test_table (name) VALUES (?1)", ["test2"])?;
            Ok(())
        });

        assert!(result.is_ok());

        // 验证数据已插入
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_table", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_table_exists() {
        let conn = create_test_connection();

        // 表不存在
        assert!(!conn.table_exists("non_existent_table").unwrap());

        // 创建表
        conn.execute("CREATE TABLE test_table (id INTEGER PRIMARY KEY)", [])
            .unwrap();

        // 表存在
        assert!(conn.table_exists("test_table").unwrap());
    }
}
