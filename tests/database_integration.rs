//! 数据库集成测试
//!
//! 测试数据库连接、迁移和基本操作。

use anyhow::Result;
use minicrm::config::AppConfig;
use minicrm::database::DatabaseManager;
use tempfile::tempdir;

/// 创建测试配置
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

    // 数据库初始化应该成功
    let db_manager = DatabaseManager::initialize(&config)?;

    // 健康检查应该通过
    let health = db_manager.check_health();
    assert!(health.healthy, "数据库健康检查失败: {:?}", health.error);

    // 应该能够获取连接
    let connection = db_manager.get_connection();
    let result: i32 = connection.query_row("SELECT 1", [], |row| row.get(0))?;
    assert_eq!(result, 1);
    Ok(())
}

#[tokio::test]
async fn test_database_migrations() -> Result<()> {
    let config = create_test_config()?;

    let db_manager = DatabaseManager::initialize(&config)?;

    // 检查系统配置表是否存在
    let table_exists = db_manager.get_connection().table_exists("system_config")?;
    assert!(table_exists, "系统配置表应该存在");

    // 检查初始配置是否插入
    let schema_version: String = db_manager.get_connection().query_row(
        "SELECT value FROM system_config WHERE key = 'schema_version'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(schema_version, "1");
    Ok(())
}

#[tokio::test]
async fn test_connection_pool() -> Result<()> {
    let config = create_test_config()?;

    let db_manager = DatabaseManager::initialize(&config)?;
    let pool = db_manager.get_pool();

    // 测试多个并发连接
    let mut handles = Vec::new();

    for i in 0..5 {
        let pool_clone = pool.clone();
        let handle = tokio::spawn(async move {
            let conn = pool_clone.get()?;
            let result: i32 = conn.query_row("SELECT ?1", [i], |row| row.get(0))?;
            Ok::<i32, anyhow::Error>(result)
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await??;
        assert_eq!(result, i32::try_from(i)?);
    }
    Ok(())
}
