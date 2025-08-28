//! MiniCRM 领域模块
//!
//! 定义业务实体、值对象和领域服务。

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod entities;
pub mod services;
pub mod validators;

// 重新导出主要类型
// pub use entities::*;  // 暂时注释掉，等实现后再启用
