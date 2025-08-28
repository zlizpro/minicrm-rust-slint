//! MiniCRM 核心模块
//!
//! 定义了系统中使用的基础trait、类型和接口。

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod entity;
pub mod error;
pub mod repository;
pub mod service;
pub mod types;

// 重新导出核心类型
pub use entity::*;
pub use error::{CoreError, CoreResult};
pub use repository::*;
pub use service::*;
pub use types::*;
