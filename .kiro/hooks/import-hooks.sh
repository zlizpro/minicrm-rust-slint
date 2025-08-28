#!/bin/bash

# Kiro Agent Hooks 自动导入脚本
# 基于 code-generation-guidelines.md 的开发hooks

echo "🚀 开始导入Kiro Agent Hooks..."

HOOKS_DIR=".kiro/hooks"
KIRO_CONFIG_DIR=".kiro/agent-hooks"

# 创建Kiro hooks配置目录
mkdir -p "$KIRO_CONFIG_DIR"

# 检查Kiro是否运行
if ! pgrep -f "Kiro" > /dev/null; then
    echo "⚠️  请先启动Kiro IDE"
    exit 1
fi

echo "📋 发现以下hooks配置文件:"
for hook_file in "$HOOKS_DIR"/*.json; do
    if [ -f "$hook_file" ]; then
        hook_name=$(basename "$hook_file" .json)
        echo "  - $hook_name"
    fi
done

echo ""
echo "🔧 开始导入hooks..."

# 复制hooks配置到Kiro配置目录
cp "$HOOKS_DIR"/*.json "$KIRO_CONFIG_DIR/" 2>/dev/null

# 创建hooks索引文件
cat > "$KIRO_CONFIG_DIR/hooks-index.json" << EOF
{
  "version": "1.0.0",
  "description": "基于code-generation-guidelines.md的开发hooks",
  "hooks": [
    {
      "id": "code-quality-on-save",
      "name": "代码质量检查 - 文件保存时",
      "file": "code-quality-on-save.json",
      "enabled": true,
      "priority": 1
    },
    {
      "id": "pre-commit-quality-gate", 
      "name": "提交前质量门禁",
      "file": "pre-commit-quality-gate.json",
      "enabled": true,
      "priority": 2
    },
    {
      "id": "test-coverage-monitor",
      "name": "测试覆盖率监控", 
      "file": "test-coverage-monitor.json",
      "enabled": true,
      "priority": 3
    },
    {
      "id": "dependency-security-check",
      "name": "依赖安全检查",
      "file": "dependency-security-check.json", 
      "enabled": true,
      "priority": 4
    },
    {
      "id": "documentation-auto-update",
      "name": "文档自动更新",
      "file": "documentation-auto-update.json",
      "enabled": false,
      "priority": 5
    },
    {
      "id": "performance-monitor",
      "name": "性能监控和优化",
      "file": "performance-monitor.json",
      "enabled": false, 
      "priority": 6
    },
    {
      "id": "refactoring-advisor",
      "name": "代码重构建议",
      "file": "refactoring-advisor.json",
      "enabled": false,
      "priority": 7
    }
  ]
}
EOF

echo "✅ Hooks配置文件已复制到 $KIRO_CONFIG_DIR"
echo ""
echo "📖 接下来的步骤:"
echo "1. 在Kiro IDE中打开命令面板 (Cmd+Shift+P)"
echo "2. 搜索并执行 'Open Kiro Hook UI'"
echo "3. 点击 'Import' 或 '导入' 按钮"
echo "4. 选择目录: $KIRO_CONFIG_DIR"
echo "5. 批量导入所有hooks配置"
echo ""
echo "🎯 推荐启用顺序:"
echo "  1️⃣ code-quality-on-save (基础质量检查)"
echo "  2️⃣ test-coverage-monitor (测试监控)" 
echo "  3️⃣ pre-commit-quality-gate (提交门禁)"
echo "  4️⃣ dependency-security-check (安全检查)"
echo "  5️⃣ 其他高级hooks (根据需要)"
echo ""
echo "🔧 如果导入失败，请尝试:"
echo "  - 重启Kiro IDE"
echo "  - 检查文件权限"
echo "  - 手动创建hooks"
echo ""
echo "📚 详细说明请查看: .kiro/hooks/README.md"