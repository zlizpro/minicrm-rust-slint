# 贡献指南

感谢您对MiniCRM项目的关注！我们欢迎各种形式的贡献。

## 开发环境设置

### 前置要求

- Rust 1.70+
- Git
- 支持的操作系统：macOS, Windows, Linux

### 设置步骤

1. Fork并克隆仓库：
```bash
git clone https://github.com/your-username/minicrm-rust-slint.git
cd minicrm-rust-slint
```

2. 安装依赖：
```bash
# Linux
sudo apt-get install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev

# macOS
brew install pkg-config
```

3. 构建项目：
```bash
cargo build --features gui
```

4. 运行测试：
```bash
cargo test --all-features
```

## 代码规范

### 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循Rust官方代码风格指南

### 提交前检查

运行质量检查脚本：
```bash
./scripts/pre-commit.sh
```

或者手动执行：
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
```

## 提交规范

### 提交消息格式

使用以下格式编写提交消息：

```
<类型>(<范围>): <描述>

<详细说明>

<脚注>
```

#### 类型
- `feat`: 新功能
- `fix`: 错误修复
- `docs`: 文档更新
- `style`: 代码风格调整（不影响功能）
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

#### 示例
```
feat(customer): 添加客户搜索功能

实现了基于姓名和电话号码的客户搜索功能，
支持模糊匹配和实时搜索。

Closes #123
```

## 分支策略

- `main`: 主分支，包含稳定的发布版本
- `develop`: 开发分支，包含最新的开发进度
- `feature/*`: 功能分支，用于开发新功能
- `bugfix/*`: 修复分支，用于修复错误
- `hotfix/*`: 热修复分支，用于紧急修复

## Pull Request流程

1. 从 `develop` 分支创建功能分支
2. 在功能分支上进行开发
3. 确保所有测试通过
4. 提交Pull Request到 `develop` 分支
5. 等待代码审查
6. 根据反馈进行修改
7. 合并到 `develop` 分支

## 代码审查

### 审查要点

- 代码功能是否正确
- 是否遵循项目代码规范
- 是否有足够的测试覆盖
- 是否有适当的文档
- 是否考虑了性能和安全性

### 审查流程

1. 自动化检查（CI/CD）
2. 人工代码审查
3. 功能测试
4. 性能测试（如需要）

## 问题报告

### 错误报告

使用以下模板报告错误：

```markdown
## 错误描述
简要描述遇到的问题

## 重现步骤
1. 步骤1
2. 步骤2
3. 步骤3

## 预期行为
描述您期望发生的情况

## 实际行为
描述实际发生的情况

## 环境信息
- 操作系统：
- Rust版本：
- 项目版本：

## 附加信息
其他相关信息、截图等
```

### 功能请求

使用以下模板请求新功能：

```markdown
## 功能描述
简要描述建议的功能

## 使用场景
描述这个功能的使用场景

## 详细设计
详细描述功能的实现方式

## 替代方案
描述其他可能的实现方式
```

## 文档贡献

- 更新README.md
- 添加代码注释
- 更新API文档
- 编写用户指南

## 测试

### 测试类型

- 单元测试：测试单个函数或模块
- 集成测试：测试模块间的交互
- 端到端测试：测试完整的用户流程

### 测试要求

- 新功能必须有对应的测试
- 测试覆盖率应保持在80%以上
- 测试应该快速、可靠、独立

## 发布流程

1. 更新版本号
2. 更新CHANGELOG.md
3. 创建发布标签
4. 自动构建和发布

## 联系方式

如有问题，请通过以下方式联系：

- 创建Issue
- 发送邮件到项目维护者

## 许可证

通过贡献代码，您同意您的贡献将在MIT许可证下发布。
