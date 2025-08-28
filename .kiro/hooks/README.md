# 开发Hooks配置

基于 `code-generation-guidelines.md` 指导方针创建的自动化开发hooks，用于提升代码质量和开发效率。

## 📋 Hooks列表

### 1. 代码质量检查 - 文件保存时
**文件**: `code-quality-on-save.json`
**触发**: 文件保存时
**功能**: 
- 自动代码格式化
- 基本质量检查
- 语言特定的静态分析
- 文件大小和复杂度检查

### 2. 提交前质量门禁
**文件**: `pre-commit-quality-gate.json`
**触发**: Git提交前
**功能**:
- 完整的代码质量检查
- 静态代码分析
- 测试执行和覆盖率检查
- 依赖安全检查
- 构建验证

### 3. 测试覆盖率监控
**文件**: `test-coverage-monitor.json`
**触发**: 代码变更时
**功能**:
- 智能测试执行
- 覆盖率趋势分析
- 未覆盖代码分析
- 测试建议生成

### 4. 依赖安全检查
**文件**: `dependency-security-check.json`
**触发**: 定期 + 依赖文件变更
**功能**:
- 安全漏洞扫描
- 依赖更新建议
- 许可证合规检查
- 依赖质量评估

### 5. 文档自动更新
**文件**: `documentation-auto-update.json`
**触发**: 代码变更时
**功能**:
- API文档生成
- README自动更新
- CHANGELOG生成
- 文档质量检查

### 6. 性能监控和优化
**文件**: `performance-monitor.json`
**触发**: 代码变更 + 定期
**功能**:
- 构建性能监控
- 代码复杂度分析
- 性能基准测试
- 优化建议生成

### 7. 代码重构建议
**文件**: `refactoring-advisor.json`
**触发**: 手动 + 定期
**功能**:
- 代码异味检测
- SOLID原则检查
- 设计模式建议
- 重构计划生成

## 🚀 使用方法

### 启用Hooks
1. 在Kiro IDE中打开 "Agent Hooks" 面板
2. 导入 `.kiro/hooks/` 目录下的hook配置文件
3. 根据需要启用或禁用特定的hooks

### 配置自定义
每个hook配置文件都支持自定义：
- 修改触发条件
- 调整质量阈值
- 配置通知方式
- 设置输出路径

### 手动触发
某些hooks支持手动触发：
```bash
# 手动运行代码重构建议
kiro hook run refactoring-advisor

# 手动运行性能监控
kiro hook run performance-monitor
```

## 📊 质量指标

### 代码质量度量标准
```rust
struct QualityMetrics {
    file_size: {
        rust_max_lines: 500,
        swift_max_lines: 400,
        python_max_lines: 300,
        typescript_max_lines: 400,
    },
    complexity: {
        cyclomatic_complexity: 10,
        function_length: 50,
        nesting_depth: 4,
    },
    duplication: {
        max_duplicate_lines: 6,
        similarity_threshold: 0.8,
    },
    coverage: {
        line_coverage: 80.0,
        branch_coverage: 70.0,
    }
}
```

### 性能基准
- **构建时间**: 监控编译时间变化
- **二进制大小**: 跟踪最终产物大小
- **测试执行时间**: 优化测试套件性能
- **内存使用**: 分析运行时内存占用

## 🔧 故障排除

### 常见问题
1. **Hook执行失败**: 检查工具依赖是否安装
2. **权限问题**: 确保有文件写入权限
3. **超时问题**: 调整timeout设置
4. **工具版本**: 确保使用兼容的工具版本

### 调试模式
启用详细日志输出：
```json
{
  "settings": {
    "debug_mode": true,
    "log_level": "debug"
  }
}
```

## 📈 效果监控

### 质量趋势
- 代码质量评分变化
- 测试覆盖率趋势
- 构建性能变化
- 依赖安全状况

### 报告输出
所有hooks生成的报告保存在：
- `.kiro/reports/` - 质量报告
- `.kiro/metrics/` - 性能指标
- `.kiro/suggestions/` - 改进建议
- `.kiro/plans/` - 重构计划

## 🎯 最佳实践

1. **渐进式启用**: 先启用基础hooks，逐步增加高级功能
2. **阈值调整**: 根据项目特点调整质量阈值
3. **团队协作**: 确保团队成员使用相同的hook配置
4. **持续改进**: 定期审查和优化hook配置

---

这套hooks系统完全基于 `code-generation-guidelines.md` 中的规范设计，确保代码质量和开发效率的持续提升。