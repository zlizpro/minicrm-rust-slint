//! 应用程序主模块
//!
//! 负责应用程序的初始化、配置加载和主要业务逻辑的协调。

use anyhow::Result;
use slint::ComponentHandle;
use tracing::{debug, error, info};

use crate::config::AppConfig;
use crate::database::DatabaseManager;

// 包含编译后的Slint UI代码
slint::include_modules!();

/// `MiniCRM` 应用程序主结构
///
/// 负责管理应用程序的生命周期，包括初始化、运行和清理。
#[derive(Debug)]
pub struct App {
    /// 应用程序配置
    config: AppConfig,
    /// 数据库管理器
    database: Option<DatabaseManager>,
}

impl App {
    /// 创建新的应用程序实例
    ///
    /// # Errors
    ///
    /// 如果配置加载失败或初始化过程中出现错误，将返回错误。
    pub fn new() -> Result<Self> {
        debug!("开始初始化应用程序");

        // 加载配置
        let config = AppConfig::load()?;
        debug!("配置加载成功: {:?}", config);

        Ok(Self {
            config,
            database: None,
        })
    }

    /// 运行应用程序主循环
    ///
    /// # Errors
    ///
    /// 如果应用程序运行过程中出现不可恢复的错误，将返回错误。
    pub fn run(mut self) -> Result<()> {
        info!("启动应用程序主循环");

        // 初始化数据库
        info!("初始化数据库...");
        let database = DatabaseManager::initialize(&self.config)?;
        self.database = Some(database);
        info!("数据库初始化完成");

        // 创建主窗口（在async上下文之外）
        Self::run_ui().map_err(|e| anyhow::anyhow!("UI运行失败: {}", e))
    }

    /// 运行UI部分（同步函数）
    fn run_ui() -> Result<()> {
        // 创建主窗口
        let main_window =
            MainWindow::new().map_err(|e| anyhow::anyhow!("创建主窗口失败: {}", e))?;

        // 设置窗口属性
        main_window.set_status_message("数据库连接正常，系统就绪".into());

        // 设置回调函数
        let window_weak = main_window.as_weak();
        main_window.on_show_about({
            let window_weak = window_weak.clone();
            move || {
                if let Some(window) = window_weak.upgrade() {
                    window.set_status_message("关于 `MiniCRM` v0.1.0 - 专为板材行业设计".into());
                }
            }
        });

        main_window.on_exit_application({
            move || {
                if let Some(window) = window_weak.upgrade() {
                    window.hide().unwrap_or_else(|e| {
                        error!("关闭窗口失败: {}", e);
                    });
                }
            }
        });

        // 显示窗口
        main_window
            .show()
            .map_err(|e| anyhow::anyhow!("显示主窗口失败: {}", e))?;

        info!("主窗口已显示，进入事件循环");

        // 运行Slint事件循环
        slint::run_event_loop().map_err(|e| anyhow::anyhow!("事件循环运行失败: {}", e))?;

        info!("应用程序事件循环结束");
        Ok(())
    }
}
