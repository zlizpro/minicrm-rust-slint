# MiniCRM - 板材行业客户关系管理系统

## 项目概述

MiniCRM是一个专为板材行业设计的跨平台客户关系管理系统，使用Rust + Slint技术栈开发，提供现代化的用户界面和高性能的数据处理能力。

## 技术栈

- **后端语言**: Rust (内存安全、高性能)
- **GUI框架**: Slint (现代化、跨平台)
- **数据库**: SQLite + rusqlite
- **构建工具**: Cargo
- **部署**: 原生可执行程序

## 项目结构

```
minicrm/
├── Cargo.toml              # 主项目配置
├── build.rs                # 构建脚本
├── src/                    # 主应用程序源码
│   ├── main.rs            # 应用程序入口
│   ├── lib.rs             # 库模块根文件
│   ├── app.rs             # 应用程序主模块
│   ├── config.rs          # 配置管理
│   └── error.rs           # 错误处理
├── ui/                     # Slint界面文件
│   ├── main_window.slint  # 主窗口
│   └── components/        # 通用组件
├── crates/                 # 子模块
│   ├── core/              # 核心模块
│   ├── infrastructure/    # 基础设施
│   ├── domain/            # 领域模块
│   ├── application/       # 应用层
│   └── presentation/      # 表示层
└── README.md              # 项目说明
```

## 构建和运行

### 前置要求

- Rust 1.70+
- Cargo

### 构建项目

```bash
# 检查项目配置
cargo check --features gui

# 构建项目
cargo build --features gui

# 运行项目
cargo run --features gui
```

### 开发模式

```bash
# 启用所有特性的开发构建
cargo run --features gui
```

## 特性

- ✅ 跨平台兼容性 (macOS, Windows)
- ✅ 现代化Slint用户界面
- ✅ 模块化架构设计
- ✅ 严格的代码质量标准
- 🚧 SQLite数据库集成
- 🚧 客户信息管理
- 🚧 任务和提醒系统
- 🚧 报价管理
- 🚧 售后服务跟踪

## 开发状态

当前正在进行项目基础架构的搭建，包括：

1. ✅ Cargo项目结构和workspace配置
2. ✅ Slint GUI框架配置
3. 🚧 代码质量工具配置
4. 🚧 Git版本控制和CI/CD配置
5. 🚧 SQLite数据库连接配置

## 许可证

MIT License