//! `MiniCRM` 主应用程序入口点
//!
//! 这是专为板材行业设计的跨平台客户关系管理系统的主入口。
//! 系统使用Rust + Slint技术栈，提供现代化的用户界面和高性能的数据处理能力。

use anyhow::Result;
use tracing::{error, info};

use minicrm::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_env_filter("minicrm=debug,info")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("启动 MiniCRM 板材行业客户管理系统");

    // 创建并运行应用程序
    match App::new() {
        Ok(app) => {
            info!("应用程序初始化成功");

            // 运行应用程序主循环
            if let Err(e) = app.run() {
                error!("应用程序运行时错误: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("应用程序初始化失败: {}", e);
            std::process::exit(1);
        }
    }

    info!("MiniCRM 应用程序正常退出");
    Ok(())
}
