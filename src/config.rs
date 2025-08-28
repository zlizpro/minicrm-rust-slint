//! 应用程序配置管理
//!
//! 负责加载和管理应用程序的各种配置选项。

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用程序主配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 用户界面配置
    pub ui: UiConfig,
    /// 日志配置
    pub logging: LoggingConfig,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库文件路径
    pub path: PathBuf,
    /// 连接池最大连接数
    pub max_connections: u32,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
}

/// 用户界面配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// 窗口标题
    pub window_title: String,
    /// 默认窗口宽度
    pub window_width: u32,
    /// 默认窗口高度
    pub window_height: u32,
    /// 主题名称
    pub theme: String,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 日志文件路径
    pub file_path: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                path: PathBuf::from("data/minicrm.db"),
                max_connections: 10,
                connection_timeout: 30,
            },
            ui: UiConfig {
                window_title: "MiniCRM - 板材行业客户管理系统".to_string(),
                window_width: 1280,
                window_height: 800,
                theme: "default".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: Some(PathBuf::from("logs/minicrm.log")),
            },
        }
    }
}

impl AppConfig {
    /// 加载应用程序配置
    ///
    /// 首先尝试从配置文件加载，如果文件不存在则使用默认配置。
    ///
    /// # Errors
    ///
    /// 如果配置文件存在但格式不正确，将返回错误。
    pub fn load() -> Result<Self> {
        // TODO: 在后续任务中实现从文件加载配置
        // 目前返回默认配置
        Ok(Self::default())
    }
}
