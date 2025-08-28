//! Repository实现模块
//!
//! 提供数据访问层的具体实现。

pub mod generic;

// 重新导出主要类型
pub use generic::GenericRepository;
