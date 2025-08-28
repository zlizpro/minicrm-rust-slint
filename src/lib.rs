//! `MiniCRM` 核心库
//!
//! 这是`MiniCRM`系统的核心库，提供了应用程序的主要功能模块。
//! 系统采用模块化架构设计，通过trait系统和泛型实现代码复用。

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

pub mod app;
pub mod config;
pub mod database;
pub mod error;

// 重新导出核心模块
pub use minicrm_application as application;
pub use minicrm_core as core;
pub use minicrm_domain as domain;
pub use minicrm_infrastructure as infrastructure;
pub use minicrm_presentation as presentation;

// 重新导出常用类型
pub use crate::config::AppConfig;
pub use crate::error::{Error, Result};
