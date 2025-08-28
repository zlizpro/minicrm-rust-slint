//! MiniCRM 应用层
//!
//! 定义用例、命令处理器和应用服务。

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod commands;
pub mod handlers;
pub mod queries;

// 重新导出主要类型
