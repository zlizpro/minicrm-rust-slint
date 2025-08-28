#!/bin/bash

# Kiro Hooks 快速设置脚本
echo "🚀 Kiro Agent Hooks 快速设置"
echo "================================"

# 检查必要工具
check_tool() {
    if command -v $1 &> /dev/null; then
        echo "✅ $1 已安装"
    else
        echo "⚠️  $1 未安装，某些功能可能无法使用"
    fi
}

echo "🔧 检查开发工具..."
check_tool "cargo"
check_tool "swift"
check_tool "python"
check_tool "node"
check_tool "git"

echo ""
echo "📁 当前项目类型检测:"
if [ -f "Cargo.toml" ]; then
    echo "🦀 Rust项目"
    PROJECT_TYPE="rust"
elif [ -f "Package.swift" ]; then
    echo "🍎 Swift项目"
    PROJECT_TYPE="swift"
elif [ -f "requirements.txt" ] || [ -f "pyproject.toml" ]; then
    echo "🐍 Python项目"
    PROJECT_TYPE="python"
elif [ -f "package.json" ]; then
    echo "📦 Node.js项目"
    PROJECT_TYPE="nodejs"
else
    echo "❓ 未识别的项目类型"
    PROJECT_TYPE="unknown"
fi

echo ""
echo "🎯 推荐的Hooks配置:"
echo "1. 代码质量检查 (文件保存时) - 推荐启用"
echo "2. 测试覆盖率监控 (代码变更时) - 推荐启用"
echo "3. 提交前质量门禁 (Git提交前) - 推荐启用"
echo "4. 依赖安全检查 (手动触发) - 可选"
echo "5. 性能监控 (手动触发) - 可选"

echo ""
echo "📖 手动导入步骤:"
echo "1. 在Kiro IDE中按 Cmd+Shift+P (macOS) 或 Ctrl+Shift+P (Windows/Linux)"
echo "2. 搜索 'Open Kiro Hook UI' 或 'Agent Hooks'"
echo "3. 点击 '创建新Hook' 或 'New Hook'"
echo "4. 复制以下配置到Hook编辑器中"

echo ""
echo "🔧 基础Hook配置 (复制到Kiro Hook UI):"
echo "----------------------------------------"

cat << 'EOF'
名称: 代码质量检查
描述: 文件保存时自动进行代码格式化和质量检查
触发: 文件保存
文件模式: **/*.rs,**/*.swift,**/*.py,**/*.ts,**/*.js

脚本:
#!/bin/bash
echo "🔍 正在进行代码质量检查..."

# 检查文件大小
if [ -f "$KIRO_FILE_PATH" ]; then
  lines=$(wc -l < "$KIRO_FILE_PATH")
  if [ $lines -gt 500 ]; then
    echo "⚠️ 警告: 文件超过500行 ($lines 行)，建议拆分"
  fi
fi

# 根据项目类型执行检查
if [ -f "Cargo.toml" ]; then
  echo "🦀 Rust项目检查..."
  cargo fmt --check || cargo fmt
  cargo clippy --quiet
elif [ -f "Package.swift" ]; then
  echo "🍎 Swift项目检查..."
  swift-format format --in-place "$KIRO_FILE_PATH" 2>/dev/null || true
elif [ -f "requirements.txt" ] || [ -f "pyproject.toml" ]; then
  echo "🐍 Python项目检查..."
  black --check "$KIRO_FILE_PATH" || black "$KIRO_FILE_PATH"
elif [ -f "package.json" ]; then
  echo "📦 Node.js项目检查..."
  prettier --check "$KIRO_FILE_PATH" || prettier --write "$KIRO_FILE_PATH"
fi

echo "✅ 代码质量检查完成"
EOF

echo ""
echo "----------------------------------------"
echo ""
echo "💡 提示:"
echo "- 先创建并测试基础的代码质量检查Hook"
echo "- 确认工作正常后，再添加其他Hooks"
echo "- 可以根据项目需要调整配置"
echo ""
echo "📚 更多配置请查看:"
echo "- .kiro/hooks/README.md (详细说明)"
echo "- .kiro/hooks/kiro-hooks-config.yaml (完整配置)"
echo ""
echo "🆘 如需帮助，请查看Kiro IDE的Agent Hooks文档"